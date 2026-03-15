#!/bin/bash
# QuinusLang build script
# First build: requires Rust (cargo build --release)
# Subsequent builds: Q compiler + C compiler only
#
# Usage: ./build.sh

set -e

# Find quinus executable (Rust-built or pre-built)
quinus=""
if [ -f "target/release/quinus" ]; then
    quinus="target/release/quinus"
elif [ -f "quinus" ]; then
    quinus="./quinus"
elif [ -f "target/debug/quinus" ]; then
    quinus="target/debug/quinus"
fi

# Seed build: use Rust if no quinus
if [ -z "$quinus" ]; then
    echo "No quinus found. Building with Rust (first-time bootstrap)..."
    cargo build --release
    quinus="target/release/quinus"
    if [ ! -f "$quinus" ]; then
        echo "Rust build failed"
        exit 1
    fi
fi

echo "Building compiler with $quinus..."
$quinus build compiler/main.q

# Output is in compiler/build/ (the Q-built compiler)
if [ -f "compiler/build/output" ]; then
    cp compiler/build/output quinus
    chmod +x quinus
    echo "quinus ready (from Q compiler)"
elif [ -f "compiler/build/output.exe" ]; then
    cp compiler/build/output.exe quinus.exe
    chmod +x quinus.exe
    echo "quinus.exe ready (from Q compiler)"
else
    echo "Build produced compiler/build/output or output.exe"
fi
