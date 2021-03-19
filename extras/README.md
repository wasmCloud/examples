## How to use  built-in `wasmcloud:extras` capability provider

`wasmcloud` host runtime comes with some built-in capability providers and `wasmcloud:extras` is one of them. In this tutorial we will generate a random number and retrive some information like request id and number of requests using `wasmcloud:extras`. 

1. Please install `wasmcloud` and `wash` binaries if you are not already did:

    https://wasmcloud.dev/overview/installation/

    ```bash
    $ wasmcloud --version
    $ wash --version
    ```

2. Compile and sign the actor:

    ```bash
    $ cargo build --release

    $ wash claims sign ./target/wasm32-unknown-unknown/release/extras.wasm -c wasmcloud:httpserver -c c wasmcloud:extras --name "extras" --ver 0.2.1
    ```

    Please note we are signing our actor for multiple capabilities including built-in ones: `wasmcloud:httpserver` and `wasmcloud:extras`. Failing to do so causes mulfunctioning of the actor.

3. Get actor id:

    ```bash
    $ wash claims inspect ./target/wasm32-unknown-unknown/release/extras_s.wasm
    ```

    This will output cryptographically signed id for our actor like:

    ```bash
     Module       MDAR7HG42IAZBHJQNS2CQPYJPCSN55CDCKBYSUDVY2R4ZB6YDFONID2B 
    ```

4. Now it is time to get provider id for `wasmcloud:extras`:

    ```
    // Start a REPL session
    $ wash up

    // Get host id
    $ ctl get hosts

    // Retrieve inventory detail using host id
    $ wash ctl get inventory «HOST_ID»
    ```

    This will output something like:

    ```bash
    Provider ID                                                Link Name                  Image Reference
    VAHNM37G4ARHZ3CYHB3L34M6TYQWQR6IZ4QVYC4NYZWTJCJ2LWP7S6Z2   __wasmcloud_lattice_cache  N/A
    VDHPKGFKDI34Y4RN4PWWZHRYZ6373HYRSNNEM4UTDLLOGO5B37TSVREP   default                    N/A          
    ```

    We need the id for the `default` one. We are using remote capability provider for `wasmcloud:httpserver` and it's id is provided below.

5. Create a manifest.yaml file with the ids you get above:

    ```bash
    labels:
        sample: "wasmcloud extra"
    actors:
        - "./target/wasm32-unknown-unknown/release/extras_s.wasm"
    capabilities:
        - image_ref: wasmcloud.azurecr.io/httpserver:0.11.1
          link_name: default
    links:
        - actor: «ACTOR_ID»
          # Remote provider id, no need to change unless you use different image
          provider_id: "VAG3QITQQ2ODAOWB5TTQSDJ53XK3SHBEIFNK4AYJ5RKAX2UNSCAPHA5M"
          contract_id: "wasmcloud:httpserver"
          link_name: default
          values:
            PORT: 8080
        - actor: «ACTOR_ID»
          provider_id: «default PROVIDER_ID»
          contract_id: "wasmcloud:extras"
          link_name: default
    ```

  You can use the manifest file provided with the repo but please remember to replace ids with the correct ones:

6. Now it's time to run the actor:

    ```bash
    $ wasmcloud -m manifest.yaml
    ```

7. Visit `http://localhost:8080` to see the result:

    ```bash
    curl http://localhost:8080/
    ```

    You will get a response like `{"guid":"76d9f7c4-13e9-4b15-b200-e295ac5dee31","random":99,"sequence":7}`