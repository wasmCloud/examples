# ImageHost example

This repository features 3 actors with identical functionality written in Rust, AssemblyScript, and TinyGo. The `Makefile` at the project root can be used to build and run these examples with the included `manifest.yaml`, provided you have `wash` and `wasmcloud` installed.

## Prerequisites
- `wash`
- `wasmcloud`
- `nats-server`
- `jq` (not required for the demo, only required to use the `run-*` Makefile steps without manually inspecting the wasm for its module key). You can simply change the `make run-*` variable `IMAGE_HOST_PKEY` to match your actor's module key.
- Language specific tools, detailed below

For installation instructions, visit the wasmCloud documentation page [here](https://wasmcloud.dev/overview/installation/). You'll need to have a local running version of `nats-server`. The simplest way to run `nats-server` if you don't already have it installed is to use the docker container:
```shell
docker run -p 4222:4222 -ti nats:latest
```

Note: As of `wasmcloud` version `0.18.2`, you no longer need `nats` to run this demo.

### Language specific requirements
- For the Rust example, you'll need `cargo` installed as well as `wasm32-unknown-unknown` installed as a target. Rust can be installed with a simple script from https://www.rust-lang.org/tools/install, and you can install the wasm toolchain with `rustup target add wasm32-unknown-unknown`.
- For the AssemblyScript example, you'll need `npm` and `asc` installed, quick start instructions can be found at https://www.assemblyscript.org/quick-start.html
- For the TinyGo example, you'll need `tinygo`, which can be installed from https://tinygo.org/getting-started/macos/.

## Running the example
1. Use the `Makefile` to either `make run-rust`, `make run-go`, or `make run-as`
2. In the `image-host` directory, run `curl -X POST --data-binary @wasmcloud.png localhost:8080/wasmcloud.png` to upload the file to your `/tmp` directory
3. Stop the `wasmcloud` binary with CTRL+C
4. Edit the manifest to comment out the `Filesystem blobstore` section and uncomment the `S3 blobstore` section, and comment out the `fs` capability in place of the `s3` capability
5. Here, you'll need to specify the following AWS environment variables from your environment (simply `export` them in your env, or edit the manifest defaults):
    - `ROOT`
    - `REGION`
    - `AWS_ACCESS_KEY`
    - `AWS_SECRET_ACCESS_KEY`
    - `AWS_TOKEN`
    - `TOKEN_VALID_FOR`
6. Use the `Makefile` again to run the example
7. Create a bucket called `wasmcloud-bucket` in your S3 environment
8. Run the `curl` command specified above, the wasmcloud logo will be uploaded to the S3 bucket `wasmcloud-bucket` without any code recompilation or change in business logic

## Upload image script
```shell
curl -X POST --data-binary @wasmcloud.png localhost:8080/wasmcloud.png
```
