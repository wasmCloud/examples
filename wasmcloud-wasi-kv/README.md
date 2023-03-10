# `wasmcloud-wasi-kv`

This repository contains a [WebAssembly System Interface (WASI) key-value contract][wasi-kv] compliant WebAssembly module which can be composed into a "fused" component to provide key value functionality.

This module is special because it implements the KV contract for [wasmCloud][wasmcloud] environments, showcasing the reusability of WASI compliant components across runtimes and platforms.

## ❓ What does that even mean?

- [WebAssembly ("Wasm")][wasm] is a compilation target for many languages that creates a platform agnostic binary that can run on any CPU architecture, operating system, or even in the browser
- [WebAssembly System Interface][wasi] is a framework that extends platform-specific APIs to the WebAssembly sandbox, allowing Wasm to do things like operate on a filesystem or make use of network sockets
- [Wasm Interface Type ("WIT")][wit] is the language (an [Interface Definition Language (IDL)][idl], like gRPC/Thrift/etc) for describing functionality that WebAssembly modules can use
- The [WASI key-value contract][wasi-kv] describes operations a cache key value store (think [Redis][redis]) would offer (ex. get, set, delete).
- WASM components in this folder implement the key-value contract, so they can be composed with other modules that *depend on* the contract, to do useful work (like an application)

TL;DR: WebAssembly components allow you to use a common interface to write an application that uses a key-value store: in a language of your choice, with the implementation of your choice, and run it on any platform.

[wasi]: https://wasi.dev/
[wasi-kv]: https://github.com/WebAssembly/wasi-keyvalue
[wasm]: https://webassembly.org
[idl]: https://en.wikipedia.org/wiki/Interface_description_language
[wit]: https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md
[redis]: https://redis.com/
[wasmcloud]: https://wasmcloud.com

## 🍱 What does this repository contain?

This repository contains two WASM modules:

| Module                  | Interface                  | Provider / Consumer | Description                                                                  |
|-------------------------|----------------------------|---------------------|------------------------------------------------------------------------------|
| `wasmcloud-wasi-kv-cc`  | [`wasi-keyvalue`][wasi-kv] | Provider            | WASM module that implements WASI KV, powered by [`cargo component`][cc]      |
| `wasmcloud-wasi-kv-wit` | [`wasi-keyvalue`][wasi-kv] | Provider            | WASM module that implements WASI KV, powered by [`wit-bindgen`][wit-bindgen] |

## 🏃 Getting started

To do it all at once run:

```console
make
```

This command will:

- Build a WASM component that implements [WASI key value][wasi-kv] ("provider")
- Build a WASM component that uses WASI key value ("consumer")
- Compose both components into a "fused" component in which the consumers imports are satisfied by the providers exports
- Run the fused demo with `wasmtime`

### Building the fused WASM module (`wit-bindgen`)

While we have produced the components that *provide*  and *consume* the [`world`](./wit/world.wit), we need to use [some tools][wtools] to [`compose`][wasm-compose] these components (the providers and consumers) together into *one* functional "fused" WASM module.

To do that (with `wit-bindgen`), run:

```console
make fused-component-wit
```

This Makefile target will:
- Build a component (`components/keyvalue.wasm`) that *provides* the [WASI key value interface][wasi-kv] ("provider")
- Build a component (`components/kv_demo.wasm`) that *uses* (demos) the key value interface ("consumer")
- Build a fused component (`components/kv_demo.fused.wasm`) with with `kv_demo.wasm` as the core, composed with `keyvalue.wasm` satisfying the key value contract.
  - Note that this is *not* a command build (which would use `wasi_snapshot_preview1.command.wasm`)

After building the component, `wasm-tools metadata show` is run, and you should see metadata that matches the following:

```
component:
    component:
        processed-by:
            wit-component: 0.7.3
        module:
            language:
                Rust
            processed-by:
                rustc: 1.69.0-nightly (f77bfb733 2023-03-01)
                clang: 15.0.6
                wit-component: 0.7.3
                wit-bindgen-rust: 0.4.0
        module wit-component:adapter:wasi_snapshot_preview1:
            language:
                Rust
            processed-by:
                rustc: 1.67.1 (d5a82bbd2 2023-02-07)
        module wit-component:shim:
            processed-by:
                wit-component: 0.7.3
        module wit-component:fixups:
            processed-by:
                wit-component: 0.7.3
    component:
        processed-by:
            wit-component: 0.7.3
        module:
            language:
                Rust
            processed-by:
                rustc: 1.69.0-nightly (f77bfb733 2023-03-01)
                clang: 15.0.6
                wit-component: 0.7.3
                wit-bindgen-rust: 0.4.0
        module wit-component:adapter:wasi_snapshot_preview1:
            language:
                Rust
            processed-by:
                rustc: 1.67.1 (d5a82bbd2 2023-02-07)
        module wit-component:shim:
            processed-by:
                wit-component: 0.7.3
        module wit-component:fixups:
            processed-by:
                wit-component: 0.7.3
        component:
        component:
```

### Running fused WASM module (`wit-bindgen`)

[wasm-tools]: https://github.com/bytecodealliance/wasm-tools
[wasm-compose]: https://crates.io/crates/wasm-compose
