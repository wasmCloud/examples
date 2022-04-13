// smithy-types.smithy
//
// Examples of data types and declarations in smithy
//
//

// Tell the code generator how to reference symbols defined in this namespace
metadata package = [{
    namespace: "org.wasmcloud.example.smithy_types",
    crate: "wasmcloud-example-smithy-types",
    py_module: "wasmcloud_example_smithy_types",
}]

namespace org.wasmcloud.example.smithy_types

use org.wasmcloud.model#wasmbus
use org.wasmcloud.model#codegenRust
use org.wasmcloud.model#n
use org.wasmcloud.model#Unit

use org.wasmcloud.common#DocumentRef

use org.wasmcloud.model#U8
use org.wasmcloud.model#U16
use org.wasmcloud.model#U32
use org.wasmcloud.model#U64

use org.wasmcloud.model#I8
use org.wasmcloud.model#I16
use org.wasmcloud.model#I32
use org.wasmcloud.model#I64

use org.wasmcloud.model#F32
use org.wasmcloud.model#F64

@wasmbus(
    contractId: "wasmcloud:example:smithy_types",
    actorReceive: true,
    providerReceive: true,
    protocol: "2" )
service SmithyTypes {
  version: "0.1",
  operations: [
    SendStuff, SendDocument, SendDocType, SendThings, SendMoreThings,
  ]
}

operation SendStuff {
    input: Stuff,
    output: Stuff,
}

operation SendDocument {
    input: Document,
    output: Document,
}

operation SendDocType {
    input: MyDocument,
    output: MyDocument,
}


/// simple type declaration
document MyDocument

structure Stuff {
    @required
    val: String,
}

operation SendThings {
    input: Things,
    output: Things,
}

operation SendMoreThings {
    input: MoreThings,
    output: MoreThings,
}

/// struct containing many types
@codegenRust(noDeriveEq: true)
structure Things {

    @required
    uint8: U8,

    @required
    uint16: U16,

    @required
    uint32: U32,

    @required
    uint64: U64,


    @required
    int8: I8,

    @required
    int16: I16,

    @required
    int32: I32,

    @required
    int64: I64,

    @required
    float32: F32,

    @required
    float64: F64,
    
    @required
    blob: Blob,

    @required
    str: String,

    @required
    doc: Document,
}

@codegenRust(noDeriveEq: true)
structure MoreThings {
    @required
    things: Things,
}


@codegenRust(noDeriveEq:true)
structure DocRef {
    @required
    docRef: DocumentRef
}

union DataType {
    @n(10)
    typeU8: Unit,
    @n(11)
    typeU16: Unit,
    @n(12)
    typeU32: Unit,
    @n(13)
    typeU64: Unit,

    @n(20)
    typeI8: Unit,
    @n(21)
    typeI16: Unit,
    @n(22)
    typeI32: Unit,
    @n(23)
    typeI64: Unit,
}

@enum([
    {
        value: "Apple",
        name: "Apple",
    },
    {
        value: "Banana",
        name: "Banana",
    },
    {
        value: "Cherry",
        name: "Cherry",
    },
    {
        value: "Peach",
        name: "Peach",
    },
])
string Fruit

