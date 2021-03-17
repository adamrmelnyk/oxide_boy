#!/usr/bin/env bash
cargo test;
cargo build --release;
RUST_LOG=info ./target/release/oxide_boy run src/dmg/rom/DEFAULT_ROM.bin;