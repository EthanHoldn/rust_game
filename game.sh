#!/bin/bash
cd ~/Desktop/Rust/Forest_fire
export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"
cargo build
cargo run