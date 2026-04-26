#! /usr/bin/env bash

# crabcc_driver.sh
#
# A compiler driver for the crabcc compiler.

set -euo pipefail

function usage() {
  echo "usage: $0 [--lex | --parse | --codegen | -S] <file.c>"  
}

step=""
source_file=""

for arg in "$@"; do
  case "$arg" in
    --lex)
      step="lex"
      ;;
    --parse)
      step="parse"
      ;;
    --codegen)
      step="codegen"
      ;;
    -S)
      step="asm"
      ;;
    *.c)
      source_file="$arg"
      ;;
    *)
      echo "error: unexpected argument '$arg'"
      exit 1
      ;;
  esac
done

if [[ -z "$source_file" ]]; then
  echo "error: expected C source file"
  usage
  exit 1
fi

if [[ ! -f "$source_file" ]]; then
  echo "error: '$source_file' not found"
  exit 1
fi

base="${source_file%.c}"
preprocessed_file="${base}.i"
assembly_file="${base}.s"
executable="${base}"

# Clean up generated files.
function cleanup() {
  rm -f "$preprocessed_file"
  if [[ "$step" != "asm" ]]; then
    rm -f "$assembly_file"
  fi
}
trap cleanup EXIT

# 1. Run preprocesser.
gcc -E -P "${source_file}" -o "$preprocessed_file"

case "$step" in
  lex)
    cargo run -- --lex $source_file
    exit 0
    ;;
  parse)
    echo "TODO: implement parser"
    exit 0
    ;;
  codegen)
    echo "TODO: implement assembly generation"
    exit 0
    ;;
esac

# 2. Compile source to assembly.
gcc -S -O0 "$preprocessed_file" -o "$assembly_file"

if [[ "$step" == "asm" ]]; then
  exit 0
fi

# 3. Assemble and link.
gcc "$assembly_file" -o "$executable"

exit 0
