# Cryptographic Signatures Demo

This document will guide you through setup for two actors, one that creates an ed25519 cryptographic signature of a document,
and one that verifies the signature. The signing keys are stored in a Hashicorp Vault, and accessed by the actors 
using the keyvalue capability contract. This demo also uses the numbergen capability contract, for creating of
a random nonce (used by the signature algorithm), and the httpserver capability, so the actors can be accessed with a REST api.

Hashicorp Vault provides a convenient single-executable server for securely storing and managing secrets.
This tutorial does not cover advanced Vault configuration, but we'll
walk you through the basic setup and provide some pointers for customizing the setup based on different use cases.

If you have never used Vault, or aren't familiar with its basic concepts, we recommend you review [What is Vault](https://developer.hashicorp.com/vault/tutorials/getting-started/getting-started-intro).
The [Getting started tutorial](https://developer.hashicorp.com/vault/tutorials/getting-started) contains a gentle introduction to secrets storage with Vault and installation options.

![image-actor](./img/sign-actor.png)

![image-actor](./img/verify-actor.png)


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

```
cd sign
wash build
cd ../verify
wash build
```

## Start the host and components

Start the wasmcloud host with `wash up`.

Open a web browser to the host's control dashboard. `http://127.0.0.1:22000`.
- [ ] Select "Start Actor", From File (hot reload), and enter the path to examples/crypto/sign/build/sign_s.wasm.
- [ ] Select "Start Actor", From File (hot reload), and enter the path to examples/crypto/verify/build/verify_s.wasm.

We're going to start two instances of the httpserver provider and the vault provider.
- [ ] Select "Start Provider", "From Registry". For Desired Host, use the local host id. Use OCI Reference `wasmcloud.azurecr.io/httpserver:0.17.0`, link name `sign`
- [ ] again with link name `verify`
- [ ] Select "Start Provider", "From Registry". For Desired Host, use the local host id. Use OCI Reference `wasmcloud.azurecr.io/kv-vault:0.3.0`, link name `sign`
- [ ] again with link name `verify`

Set up links:
- [ ] "Define Link", Actor:sign, Provider:HTTP Server, Link: `sign`, Contract: `wasmcloud:httpserver`, Values: `port=9901`. Submit
- [ ] "Define Link", Actor:verify, Provider:HTTP Server, Link: `verify`, Contract: `wasmcloud:httpserver`, Values: `port=9902`. Submit
- [ ] "Define Link", Actor:sign, Provider:KeyValue:Hashicorp Vault, Link: `sign`, Contract: `wasmcloud:keyvalue`, Values: `mount=kv,token=****`  (replace `****` with the value of your root token from above)
- [ ] "Define Link", Actor:verify, Provider:KeyValue:Hashicorp Vault, Link: `verify`, Contract: `wasmcloud:keyvalue`, Values: `mount=kv,token=****`  (replace `****` with the value of your root token from above)

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


## Additional security configuration 

When using the kv-vault provider, the optional `mount` parameter of the LinkDefinition defines the key namespace.
Since the sign and verify actors use different link definitions, with different mount parameters and key paths,
the access policies can be configured separately.
