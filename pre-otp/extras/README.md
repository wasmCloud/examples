## How to use  built-in `wasmcloud:extras` capability provider

The `wasmcloud` host runtime comes with a built-in capability provider (wasmcloud:extras) that includes some useful and fairly common tools. In this tutorial we will generate a random number and retrieve some information like request ID and number of requests using `wasmcloud:extras`. 

1. Please install `wasmcloud` and `wash` binaries if you do not have them installed already:

    [https://wasmcloud.dev/overview/installation/](https://wasmcloud.dev/overview/installation/)

    ```bash
    $ wasmcloud --version
    $ wash --version
    ```

2. Compile and sign the actor:

    ```bash
    $ cargo build --release

    $ wash claims sign ./target/wasm32-unknown-unknown/release/extras.wasm -c wasmcloud:httpserver -c c wasmcloud:extras --name "extras" --ver 0.2.1
    ```

    Please note we are signing our actor for multiple capabilities including the built-in ones: `wasmcloud:httpserver` and `wasmcloud:extras`. Failing to sign our actor with those capabilities will prevent our actor from using them, as wasmcloud uses a deny-by-default security model for capabilities.

3. Get the actor ID:

    ```bash
    $ wash claims inspect ./target/wasm32-unknown-unknown/release/extras_s.wasm
    ```

    This will output cryptographically signed ID for our actor like:

    ```bash
     Module       MDAR7HG42IAZBHJQNS2CQPYJPCSN55CDCKBYSUDVY2R4ZB6YDFONID2B 
    ```

4. Now it is time to get the provider ID for `wasmcloud:extras`:

    ```
    // Start a REPL session
    $ wash up

    // Get the host ID
    $ ctl get hosts

    // Retrieve the inventory details using the host ID
    $ wash ctl get inventory «HOST_ID»
    ```

    This will output something like:

    ```bash
    Provider ID                                                Link Name                  Image Reference
    VAHNM37G4ARHZ3CYHB3L34M6TYQWQR6IZ4QVYC4NYZWTJCJ2LWP7S6Z2   __wasmcloud_lattice_cache  N/A
    VDHPKGFKDI34Y4RN4PWWZHRYZ6373HYRSNNEM4UTDLLOGO5B37TSVREP   default                    N/A          
    ```

    We need the ID for the `default` one. We are using remote capability provider for `wasmcloud:httpserver` and its ID is provided below.
    
    We are in the process of adding the provider name into the inventory output.

5. You can create `manifest` file with the IDs you get above or use the one, `manifest.yaml`, in the project's directory as an example. You can use that file as a base but you'll have to replace the actor IDs in the links section with the actor ID that you found above in step 3:

    ```bash
    # manifest.yaml
    labels:
        sample: "wasmcloud extra"
    actors:
        - "./target/wasm32-unknown-unknown/release/extras_s.wasm"
    capabilities:
        - image_ref: wasmcloud.azurecr.io/httpserver:0.11.1
          link_name: default
    links:
        - actor: «ACTOR_ID»
          # Remote provider ID, no need to change unless you use different image
          provider_id: "VAG3QITQQ2ODAOWB5TTQSDJ53XK3SHBEIFNK4AYJ5RKAX2UNSCAPHA5M"
          contract_id: "wasmcloud:httpserver"
          link_name: default
          values:
            PORT: 8080
        - actor: «ACTOR_ID»
          provider_id: "VDHPKGFKDI34Y4RN4PWWZHRYZ6373HYRSNNEM4UTDLLOGO5B37TSVREP"
          contract_id: "wasmcloud:extras"
          link_name: default
    ```

6. Now it's time to run the actor:

    ```bash
    $ wasmcloud -m manifest.yaml
    ```

7. Visit `http://localhost:8080` to see the result:

    ```bash
    curl http://localhost:8080/
    ```

    You will get a response like:
    
    ```json
     `{"guid":"76d9f7c4-13e9-4b15-b200-e295ac5dee31","random":99,"sequence":7}`
    ```
