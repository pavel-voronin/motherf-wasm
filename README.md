# motherf-wasm

## Goal

To have the [Motherf](https://github.com/pavel-voronin/motherf) language written in Rust and compiled in WASM so that it is truly embeddable.

## Usage

You need to have `wasm-pack` installed.

```
npm run compile
npx http-server -o
```

## Progress

So far I have:

- Basic BF (not Motherf) interpreter written in Rust
- Interpreter compiled in WASM
- Loaded WASM into JS
- Program in BF computed in WASM and result logged to browser
- Protection from the endless loops
