// payments.smithy
//
// Sample api for a simple payments provider
//

// Tell the code generator how to reference symbols defined in this namespace
metadata package = [{
    namespace: "org.wasmcloud.examples.payments",
    crate: "wasmcloud_example_payments",
    py_module: "wasmcloud_example_payments",
}]

namespace org.wasmcloud.examples.payments

use org.wasmcloud.model#wasmbus
use org.wasmcloud.model#U32
use org.wasmcloud.model#U64

@wasmbus(
    contractId: "wasmcloud:example:payments",
    providerReceive: true )
service Payments {
  version: "0.1",
  operations: [ AuthorizePayment, CompletePayment, GetPaymentMethods ]
}

/// AuthorizePayment - Validates that a potential payment transaction
/// can go through. If this succeeds then we should assume it is safe
/// to complete a payment. Payments _cannot_ be completed without getting
/// a validation code (in other words, all payments have to be pre-authorized).
operation AuthorizePayment {
    input: AuthorizePaymentRequest,
    output: AuthorizePaymentResponse,
}

/// Completes a previously authorized payment.
/// This operation requires the "authorization code" from a successful
/// authorization operation.
operation CompletePayment {
    input: CompletePaymentRequest,
    output: CompletePaymentResponse,
}


/// `GetPaymentMethods` - Retrieves an _opaque_ list of payment methods,
/// which is a list of customer-facing method names and the
/// _[tokens](https://en.wikipedia.org/wiki/Tokenization_(data_security))_
/// belonging to that payment method. You could think of this list as
/// a previously saved list of payment methods stored in a "wallet".
/// A payment method _token_ is required to authorize and subsequently
/// complete a payment transaction. A customer could have previously
/// supplied their credit card and user-friendly labels for those methods
/// like "personal" and "work", etc.
@readonly
operation GetPaymentMethods {
    output: PaymentMethods
}

/// Parameters sent for AuthorizePayment
structure AuthorizePaymentRequest {
    /// Amount of transaction, in cents.
    @required
    @n(0)
    amount: U32,

    /// Amount of tax applied to this transaction, in cents
    @required
    @n(1)
    tax: U32,

    /// Token of the payment method to be used
    @required
    @n(2)
    paymentMethod: String,

    /// The entity (customer) requesting this payment
    @required
    @n(3)
    paymentEntity: String,

    /// Opaque Reference ID (e.g. order number)
    @required
    @n(4)
    referenceId: String,
}


/// Response to AuthorizePayment
structure AuthorizePaymentResponse {
    /// Indicates a successful authorization
    @required
    success: Boolean,

    /// Optional string containing the tx ID of auth
    authCode: String,

    /// Optional string w/rejection reason
    failReason: String,
}


/// Confirm the payment (e.g., cause the transaction amount
/// to be withdrawn from the payer's account)
structure CompletePaymentRequest {

    /// authorization code from the AuthorizePaymentResponse
    @required
    authCode: String,

    /// An optional description field to be added to the payment summary
    /// (e.g., memo field of a credit card statement) |
    description: String
}

structure CompletePaymentResponse {

    /// True if the payment was successful
    @required
    success: Boolean,


    /// Transaction id issued by Payment provider
    @required
    txid: String,

    /// Timestamp (milliseconds since epoch, UTC)
    @required
    timestamp: U64,
}

/// A PaymentMethod contains a token string and a description
structure PaymentMethod {
    token: String,
    description: String,
}

/// An ordered list of payment methods.
list PaymentMethods {
    member: PaymentMethod
}


