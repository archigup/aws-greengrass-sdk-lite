// aws-greengrass-sdk-lite - Lightweight AWS IoT Greengrass SDK
// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#include "receive.h"
#include "eventstream_io.h"
#include <assert.h>
#include <ggl/attr.h>
#include <ggl/buffer.h>
#include <ggl/cleanup.h>
#include <ggl/error.h>
#include <ggl/eventstream/decode.h>
#include <ggl/eventstream/rpc.h>
#include <ggl/file.h>
#include <ggl/init.h>
#include <ggl/ipc/client_priv.h>
#include <ggl/ipc/client_raw.h>
#include <ggl/log.h>
#include <ggl/socket_epoll.h>
#include <inttypes.h>
#include <pthread.h>
#include <sys/types.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <stdnoreturn.h>

static_assert(
    GGL_IPC_MAX_STREAMS >= 2, "At least 2 streams must be supported."
);

static GglError init_ipc_recv_thread(void);
noreturn static void *recv_thread(void *args) ACCESS(none, 1);

static int epoll_fd = -1;

// TODO: These could be 1 byte per stream (4 possible values for stream handler;
// one is NULL, two are serialized so can grab ctx from global, and last can get
// its ctx using stream id)
static GglIpcStreamHandler stream_handler[GGL_IPC_MAX_STREAMS] = { 0 };
static int32_t stream_handler_id[GGL_IPC_MAX_STREAMS] = { 0 };
static void *stream_handler_ctx[GGL_IPC_MAX_STREAMS];

static pthread_mutex_t stream_state_mtx;

__attribute__((constructor)) static void init_ipc_recv_stream_mtx(void) {
    pthread_mutexattr_t mutattr = { 0 };
    pthread_mutexattr_init(&mutattr);
    pthread_mutexattr_settype(&mutattr, PTHREAD_MUTEX_RECURSIVE);
    pthread_mutex_init(&stream_state_mtx, &mutattr);
    pthread_mutexattr_destroy(&mutattr);
}

__attribute__((constructor)) static void register_init_ipc_recv_thread(void) {
    static GglInitEntry entry = { .fn = &init_ipc_recv_thread };
    ggl_register_init_fn(&entry);
}

static GglError init_ipc_recv_thread(void) {
    GglError ret = ggl_socket_epoll_create(&epoll_fd);
    if (ret != GGL_ERR_OK) {
        GGL_LOGE("Failed to create epoll for GG-IPC sockets.");
        return ret;
    }

    pthread_t recv_thread_handle;
    int sys_ret = pthread_create(&recv_thread_handle, NULL, &recv_thread, NULL);
    if (sys_ret != 0) {
        GGL_LOGE("Failed to create GG-IPC receive thread: %d.", sys_ret);
        return GGL_ERR_FATAL;
    }
    pthread_detach(recv_thread_handle);
    return GGL_ERR_OK;
}

bool ggipc_get_stream_index(int32_t stream, size_t *index) {
    for (size_t i = 0; i < GGL_IPC_MAX_STREAMS; i++) {
        if (stream_handler_id[i] == stream) {
            *index = i;
            return true;
        }
    }
    return false;
}

void ggipc_set_stream_handler_at(
    int32_t stream, GglIpcStreamHandler handler, void *ctx
) {
    GGL_MTX_SCOPE_GUARD(&stream_state_mtx);

    size_t index;
    bool found = ggipc_get_stream_index(stream, &index);
    if (!found) {
        assert(false);
        return;
    }

    stream_handler_ctx[index] = ctx;
    stream_handler[index] = handler;
}

void ggipc_clear_stream_handler_if_eq(
    int32_t stream, GglIpcStreamHandler handler, void *ctx
) {
    GGL_MTX_SCOPE_GUARD(&stream_state_mtx);

    size_t index;
    bool found = ggipc_get_stream_index(stream, &index);
    if (!found) {
        return;
    }

    if ((stream_handler_ctx[index] == ctx)
        && (stream_handler[index] == handler)) {
        stream_handler_ctx[index] = NULL;
        stream_handler[index] = NULL;
        stream_handler_id[index] = -1;
    }
}

GglError ggipc_set_stream_handler(
    GglIpcStreamHandler handler, void *ctx, int32_t *stream
) {
    GGL_MTX_SCOPE_GUARD(&stream_state_mtx);

    static int32_t stream_id = 1;

    for (int32_t i = 1; i < GGL_IPC_MAX_STREAMS; i++) {
        if (stream_handler_id[i] <= 0) {
            *stream = stream_id++;
            stream_handler_id[i] = *stream;
            stream_handler_ctx[i] = ctx;
            stream_handler[i] = handler;
            return GGL_ERR_OK;
        }
    }

    GGL_LOGE("Exceeded maximum GG-IPC eventstream streams.");
    return GGL_ERR_NOMEM;
}

GglError ggipc_register_ipc_socket(int conn) {
    assert(epoll_fd >= 0);
    return ggl_socket_epoll_add(epoll_fd, conn, (uint64_t) conn);
}

static GglError forward_incoming_packet(int conn) {
    static uint8_t payload_buf[GGL_IPC_MAX_MSG_LEN];

    EventStreamMessage msg;
    GglError ret = ggipc_get_message(conn, &msg, GGL_BUF(payload_buf));
    if (ret != GGL_ERR_OK) {
        GGL_LOGE("Failed to read eventstream packet.");
        return ret;
    }

    EventStreamCommonHeaders common_headers;
    ret = eventstream_get_common_headers(&msg, &common_headers);
    if (ret != GGL_ERR_OK) {
        GGL_LOGE("Eventstream packet missing required headers.");
        return ret;
    }

    int32_t stream_id = common_headers.stream_id;

    if (stream_id < 0) {
        GGL_LOGE("Eventstream packet has negative stream id.");
        return GGL_ERR_FAILURE;
    }

    GGL_MTX_SCOPE_GUARD(&stream_state_mtx);

    size_t index;
    bool found = ggipc_get_stream_index(stream_id, &index);

    if (!found) {
        GGL_LOGE(
            "Unhandled eventstream packed with stream id %" PRId32 " dropped.",
            stream_id
        );
        return GGL_ERR_OK;
    }

    return stream_handler[index](
        stream_handler_ctx[index], common_headers, msg
    );

    // TODO: Terminate stream if flag set
}

static GglError data_ready_callback(void *ctx, uint64_t data) ACCESS(none, 1);

static GglError data_ready_callback(void *ctx, uint64_t data) {
    (void) ctx;
    int conn = (int) data;

    GglError ret = forward_incoming_packet(conn);

    if (ret != GGL_ERR_OK) {
        GGL_LOGE(
            "Error receiving from GG-IPC connection on fd %d. Closing "
            "connection.",
            conn
        );
        (void) ggl_close(conn);
    }

    return ret;
}

noreturn static void *recv_thread(void *args) {
    (void) args;

    GGL_LOGI("Starting GG-IPC receive thread.");

    (void) ggl_socket_epoll_run(epoll_fd, &data_ready_callback, NULL);

    GGL_LOGE("GG-IPC receive thread failed. Exiting.");
    _Exit(1);
}
