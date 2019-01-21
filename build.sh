#!/usr/bin/env bash
cargo +nightly build --target wasm32-unknown-unknown --lib && \
  wasm-bindgen target/wasm32-unknown-unknown/debug/emuchip_8.wasm --out-dir www/