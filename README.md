# wasmCloud Examples

Example actors, capability providers, and other demonstrations

## Actors

The following is a list of example actors for use with wasmCloud hosts.

| Example | Description | OCI Reference (refer to example for latest version) |
|---|---|---|
| [echo](https://github.com/wasmcloud/examples/tree/main/actor/echo) | An actor that returns a JSON payload describing the incoming request |  `wasmcloud.azurecr.io/echo` |
| [echo-messaging](./actor/echo-messaging) | Actor that echoes messages received via message broker provider | `wasmcloud.azurecr.io/echo-messaging` |
| [hello](./actor/hello) | Canonical "hello world" example invoked via HTTP provider | `wasmcloud.azurecr.io/hello` |
| [kvcounter](https://github.com/wasmcloud/examples/tree/main/actor/kvcounter) | An actor that uses the key-value store to increment a counter and return a value for every HTTP request it receives | `wasmcloud.azurecr.io/kvcounter` |
| [logging](./actor/logging) | Demonstrates the consumption of the built-in logging provider | `wasmcloud.azurecr.io/example-logging` |
| [random](./actor/random) | Demonstrates using the built-in number generator provider | `wasmcloud.azurecr.io/example-random` |
| [todo](./actor/todo) | Implementation of the TODO backend spec | `wasmcloud.azurecr.io/todo` |
| [xkcd](./actor/xkcd) | XKCD comic generator | `wasmcloud.azurecr.io/xkcd` |

## Interfaces
The following is a list of example interfaces made available to consumers from [Smithy](https://awslabs.github.io/smithy/)-defined models.

| Example | Description |
|---|---|
| [payments](./interface/payments) | Example interface for a payments provider |

## Providers
The following is a list of example capability providers

| Example | Description |
|---|---|
| [factorial](./provider/factorial) | Implementation of the `wasmcloud:example:factorial` provider contract |
| [fakepay](./provider/fakepay) | Implementation of the `wasmcloud:example:payments` capability contract ID for the "payments" sample in the official documentation. |

## Reference Applications

| Example | Description | 
|---|---|
| [Pet Clinic](./petclinic) | A wasmCloud implementation of the classic Spring Boot/Cloud Pet Clinic |


## ⚠️ Pre-OTP examples moved ⚠️

Examples compatible with the `0.18` wasmCloud host and earlier have been moved to the
[pre-otp](./pre-otp) folder.


