# Wasm AES128

Thin wrapper around aes-soft. Implements encrypt and decrypt functions.

Blocks to encode must have length of 16 bytes.

To build, use `wasm-pack build`.

## Usign with Deno

In the output from `wasm-pack build` there's a javascript file with a header:

```javascript
import * as wasm from "./wasm_aes_soft_bg.wasm";
```

Replace it with:

```javascript
const binary = await Deno.readFile("./wasm_aes_soft_bg.wasm");
const wasmModule = new WebAssembly.Module(binary);
const instance = await WebAssembly.instantiate(wasmModule);
const wasm = instance.exports;
```

And you are good to go!

## Credit

This is a thin wrapper around the create [aes-soft](https://docs.rs/crate/aes-soft/0.3.3). All credit goes to their creators.
