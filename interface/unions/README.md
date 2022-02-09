
# Union demo

This project demonstrates declaration of Unions in a smithy interface file,
and using the code generated from them.

Running:
- To build, `cargo build`
- To run the program examples/main.rs: `cargo run --example main`
- To run the tests: `cargo test -- --nocapture`

## Defining unions

To define a union, use the `union` smithy type. Each variant requires a field name,
a data type, and a `@n(_)` trait for the field number. Data types of variants may be any
smithy data type, including primitive types, structures, arrays, and maps.

```smithy
union Number {
   @n(0)
   intVal: U64
   
   @n(1)
   floatVal: F64
}
```

Set `@wasmbus(protocol="2")` for the service
```smithy
@wasmbus(
    contractId: "wasmcloud:example:union_demo",
    actorReceive: true,
    protocol: "2" )
service UnionDemo {
    version: "0.1",
    operations: [ Get ]
}
```

Also check that you have the latest rpc and code generation dependencies: in `Cargo.toml`:
```toml
[dependencies]
wasmbus-rpc = "0.7.3"

[dev-dependencies]
weld-codegen = "0.3.2"

```

Important: Setting protocol to "2" changes message serialization from msgpack to CBOR. 
CBOR is somewhat more flexible, and has slightly higher performance, based on initial benchmarks.
All users of an interface (e.g., actors and capability providers)
must be compiled against the same service protocol version to be able to communicate.
In other words, changing protocol breaks binary compatibility, 
and should only be done for newly developed interfaces, 
or for interfaces where you can ensure that all interface users
have been recompiled before they are deployed to a wasmcloud lattice.

## Rust codegen

The Rust code generator turns unions into rust enums.
The union declared above will be generated as (roughly):

```rust
pub enum Number {
    IntVal(u64),
    FloatVal(f64),
}
```

