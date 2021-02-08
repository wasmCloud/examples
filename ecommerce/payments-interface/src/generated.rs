extern crate rmp_serde as rmps;
use rmps::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io::Cursor};

extern crate log;
#[cfg(feature = "guest")]
extern crate wapc_guest as guest;
#[cfg(feature = "guest")]
use guest::prelude::*;

#[cfg(feature = "guest")]
pub struct Host {
    binding: String,
}

#[cfg(feature = "guest")]
impl Default for Host {
    fn default() -> Self {
        Host {
            binding: "default".to_string(),
        }
    }
}

/// Creates a named host binding for the payments capability
#[cfg(feature = "guest")]
pub fn host(binding: &str) -> Host {
    Host {
        binding: binding.to_string(),
    }
}

/// Creates the default host binding for the payments capability
#[cfg(feature = "guest")]
pub fn default() -> Host {
    Host::default()
}

#[cfg(feature = "guest")]
impl Host {
    pub fn authorize_payment(
        &self,
        request: AuthorizePaymentRequest,
    ) -> HandlerResult<AuthorizePaymentResponse> {
        host_call(
            &self.binding,
            "examples:payments",
            "AuthorizePayment",
            &serialize(request)?,
        )
        .map(|vec| {
            let resp = deserialize::<AuthorizePaymentResponse>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }

    pub fn complete_payment(
        &self,
        request: CompletePaymentRequest,
    ) -> HandlerResult<CompletePaymentResponse> {
        host_call(
            &self.binding,
            "examples:payments",
            "CompletePayment",
            &serialize(request)?,
        )
        .map(|vec| {
            let resp = deserialize::<CompletePaymentResponse>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }

    pub fn get_payment_methods(&self) -> HandlerResult<PaymentMethodList> {
        let input_args = GetPaymentMethodsArgs {};
        host_call(
            &self.binding,
            "examples:payments",
            "GetPaymentMethods",
            &serialize(input_args)?,
        )
        .map(|vec| {
            let resp = deserialize::<PaymentMethodList>(vec.as_ref()).unwrap();
            resp
        })
        .map_err(|e| e.into())
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct GetPaymentMethodsArgs {}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct AuthorizePaymentRequest {
    #[serde(rename = "amount")]
    pub amount: u32,
    #[serde(rename = "tax")]
    pub tax: u32,
    #[serde(rename = "payment_method")]
    pub payment_method: String,
    #[serde(rename = "payment_entity")]
    pub payment_entity: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct AuthorizePaymentResponse {
    #[serde(rename = "success")]
    pub success: bool,
    #[serde(rename = "auth_code")]
    pub auth_code: Option<String>,
    #[serde(rename = "fail_reason")]
    pub fail_reason: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct CompletePaymentRequest {
    #[serde(rename = "auth_code")]
    pub auth_code: String,
    #[serde(rename = "request")]
    pub request: AuthorizePaymentRequest,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct CompletePaymentResponse {
    #[serde(rename = "success")]
    pub success: bool,
    #[serde(rename = "txid")]
    pub txid: String,
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct PaymentMethodList {
    #[serde(rename = "methods")]
    pub methods: HashMap<String, String>,
}

/// The standard function for serializing codec structs into a format that can be
/// used for message exchange between actor and host. Use of any other function to
/// serialize could result in breaking incompatibilities.
pub fn serialize<T>(
    item: T,
) -> ::std::result::Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>>
where
    T: Serialize,
{
    let mut buf = Vec::new();
    item.serialize(&mut Serializer::new(&mut buf).with_struct_map())?;
    Ok(buf)
}

/// The standard function for de-serializing codec structs from a format suitable
/// for message exchange between actor and host. Use of any other function to
/// deserialize could result in breaking incompatibilities.
pub fn deserialize<'de, T: Deserialize<'de>>(
    buf: &[u8],
) -> ::std::result::Result<T, Box<dyn std::error::Error + Send + Sync>> {
    let mut de = Deserializer::new(Cursor::new(buf));
    match Deserialize::deserialize(&mut de) {
        Ok(t) => Ok(t),
        Err(e) => Err(format!("Failed to de-serialize: {}", e).into()),
    }
}
