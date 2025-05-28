#!/bin/sh
set -e

# Check if debug mode is requested
if [ "$1" = "debug" ]; then
  # Build in debug mode
  cargo build
  # Copy debug library
  cp ../target/debug/libhello.so lib/hello/hello.gclib
else
  # Build in release mode (default)
  cargo build --release
  # Copy release library
  cp ../target/release/libhello.so lib/hello/hello.gclib
fi