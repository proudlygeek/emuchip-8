#!/usr/bin/env bash
cargo +nightly build --target wasm32-unknown-unknown --release --lib && \
  wasm-bindgen target/wasm32-unknown-unknown/release/emuchip_8.wasm --out-dir www/