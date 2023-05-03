# Cryptographic Signatures Tutorial

This document will guide you through creation of a wasmcloud microservice that performs ed25519 cryptographic signatures and signature verification, with a REST api, and secure key storage using Hashicorp Vault.
All the code for this microservice is available in the wasmcloud github repository. The service is componse of two actors, three capability contracts (Key-Value, Logging, and random number generation).

Here's a diagram of the message paths for signing and verification. The sign and verify actors are written in rust and compiled to WebAssembly modules.

![image-actor](./img/sign-actor.png)

![image-actor](./img/verify-actor.png)

## When should you use this pattern?

- You want to calculate cryptographic signatures over data (documents, text, or binary data), or verify signatures
- You want to store the signing key securely, using Hashicorp Vault, and (may) want to have fine-grained control over which actors can access it
- You want the signing or verification to be done by an actor, as part of a wasmcloud application.

### Why do the signing in an actor and not a capability provider?

You can of course create or verify signatures from within a capability provider, however for many use cases an actor may be preferable:

- the actor already has the data that needs to be signed, and you want to avoid an extra rpc copy of the data from actor to provider
- you want to take advantage of actor scaling and distribution to handle variable load, or high load
- you want a "lighter weight" development process, with the portability advantages of actors, and want to leverage existing open source capability providers without writing a new one.
- the signature algorithm can be compiled to wasm, and you don't need GPU or SIMD support. (Some WASM runtimes do support SIMD, but support is still incomplete or unoptimized on some platforms, as of Jan 2023).
  The demo implementation uses ed25519, however the architecture can be easily adapted to change the signing algorithm, key format, or even implementation (for example, if you want to use HSMs or Cloud KMS services).

### Why use Hashicorp Vault?

Hashicorp Vault provides a convenient single-executable server for securely storing and managing secrets. It can run on most OS's directly, or in docker, and it can use a variety of storages and databases for persistence.
This tutorial does not cover advanced Vault configuration, but we'll walk you through a basic setup and provide some pointers for customizing the setup based on different use cases.

If you have never used Vault, or aren't familiar with its basic concepts, we recommend you review [What is Vault](https://developer.hashicorp.com/vault/tutorials/getting-started/getting-started-intro).
The [Getting started tutorial](https://developer.hashicorp.com/vault/tutorials/getting-started) contains a gentle introduction to secrets storage with Vault and installation options.

## Prerequisites

You'll need to install [wash](https://wasmcloud.dev/overview/installation/).

You'll also need a running Vault server. Hashicorp's [Vault installation guide](https://developer.hashicorp.com/vault/docs/install) page describes installation methods for various platforms.

To keep this tutorial simple, we'll start the vault server in developer mode. In the default developer mode, vault runs as a single server (no clustering), stores all data in-memory (no persistence), and uses an unencrypted connection to http://127.0.0.1:8200 (no TLS).

```
vault server -dev
```

You should see some log messages on the console as the server starts,
including a value for the "Root Token". When the development server is started, it saves a copy of the root token in the file `$HOME/.vault-token`.
All the following vault commands need a token to access the vault. If the file `$HOME/.vault-token` does not exist, vault will use the value in the environment variable `VAULT_TOKEN`.

Enable the kv v2 secrets engine ([kv v2 secrets engine](https://developer.hashicorp.com/vault/docs/secrets/kv/kv-v2))

```
vault secrets enable -version=2 -local -address=http://127.0.0.1:8200 kv
```

You should see a message `Success! Enavbled the kv secrets engine at: kv/`

To avoid needing to type the `-address=http://127.0.0.1:8200` parameter for all of our commands, set the address in the environment

```
  export VAULT_ADDR=http://127.0.0.1:8200
```

## Create a public+private key pair and store them in the vault

Create the ed25519 signing (private) key with openssl. (Note for macos users: the system-installed openssl doesn't work for this (at least for Monterey), but after installing version 3 with homebrew this binary works: `$HOMEBREW_PREFIX/opt/openssl/bin/openssl` )
The second command below uses the public key to generate the private key.

```sh
openssl genpkey -algorithm ed25519  > key-priv.pem
openssl pkey -in key-priv.pem -pubout > key-pub.pem
```

The file key-priv.pem should look something like this:

```pem
-----BEGIN PRIVATE KEY-----
MC4CAQAwBQYDK2VwBCIEIA1suKTqTkstXD+zVc5czgSdPIWwL6t0OsJOmICCQsdQ
-----END PRIVATE KEY-----
```

Now, store them in the vault. Although technically, the private key could be used for both signing and validation,
(since the validator can derive the public key from the private one), we may want to deploy the validator on a different
server, or as part of a different application, and it really only needs the public key. In keeping with the Principle of Least Privilege (PLP), we're
only going to give the validator access to the public key. Key access will be limited by the key path parameter
of the Link Definition.

If your wasmcloud application also uses an alternate key-value store implementation, such as Redis,
you might choose to store the public validation key in there. One of the benefits of the
capability contract is that the storage implementation and storage location are abstracted away from the actor.
Changing a key value implementation only requires using different parameters for the Link Definition.
To keep this demo a little simpler, we will use Vault for both keys.

```shell
VAULT_ADDR=http://127.0.0.1:8200 vault kv put kv/demo/public-key key=- < .key-pub.pem
VAULT_ADDR=http://127.0.0.1:8200 vault kv put kv/demo/private-key key=- < .key-priv.pem
```

To check that the value was stored correctly,

```sh
VAULT_ADDR=http://127.0.0.1:8200 vault kv get -format json kv/demo/public-key | jq -r '.data.data.key'
```

## Compile the actors

Each actor source folder contains a file `wasmcloud.toml` with settings used by `wash build`.
This is where you you can set the project name, a version number (which should be updated for each release), and the list of capability claims.

```
cd sign
wash build
cd ../verify
wash build
```

After compiling, the file build/<PROJECT>\_s.wasm is created. The `_s` suffix on the file name means it is signed for use by wasmcloud.
You can see some information about the signed file with `wash claims inspect build/sign_s.wasm`. The output will show the actor's module id (starts with 'M'), also known as its public key, any call aliases used by the actor, and a list of signed claims.

## Start the host and components

Start the wasmcloud host with `wash up`.

Open a web browser to the host's control dashboard. `http://127.0.0.1:22000`.

- [ ] Select "Start Actor", From File (hot reload), and enter the path to examples/crypto/sign/build/sign_s.wasm.
- [ ] Select "Start Actor", From File (hot reload), and enter the path to examples/crypto/verify/build/verify_s.wasm.

Start the Start the httpserver provider and the vault provider.

- [ ] Select "Start Provider", "From Registry". For Desired Host, use the local host id. Use OCI Reference `wasmcloud.azurecr.io/httpserver:0.17.0`, link name `default`
- [ ] Select "Start Provider", "From Registry". For Desired Host, use the local host id. Use OCI Reference `wasmcloud.azurecr.io/kv-vault:0.4.0`, link name `default`

Set up links, pairing each of the two actors with both providers:

- [ ] "Define Link", Actor:sign, Provider:HTTP Server, Contract: `wasmcloud:httpserver`, Values: `port=9901`. Submit
- [ ] "Define Link", Actor:verify, Provider:HTTP Server, Contract: `wasmcloud:httpserver`, Values: `port=9902`. Submit
- [ ] "Define Link", Actor:sign, Provider:KeyValue:Hashicorp Vault, Contract: `wasmcloud:keyvalue`, Values: `mount=kv,token=****` (replace `****` with the value of your root token in $HOME/.vault-token)
- [ ] "Define Link", Actor:verify, Provider:KeyValue:Hashicorp Vault, Contract: `wasmcloud:keyvalue`, Values: `mount=kv,token=****` (replace `****` with the value of your root token in $HOME/.vault-token)

### For further exploration ...

:small_blue_diamond: When using the kv-vault provider, the optional `mount` parameter of the LinkDefinition defines the key namespace.
Since the sign and verify actors use different link definitions, with different mount parameters and key paths,
the access policies can be configured separately.

:small_blue_diamond: Although the actors are also linked with the logging provider and numbergen provider, we don't need to explicitly create links.
If an actor is signed with claims for a builtin capability (`wasmcloud:builtin:login` or `wasmcloud:builtin:numbergen`), it is automatically linked when it's started.
The capability claims attached to the compiled WASM module are determined by the `claims` setting in `wasmcloud.toml`. You can view claims on a signed module (with name ending in `_s.wasm`) with `wash claims inspect FILE_s.wasm`.

:small_blue_diamond: You can see the link definitions defined in your lattice with `wash ctl link query`

## Sign a file

Sign the file $FILE and save the signature in $FILE.sig

```shell
FILE=/path/to/file-to-sign
curl -T $FILE 'http://127.0.0.1:9901/sign?key=demo/private-key' > $FILE.sig
```

Verify the signature. If the signature is valid, the http call returns status 200. If the signature is invalid, the call returns HTTP status 403.
If there are any other errors, the status returned is 400.

```shell
SIG=$(cat $FILE.sig)
curl -T $FILE "http://127.0.0.1:9902/verify?key=demo/public-key&sig=$SIG"
```

## Congratulations!

You've set up a wasmcloud microservice for signing documents and verifying signatures, with secure key storage using Hashicorp Vault, and a REST api.
