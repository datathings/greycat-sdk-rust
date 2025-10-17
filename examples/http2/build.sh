#!/bin/sh
set -e

NAME="csv2"

# Check if debug mode is requested
if [ "$1" = "debug" ]; then
  # Build in debug mode
  cargo build
  # Copy debug library
  cp "../target/debug/lib$NAME.so" "lib/$NAME/$NAME.gclib"
else
  # Build in release mode (default)
  cargo build --release
  # Copy release library
  cp "../target/release/lib$NAME.so" "lib/$NAME/$NAME.gclib"
fi