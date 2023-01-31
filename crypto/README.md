
# wasmCloud Cryptography Examples

## Signatures

- [ed25519 signatures tutorial](./signature-demo.md) REST microservice for cryptographically signing documents and verifying signatures, using the keyvalue capability contract and Hashicorp Vault for secret key storage.
  - Actors
    - [sign](./sign/) - sign http payload and return signature
    - [verify](./verify/) - verify signature
  - Capabilities used
    - [wasmcloud:keyvalue](https://github.com/wasmCloud/interfaces/tree/main/keyvalue), for key storage. Implementation: [kv-value](https://github.com/wasmCloud/capability-providers/tree/main/kv-vault) Hashicorp Vault
    - [wasmcloud:builtin:logging](https://github.com/wasmCloud/interfaces/tree/main/logging)
    - [wasmcloud:builtin:numbergen](https://github.com/wasmCloud/interfaces/tree/main/numbergen) for random nonce generation
    - [wasmcloud:httpserver](https://github.com/wasmCloud/interfaces/tree/main/httpserver) for REST api. Implementation: [httpserver-rs](https://github.com/wasmCloud/capability-providers/tree/main/httpserver-rs)

