use http::Method;
use wapc_guest::HandlerResult;
use wasmcloud_actor_core as actor;
use wasmcloud_actor_http_server as http;
#[allow(unused)]
use wasmcloud_actor_keyvalue as kv;

#[actor::init]
fn init() {
    http::Handlers::register_handle_request(req_handler);
}

fn req_handler(req: http::Request) -> HandlerResult<http::Response> {
    let method = req.method();
    let segments = req.path_segments();

    match (method, &*segments) {
        (Method::Get, &["hello"]) => Ok(http::Response::json("world", 200, "OK")),
        // Step 1: Uncomment the 3 lines below and the `fib` function
        // (Method::Get, &["fib", num]) => {
        //     Ok(http::Response::json(fib(num.parse::<u32>()?), 200, "OK"))
        // }

        // Step 2: Uncomment the cachefib handler below and the `cachefib` function
        // (Method::Get, &["cachefib", num]) => Ok(http::Response::json(
        //     cache_fib(num.parse::<u32>()?),
        //     200,
        //     "OK",
        // )),
        _ => Ok(http::Response::bad_request()),
    }
}

// Step 1: Uncomment this function
// fn fib(num: u32) -> u32 {
//     match num {
//         0 | 1 => 1,
//         _ => fib(num - 1) + fib(num - 2),
//     }
// }

// Step 2: Uncomment this function
// fn cache_fib(num: u32) -> u32 {
//     let kv = kv::default();
//     if let Ok(val) = kv.get(num.to_string()) {
//         if val.exists {
//             return val.value.parse::<u32>().unwrap_or_else(|_| 0);
//         }
//     }
//     match num {
//         0 | 1 => 1,
//         _ => {
//             let res = cache_fib(num - 1) + cache_fib(num - 2);
//             let _ = kv.set(num.to_string(), res.to_string(), 0);
//             res
//         }
//     }
// }
