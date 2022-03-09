//! Tests for union-demo

use wasmcloud_example_union_demo::*;

/// Test helper function to Encode and decode 'AnyValue'
/// It's not necessary to include this kind of test in your library
/// - it is used here to test code generation for union serialization
macro_rules! encode_decode_test {
    ( $enc_fn:ident, $dec_fn:ident, $val:expr, $name:expr) => {
        let mut buf = Vec::new();
        let mut encoder = wasmbus_rpc::cbor::Encoder::new(&mut buf);
        $enc_fn(&mut encoder, $val).expect(&format!("encoding {}", $name));
        let mut decoder = wasmbus_rpc::cbor::Decoder::new(&buf);
        let val2 = $dec_fn(&mut decoder).expect(&format!("decoding {}", $name));
        assert_eq!($val, &val2, "ser/deser error for {}", $name);
    };
}

/// test creating, encoding, and decoding the 'AnyValue' union
#[test]
fn test_value() {
    let x = AnyValue::ValU8(42);
    encode_decode_test!(encode_any_value, decode_any_value, &x, "u8");

    let x = AnyValue::ValU16(1000);
    encode_decode_test!(encode_any_value, decode_any_value, &x, "u16");

    let x = AnyValue::ValU32(10000000);
    encode_decode_test!(encode_any_value, decode_any_value, &x, "u32");

    let x = AnyValue::ValU64(1 << 60);
    encode_decode_test!(encode_any_value, decode_any_value, &x, "u64");

    let x = AnyValue::ValF64(3.14);
    encode_decode_test!(encode_any_value, decode_any_value, &x, "f64");

    let x = AnyValue::ValStr("hello world".to_string());
    encode_decode_test!(encode_any_value, decode_any_value, &x, "str");

    let bytes = vec![0u8, 1, 2, 3, 4, 5, 99, 100];
    let x = AnyValue::ValBin(bytes);
    encode_decode_test!(encode_any_value, decode_any_value, &x, "bytes");
}
