#!/usr/bin/env bash

set -e

REPO_DIR=$(git rev-parse --show-toplevel)

# shellcheck source=./fuzz-util.sh
source "$REPO_DIR/fuzz/fuzz-util.sh"

# 1. Generate fuzz/Cargo.toml
cat > "$REPO_DIR/fuzz/Cargo.toml" <<EOF
[package]
name = "simplicity-fuzz"
edition = "2018"
version = "0.0.1"
authors = ["Generated by fuzz/generate-files.sh"]
publish = false

[package.metadata]
cargo-fuzz = true

[dependencies]
honggfuzz = { version = "0.5.55", default-features = false }
simplicity = { path = ".." }
EOF

for targetFile in $(listTargetFiles); do
    targetName=$(targetFileToName "$targetFile")
    cat >> "$REPO_DIR/fuzz/Cargo.toml" <<EOF

[[bin]]
name = "$targetName"
path = "$targetFile"
EOF
done

# 2. Generate .github/workflows/fuzz.yml
cat > "$REPO_DIR/.github/workflows/fuzz.yml" <<EOF
# Automatically generated by fuzz/generate-files.sh
name: Fuzz

on:
  push:
    branches:
      - master
      - 'test-ci/**'
  pull_request:

jobs:
  fuzz:
    if: \${{ !github.event.act }}
    runs-on: ubuntu-20.04
    strategy:
      fail-fast: false
      matrix:
        fuzz_target: [
$(for name in $(listTargetNames); do echo "$name,"; done)
        ]
    steps:
      - name: Install test dependencies
        run: sudo apt-get update -y && sudo apt-get install -y binutils-dev libunwind8-dev libcurl4-openssl-dev libelf-dev libdw-dev cmake gcc libiberty-dev
      - uses: actions/checkout@v2
      - uses: actions/cache@v2
        id: cache-fuzz
        with:
          path: |
            ~/.cargo/bin
            fuzz/target
            target
          key: cache-\${{ matrix.target }}-\${{ hashFiles('**/Cargo.toml','**/Cargo.lock') }}
      - uses: dtolnay/rust-toolchain@master
        with:
          toolchain: 1.65.0
      - name: fuzz
        run: |
          echo "Using RUSTFLAGS \$RUSTFLAGS"
          cd fuzz && ./fuzz.sh "\${{ matrix.fuzz_target }}"
      - run: echo "\${{ matrix.fuzz_target }}" >executed_\${{ matrix.fuzz_target }}
      - uses: actions/upload-artifact@v2
        with:
          name: executed_\${{ matrix.fuzz_target }}
          path: executed_\${{ matrix.fuzz_target }}

  verify-execution:
    if: \${{ !github.event.act }}
    needs: fuzz
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions/download-artifact@v2
      - name: Display structure of downloaded files
        run: ls -R
      - run: find executed_* -type f -exec cat {} + | sort > executed
      - run: source ./fuzz/fuzz-util.sh && listTargetNames | sort | diff - executed
EOF
