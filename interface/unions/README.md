
# Union demo (demonstration release)

This project demonstrates declaration of Unions in a smithy interface file,
and using the code generated from them. 

| ____ Demo release ____                                                                                                                                                                                                                                                                                                                                     |
|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| This is a demonstration release. Code generation for unions may incur breaking changes over the next 1.5 months (through end of March 2022) while we gather feedback from internal and extenral developers. Please try it out and let us know on wasmcloud Slack (or as an issue in this repo) if you want to use unions and if you find this useful. |


Running:
- To build, `cargo build`
- To run the example (`examples/main.rs`): `cargo run --example main`
- To run the tests: `cargo test -- --nocapture`

## Defining unions in smithy files

To define a union, use the `union` smithy type. Each variant requires a field name,
a data type, and a `@n(_)` trait for the field number. Variant data types may be any
smithy data type, including simple types, structures, arrays, and maps.

```smithy
union Number {
   @n(0)
   intVal: U64
   
   @n(1)
   floatVal: F64
}
```

Add `protocol="2"` to the `@wasmbus` trait for any service whose method parameters contain unions (directly or indirectly). You'll get an error if a union is used by a sevice that doesn't declare `protocol="2"`.
```smithy
@wasmbus(
    contractId: "wasmcloud:example:union_demo",
    actorReceive: true,
    providerReceive: true,
    protocol: "2" )
service UnionDemo {
    version: "0.1",
    operations: [ Get ]
}
```

Also make sure you have the latest rpc and code generation dependencies: in `Cargo.toml`:
```toml
[dependencies]
wasmbus-rpc = "0.8.2"

[dev-dependencies]
weld-codegen = "0.4.2"

```

**Important**: Setting protocol to "2" changes message serialization from msgpack to CBOR. 
CBOR is somewhat more flexible, and has slightly better performance, based on initial benchmarks.
All code that uses an interface (e.g., actors and capability providers)
must be compiled against the same service protocol version to be able to communicate.
In other words, changing protocol version breaks binary compatibility, 
and should only be done for new interfaces, 
or for interfaces where you can ensure that all interface users
have been recompiled with the same interface version before they are deployed to a wasmcloud lattice.

## Rust codegen

The code generator turns smithy unions into rust enums.
The union declared above will be generated as (roughly):

```rust
pub enum Number {
    IntVal(u64),
    FloatVal(f64),
}
```
