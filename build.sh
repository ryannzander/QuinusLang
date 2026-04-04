#!/bin/bash
# Q++ build script
# First build: requires Rust (cargo build --release)
# Subsequent builds: Q compiler + C compiler only
#
# Usage: ./build.sh

set -e

# Find qpp executable (Rust-built or pre-built)
qpp=""
if [ -f "target/release/qpp" ]; then
    qpp="target/release/qpp"
elif [ -f "qpp" ]; then
    qpp="./qpp"
elif [ -f "target/debug/qpp" ]; then
    qpp="target/debug/qpp"
fi

# Seed build: use Rust if no qpp
if [ -z "$qpp" ]; then
    echo "No qpp found. Building with Rust (first-time bootstrap)..."
    cargo build --release
    qpp="target/release/qpp"
    if [ ! -f "$qpp" ]; then
        echo "Rust build failed"
        exit 1
    fi
fi

echo "Building compiler with $qpp..."
$qpp build compiler/main.q

# Output is in compiler/build/ (the Q-built compiler)
if [ -f "compiler/build/output" ]; then
    cp compiler/build/output qpp
    chmod +x qpp
    echo "qpp ready (from Q compiler)"
elif [ -f "compiler/build/output.exe" ]; then
    cp compiler/build/output.exe qpp.exe
    chmod +x qpp.exe
    echo "qpp.exe ready (from Q compiler)"
else
    echo "Build produced compiler/build/output or output.exe"
fi
