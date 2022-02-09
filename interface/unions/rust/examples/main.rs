use std::collections::HashMap;
use wasmcloud_example_union_demo::*;

fn main() {
    // create a union type - as a rust enum
    let val = AnyValue::ValU8(42);

    // create a value map
    let mut map = HashMap::<String, AnyValue>::new();
    map.insert("small".to_string(), val);
    map.insert("big".to_string(), AnyValue::ValU64(1 << 60));
    map.insert("pi".to_string(), AnyValue::ValF64(3.14159));
    map.insert("data".to_string(), AnyValue::ValBin(vec![1u8, 2, 3, 4]));

    // read a value
    if let Some(AnyValue::ValF64(pi)) = map.get("pi") {
        println!("good pi: {}", pi);
    } else {
        println!("still hungry");
    }
    let num_sent_items = map.len();

    // build a success response
    let good = Response::Values(map);
    // build error response
    let bad = Response::Error(ErrorResponse {
        message: "Something went wrong".to_string(),
        ..Default::default()
    });

    // check responses
    match good {
        Response::Values(data) => {
            println!("success response!");
            assert_eq!(
                data.len(),
                num_sent_items,
                "success! map with {} items",
                data.len()
            )
        }
        Response::Error(ErrorResponse { .. }) => {
            assert!(false, "did not expect error here");
        }
    }
    match bad {
        Response::Values(_data) => {
            assert!(false, "did not expect data with error");
        }
        Response::Error(ErrorResponse { message, .. }) => {
            println!("expected error message: {} (but nothing is wrong)", message);
            assert!(true, "expected error message: {}", message);
        }
    }

    assert!(true, "done");
}
