#!/usr/bin/env bash
cargo test;
RUST_LOG=info cargo run run debug;