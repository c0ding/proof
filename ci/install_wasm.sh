#!/bin/bash

# This script installs the wasm32 target for the desired toolchain

# Fail fast if any commands exists with error
set -e

# Print all executed commands
set -x

# Install the desired toolchain
rustup toolchain install $(cat rust-toolchain)

# Install WASM32 for the desired toolchain
rustup target add wasm32-unknown-unknown --toolchain $(cat rust-toolchain)

# Show the installed versions
rustup show