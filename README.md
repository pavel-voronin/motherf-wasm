# motherf-wasm

## Goal

To have the [Motherf](https://github.com/pavel-voronin/motherf) language written in Rust and compiled in WASM so that it is truly embeddable.

## Progress

So far I have:

- Basic BF (not Motherf) interpreter written in Rust
- Interpreter compiled in WASM
- Loaded WASM into JS
- "Hello, World!" called from JS, computed in WASM and result logged to browser
