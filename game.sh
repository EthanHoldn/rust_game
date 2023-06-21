#!/bin/bash
cd ~/Desktop/rust/game
export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"
cargo build
cargo run