#!/bin/sh

cargo build --release -p eguikit-demo --lib --target wasm32-unknown-unknown

wasm-bindgen "target/wasm32-unknown-unknown/release/eguikit_demo.wasm" \
  --out-dir target/website --no-modules --no-typescript

cp demo/index.html target/website/
