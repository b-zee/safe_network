#!/bin/sh

cargo build --release --package=autonomi-uniffi

cargo run --features=uniffi/cli --bin=uniffi-bindgen-swift -- \
    target/release/libautonomi_uniffi.so ./out/ \
    --modulemap --modulemap-filename module.modulemap \
    --headers \
    --swift-sources
    # --xcframework

cp ./out/autonomi_uniffi.swift /tmp/MyCLI/Sources/autonomi_uniffi.swift
cp ./out/module.modulemap /tmp/MyCLI/Sources/module.modulemap
cp ./out/autonomi_uniffiFFI.h /tmp/MyCLI/Sources/autonomi_uniffiFFI/autonomi_uniffiFFI.h
cp ./target/release/libautonomi_uniffi.so /tmp/MyCLI/libautonomi_uniffi.so
