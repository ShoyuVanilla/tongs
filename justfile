#!/usr/bin/env just --justfile

## Uncomment this on Windows
# set shell := ["powershell.exe", "-c"]

default_build_profile := 'dev'

build-client profile=default_build_profile:
    wasm-pack build crates/tongs-client --{{profile}} --target web --out-name tongs --out-dir ../../static/wasm

run: build-client
    cargo run -p tongs-server
