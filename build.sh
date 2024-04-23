#!/bin/bash
set -ex

VERSION=${VERSION:-"0.0.0"}

cargo test
cargo build --release