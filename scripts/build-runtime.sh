#!/bin/bash
# Build QuinusLang runtime to runtime.o
# Requires: clang or gcc in PATH
# Output: dist-runtime/runtime.o

set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
RUNTIME_DIR="$ROOT_DIR/runtime"
OUT_DIR="$ROOT_DIR/dist-runtime"

mkdir -p "$OUT_DIR"
OUT_PATH="$OUT_DIR/runtime.o"

CC=""
for c in clang gcc; do
    if command -v $c &>/dev/null; then
        CC=$c
        break
    fi
done

if [ -z "$CC" ]; then
    echo "Error: No C compiler found (clang or gcc). Install LLVM or GCC."
    exit 1
fi

echo "Building runtime with $CC..."
$CC -c -O2 -o "$OUT_PATH" "$RUNTIME_DIR/runtime.c"

echo "Built: $OUT_PATH"
