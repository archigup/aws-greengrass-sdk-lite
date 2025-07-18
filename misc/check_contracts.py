#!/usr/bin/env python

import subprocess
import re
import sys
import os
import json
import argparse


def run(*args):
    subprocess.run(args, check=True)


def run_get_lines(*args):
    result = subprocess.run(args, capture_output=True, text=True, check=True)
    return frozenset(result.stdout.splitlines())


parser = argparse.ArgumentParser(description='Check CBMC contracts')
parser.add_argument(
    '-p',
    '--build-dir',
    default='build',
    help='Directory containing compile_commands.json (default: build)')
parser.add_argument('files', nargs='*', help='Files to check')
args = parser.parse_args()

files = args.files
build_dir = args.build_dir

with open(os.path.join(build_dir, 'compile_commands.json'), 'r') as f:
    compdb = json.load(f)

cwd = os.getcwd()

resolved_files = []
# Find files under dir args and make files relative to cwd
for file in files:
    file = os.path.abspath(file)
    if not file.startswith(cwd):
        raise ValueError(
            f"File path {file} is not under current working directory {cwd}")

    if os.path.isdir(file):
        # Find all files in compdb that are in this directory
        for entry in compdb:
            entry_file = os.path.join(entry['directory'], entry['file'])
            entry_file = os.path.abspath(entry_file)

            # Check if this file is under the specified directory
            if entry_file.startswith(file):
                rel_path = os.path.relpath(entry_file, cwd)
                resolved_files.append(rel_path)
    else:
        rel_path = os.path.relpath(file, os.getcwd())
        resolved_files.append(rel_path)

# Remove duplicates while preserving order
resolved_files = list(dict.fromkeys(resolved_files))

for file in resolved_files:
    print(f"\033[1;34mChecking contracts for '{file}'\033[0m")

    # Find the matching entry in the compilation database
    file_abs = os.path.abspath(file)
    matching_entry = None
    for entry in compdb:
        entry_file = os.path.join(entry['directory'], entry['file'])
        entry_file = os.path.abspath(entry_file)
        if entry_file == file_abs:
            matching_entry = entry
            break

    if not matching_entry:
        raise ValueError(f"File {file} not found in compile_commands.json")

    # Extract the command from the matching entry
    if 'arguments' in matching_entry:
        command = matching_entry['arguments']
    else:
        # Parse the command string into arguments
        import shlex
        command = shlex.split(matching_entry['command'])

    output_file = f"{build_dir}/cbmc/{file}.goto"

    os.makedirs(os.path.dirname(output_file), exist_ok=True)

    if os.path.exists(output_file):
        os.remove(output_file)

    command[0] = 'goto-cc'
    # Remove this once the ggl/attr.h hack isn't needed anymore
    command.extend(['-w'])
    command.extend(['-D__CPROVER__=1', '-o', output_file,
                    '--export-file-local-symbols'])

    subprocess.run(command, check=True)

    list_fns_output = run_get_lines('goto-instrument', '--list-goto-functions',
                                    output_file)

    undef_fns_output = run_get_lines('goto-instrument',
                                     '--list-undefined-functions', output_file)
    undef_fns = frozenset(f for f in undef_fns_output
                          if re.match(r'^[a-zA-Z_][a-zA-Z0-9_]*$', f))
    contracts = frozenset(
        f.removeprefix('contract::') for f in undef_fns_output
        if re.match(r'^contract::[a-zA-Z_][a-zA-Z0-9_]*$', f))

    enforce_contracts = frozenset(c for c in contracts
                                  if f"{c} /* {c} */" in list_fns_output)
    use_contracts = contracts & undef_fns

    if len(enforce_contracts) > 0:
        run('goto-instrument', output_file, output_file,
            '--unsigned-overflow-check', '--conversion-check',
            '--enum-range-check', '--nondet-static', '--add-library')

    for fn in enforce_contracts:
        print(f"\033[1;36mChecking function '{fn}'\033[0m")

        fn_output_file = f"{output_file}::{fn}.goto"
        if os.path.exists(fn_output_file):
            os.remove(fn_output_file)

        run('goto-harness', output_file, fn_output_file,
            '--harness-function-name', 'main', '--harness-type',
            'call-function', '--function', fn)

        run('goto-cc', '-o', fn_output_file, fn_output_file)

        instrument_command = [
            'goto-instrument', fn_output_file, fn_output_file, '--dfcc',
            'main', '--apply-loop-contracts', '--enforce-contract', fn
        ]

        for c in use_contracts | (enforce_contracts - {fn}):
            instrument_command.extend(['--replace-call-with-contract', c])

        run(*instrument_command)

        run('cbmc', fn_output_file)
