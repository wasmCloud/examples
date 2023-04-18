#![allow(unused_imports)]
use crate::stringify;
use crate::BASE_URL;
use jammin_interfaces_apigw::RoutedRequest;
use std::collections::HashMap;
use strum::EnumString;
use url::Url;
use wasmbus_rpc::actor::prelude::Context;
use wasmcloud_interface_httpserver::HttpRequest;
use wasmcloud_interface_logging::{debug, error, info, log, warn};

// ContentType may be required if using multiple mime types, for now use json
// pub enum ContentType {
//     application/x-www-form-urlencoded
//     json
// }

#[derive(Copy, Clone, Debug, PartialEq, EnumString)]
pub enum Method {
    #[strum(ascii_case_insensitive)]
    GET,
    #[strum(ascii_case_insensitive)]
    HEAD,
    #[strum(ascii_case_insensitive)]
    POST,
    #[strum(ascii_case_insensitive)]
    PUT,
    #[strum(ascii_case_insensitive)]
    DELETE,
    #[strum(ascii_case_insensitive)]
    CONNECT,
    #[strum(ascii_case_insensitive)]
    OPTIONS,
    #[strum(ascii_case_insensitive)]
    TRACE,
    #[strum(ascii_case_insensitive)]
    PATCH,
}

impl Method {
    pub async fn create_request(
        &self,
        _ctx: &Context,
        req: &HttpRequest,
    ) -> Result<RoutedRequest, anyhow::Error> {
        // TO BE REFACTORED
        // Think on how to DRY out request creation. Most things are common, but ? is required for query string. Was doing everything outside of match initially. May create my own functions instead of using URL crate. Not worth effort atm.
        let rreq: RoutedRequest = match self {
            Method::GET => {
                let host = format!(
                    "{}{}?{}",
                    BASE_URL,
                    req.path.clone(),
                    req.query_string.clone()
                );
                info!("host: {:?}", host);
                let path = match parse_url_path(&host) {
                    Ok(p) => p,
                    Err(e) => stringify(e),
                };
                // info!("path: {:?}", path);
                let path_query = match parse_url_query(&host) {
                    Ok(pq) => pq,
                    Err(e) => HashMap::from([("error".to_string(), stringify(e))]),
                };
                // info!("path_query: {:?}", path_query);
                RoutedRequest {
                    path: path.clone(),
                    body: serde_json::to_vec(&path_query).unwrap(),
                    method: req.method.clone(),
                    timeout_ms: 100,
                }
            }
            Method::HEAD => todo!(),
            Method::POST => {
                let host = format!("{}{}", BASE_URL, req.path.clone(),);
                // info!("host: {:?}", host);
                let path = match parse_url_path(&host) {
                    Ok(p) => p,
                    Err(e) => stringify(e),
                };
                // info!("path: {:?}", path);
                // must be json data if using POST for now
                let body = match parse_body(&req.body) {
                    Ok(pq) => pq,
                    Err(e) => HashMap::from([("error".to_string(), stringify(e))]),
                };
                // info!("body: {:?}", body);
                RoutedRequest {
                    path: path.clone(),
                    body: serde_json::to_vec(&body).unwrap(),
                    method: req.method.clone(),
                    timeout_ms: 100,
                }
            }
            Method::PUT => todo!(),
            Method::DELETE => todo!(),
            Method::CONNECT => todo!(),
            Method::OPTIONS => todo!(),
            Method::TRACE => todo!(),
            Method::PATCH => todo!(),
        };
        Ok(rreq)
    }
}

fn parse_url_path(url: &String) -> Result<String, String> {
    let parsed_path = Url::parse(url);
    let path = match parsed_path {
        Ok(path) => path.path().to_string().replace("/", ""),
        Err(error) => stringify(error),
    };
    Ok(path)
}

fn parse_url_query(url: &String) -> Result<HashMap<String, String>, String> {
    let parsed_url = Url::parse(url);

    let query_pairs: HashMap<String, String> = match parsed_url {
        Ok(pairs) => pairs.query_pairs().into_owned().collect(),
        Err(e) => HashMap::from([("parse_url_query".to_string(), stringify(e))]),
    };

    Ok(query_pairs)
}

fn parse_body(body: &Vec<u8>) -> Result<HashMap<String, String>, String> {
    let json = match serde_json::from_slice::<HashMap<String, String>>(body) {
        Ok(j) => j,
        Err(e) => HashMap::from([("parse_body".to_string(), stringify(e))]),
    };

    Ok(json)
}

// fn parse_url_path_segments(url: &String) -> Result<Vec<String>, String> {
//     // Example Usage:
//     // let path_segments = match parse_url_path_segments(&host).await {
//     //     Ok(ps) => ps,
//     //     Err(e) => vec![stringify(e)],
//     // };
//     // info!("path_segments: {:?}", path_segments);
//     let parsed_url = Url::parse(url);
//     let path_segments: Vec<String> = match parsed_url {
//         Ok(path) => path
//             .path_segments()
//             .unwrap()
//             .map(|s| s.to_string())
//             .collect(),
//         Err(e) => vec![stringify(e)],
//     };

//     Ok(path_segments)
// }


// CONVERT BELOW TO INTEGRATION TESTS AS SEEN IN BLOBBY EXAMPLE

// UNIT TESTS - CURRENTLY NEED TO COMMENT ANYTHING THAT REQUIRES WASM32 TARGET.
// NAMELY to_actor() calls and wasm_interface_logging calls
// #[cfg(test)]
// mod tests {
//     use crate::methods::parse_http_req::parse_url_path;
//     use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse};

//     #[test]
//     fn parse_path_single_segment() {
//         let req = HttpRequest {
//             path: "/api".to_string(),
//             ..Default::default()
//         };
//         assert_eq!("api".to_string(), parse_url_path(&req.path).unwrap());
//     }

//     #[test]
//     fn parse_path_multiple_segments() {
//         let req = HttpRequest {
//             path: "/api/path".to_string(),
//             ..Default::default()
//         };
//         assert_eq!("apipath".to_string(), parse_url_path(&req.path).unwrap());
//     }
// }
