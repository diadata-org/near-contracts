#!/bin/bash
set -e

RUSTFLAGS='-C link-arg=-s' cargo +stable build --all --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/dia_contract.wasm ./res/
cp target/wasm32-unknown-unknown/release/client_test_contract.wasm ./res/
