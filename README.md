# Wasm AES128

Thin wrapper around [aes-soft](https://docs.rs/crate/aes-soft/0.3.3). Implements encrypt and decrypt functions.

Blocks to encode must have length of 16 bytes.

## Build

To build, use `wasm-pack build`.

## Using with Deno

After running `wasm-pack build`, a `pkg/` directory is created on your project root.

Inside this directory there's a javascript file `wasm_aes_soft_bg.js` which starts with:

```javascript
import * as wasm from "./wasm_aes_soft_bg.wasm";
```

Replace that line with these lines:

```javascript
const binary = await Deno.readFile("./wasm_aes_soft_bg.wasm");
const wasmModule = new WebAssembly.Module(binary);
const instance = await WebAssembly.instantiate(wasmModule);
const wasm = instance.exports;
```

Copy that file together with the `wasm_aes_soft_bg.wasm` binary to the project where you want to use AES128, and you are good to go!

## Credit

This is a thin wrapper around the create [aes-soft](https://docs.rs/crate/aes-soft/0.3.3). All credit goes to their creators.
