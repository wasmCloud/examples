// This file is generated automatically using wasmcloud/weld-codegen and smithy model definitions
//

#![allow(unused_imports, clippy::ptr_arg, clippy::needless_lifetimes)]
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, io::Write, string::ToString};
use wasmbus_rpc::{
    deserialize, serialize, Context, Message, MessageDispatch, RpcError, RpcResult, SendOpts,
    Timestamp, Transport,
};

pub const SMITHY_VERSION: &str = "1.0";

/// Parameters sent for AuthorizePayment
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct AuthorizePaymentRequest {
    /// Amount of transaction, in cents.
    pub amount: u32,
    /// The entity (customer) requesting this payment
    #[serde(rename = "paymentEntity")]
    #[serde(default)]
    pub payment_entity: String,
    /// Token of the payment method to be used
    #[serde(rename = "paymentMethod")]
    #[serde(default)]
    pub payment_method: String,
    /// Opaque Reference ID (e.g. order number)
    #[serde(rename = "referenceId")]
    #[serde(default)]
    pub reference_id: String,
    /// Amount of tax applied to this transaction, in cents
    pub tax: u32,
}

/// Response to AuthorizePayment
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct AuthorizePaymentResponse {
    /// Optional string containing the tx ID of auth
    #[serde(rename = "authCode")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_code: Option<String>,
    /// Optional string w/rejection reason
    #[serde(rename = "failReason")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fail_reason: Option<String>,
    /// Indicates a successful authorization
    #[serde(default)]
    pub success: bool,
}

/// Confirm the payment (e.g., cause the transaction amount
/// to be withdrawn from the payer's account)
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CompletePaymentRequest {
    /// authorization code from the AuthorizePaymentResponse
    #[serde(rename = "authCode")]
    #[serde(default)]
    pub auth_code: String,
    /// An optional description field to be added to the payment summary
    /// (e.g., memo field of a credit card statement) |
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CompletePaymentResponse {
    /// True if the payment was successful
    #[serde(default)]
    pub success: bool,
    /// Timestamp (milliseconds since epoch, UTC)
    pub timestamp: u64,
    /// Transaction id issued by Payment provider
    #[serde(default)]
    pub txid: String,
}

/// A PaymentMethod contains a token string and a description
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct PaymentMethod {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

/// An ordered list of payment methods.
pub type PaymentMethods = Vec<PaymentMethod>;

/// wasmbus.contractId: wasmcloud:example:payments
/// wasmbus.providerReceive
#[async_trait]
pub trait Payments {
    /// returns the capability contract id for this interface
    fn contract_id() -> &'static str {
        "wasmcloud:example:payments"
    }
    /// AuthorizePayment - Validates that a potential payment transaction
    /// can go through. If this succeeds then we should assume it is safe
    /// to complete a payment. Payments _cannot_ be completed without getting
    /// a validation code (in other words, all payments have to be pre-authorized).
    async fn authorize_payment(
        &self,
        ctx: &Context,
        arg: &AuthorizePaymentRequest,
    ) -> RpcResult<AuthorizePaymentResponse>;
    /// Completes a previously authorized payment.
    /// This operation requires the "authorization code" from a successful
    /// authorization operation.
    async fn complete_payment(
        &self,
        ctx: &Context,
        arg: &CompletePaymentRequest,
    ) -> RpcResult<CompletePaymentResponse>;
    /// `GetPaymentMethods` - Retrieves an _opaque_ list of payment methods,
    /// which is a list of customer-facing method names and the
    /// _[tokens](https://en.wikipedia.org/wiki/Tokenization_(data_security))_
    /// belonging to that payment method. You could think of this list as
    /// a previously saved list of payment methods stored in a "wallet".
    /// A payment method _token_ is required to authorize and subsequently
    /// complete a payment transaction. A customer could have previously
    /// supplied their credit card and user-friendly labels for those methods
    /// like "personal" and "work", etc.
    async fn get_payment_methods(&self, ctx: &Context) -> RpcResult<PaymentMethods>;
}

/// PaymentsReceiver receives messages defined in the Payments service trait
#[doc(hidden)]
#[async_trait]
pub trait PaymentsReceiver: MessageDispatch + Payments {
    async fn dispatch(&self, ctx: &Context, message: &Message<'_>) -> RpcResult<Message<'_>> {
        match message.method {
            "AuthorizePayment" => {
                let value: AuthorizePaymentRequest = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = Payments::authorize_payment(self, ctx, &value).await?;
                let buf = serialize(&resp)?;
                Ok(Message {
                    method: "Payments.AuthorizePayment",
                    arg: Cow::Owned(buf),
                })
            }
            "CompletePayment" => {
                let value: CompletePaymentRequest = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = Payments::complete_payment(self, ctx, &value).await?;
                let buf = serialize(&resp)?;
                Ok(Message {
                    method: "Payments.CompletePayment",
                    arg: Cow::Owned(buf),
                })
            }
            "GetPaymentMethods" => {
                let resp = Payments::get_payment_methods(self, ctx).await?;
                let buf = serialize(&resp)?;
                Ok(Message {
                    method: "Payments.GetPaymentMethods",
                    arg: Cow::Owned(buf),
                })
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "Payments::{}",
                message.method
            ))),
        }
    }
}

/// PaymentsSender sends messages to a Payments service
/// client for sending Payments messages
#[derive(Debug)]
pub struct PaymentsSender<T: Transport> {
    transport: T,
}

impl<T: Transport> PaymentsSender<T> {
    /// Constructs a PaymentsSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }

    pub fn set_timeout(&self, interval: std::time::Duration) {
        self.transport.set_timeout(interval);
    }
}

#[cfg(target_arch = "wasm32")]
impl PaymentsSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for sending to a Payments provider
    /// implementing the 'wasmcloud:example:payments' capability contract, with the "default" link
    pub fn new() -> Self {
        let transport = wasmbus_rpc::actor::prelude::WasmHost::to_provider(
            "wasmcloud:example:payments",
            "default",
        )
        .unwrap();
        Self { transport }
    }

    /// Constructs a client for sending to a Payments provider
    /// implementing the 'wasmcloud:example:payments' capability contract, with the specified link name
    pub fn new_with_link(link_name: &str) -> wasmbus_rpc::RpcResult<Self> {
        let transport = wasmbus_rpc::actor::prelude::WasmHost::to_provider(
            "wasmcloud:example:payments",
            link_name,
        )?;
        Ok(Self { transport })
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> Payments for PaymentsSender<T> {
    #[allow(unused)]
    /// AuthorizePayment - Validates that a potential payment transaction
    /// can go through. If this succeeds then we should assume it is safe
    /// to complete a payment. Payments _cannot_ be completed without getting
    /// a validation code (in other words, all payments have to be pre-authorized).
    async fn authorize_payment(
        &self,
        ctx: &Context,
        arg: &AuthorizePaymentRequest,
    ) -> RpcResult<AuthorizePaymentResponse> {
        let buf = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Payments.AuthorizePayment",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "AuthorizePayment", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    /// Completes a previously authorized payment.
    /// This operation requires the "authorization code" from a successful
    /// authorization operation.
    async fn complete_payment(
        &self,
        ctx: &Context,
        arg: &CompletePaymentRequest,
    ) -> RpcResult<CompletePaymentResponse> {
        let buf = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Payments.CompletePayment",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "CompletePayment", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    /// `GetPaymentMethods` - Retrieves an _opaque_ list of payment methods,
    /// which is a list of customer-facing method names and the
    /// _[tokens](https://en.wikipedia.org/wiki/Tokenization_(data_security))_
    /// belonging to that payment method. You could think of this list as
    /// a previously saved list of payment methods stored in a "wallet".
    /// A payment method _token_ is required to authorize and subsequently
    /// complete a payment transaction. A customer could have previously
    /// supplied their credit card and user-friendly labels for those methods
    /// like "personal" and "work", etc.
    async fn get_payment_methods(&self, ctx: &Context) -> RpcResult<PaymentMethods> {
        let buf = *b"";
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Payments.GetPaymentMethods",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "GetPaymentMethods", e)))?;
        Ok(value)
    }
}
