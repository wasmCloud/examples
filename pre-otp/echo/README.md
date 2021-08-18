# Echo Example

An actor is the smallest unit of deployable, portable compute within the wasmcloud ecosystem.

Actors are small WebAssembly modules that can handle messages delivered to them by the host runtime and can invoke functions on capability providers, provided they have been granted the appropriate privileges.

Check out [https://wasmcloud.dev/reference/host-runtime/actors/](https://wasmcloud.dev/reference/host-runtime/actors/) for more.

In this example our actor responds to all incoming HTTP requests by returning the HTTP request as a JSON object.

We will have a two part tutorial, starting with running a precompiled actor stored in a remote  OCI registry, then we will write our own actor and swap it with the remote one. We will use a manifest file to run our actor declaratively via `wasmcloud`.

In the second part, we will spin up a NATS server and a local docker registry to store our WebAssembly module. We'll use the wasmcloud shell, `wash`, a feature-rich cli tool that provides an interactive REPL environment to run our actor and interact with it.

## Running an echo server

1. Please install `wasmcloud` and `wash` binaries if you do not have them installed already. `wasmcloud` will provide the runtime environment for our actor while `wash` will be used for development later in the tutorial:

    [https://wasmcloud.dev/overview/installation/](https://wasmcloud.dev/overview/installation/)

    ```bash
    $ wasmcloud --version
    $ wash --version
    ```

2. Locate manifest.yaml file here in the project folder and run it with `--manifest` flag:

    ```bash
    $ wasmcloud -m manifest.yaml
    ```

    Here are the contents of the file:

    ```bash
    labels:
        sample: "wasmcloud echo"
    actors:
        - "wasmcloud.azurecr.io/echo:0.2.0"
    capabilities:
        - image_ref: wasmcloud.azurecr.io/httpserver:0.11.1
          link_name: default
    links:
        - actor: ${ECHO_ACTOR:MBCFOPM6JW2APJLXJD3Z5O4CN7CPYJ2B4FTKLJUR5YR5MITIU7HD3WD5}
          provider_id: "VAG3QITQQ2ODAOWB5TTQSDJ53XK3SHBEIFNK4AYJ5RKAX2UNSCAPHA5M"
          contract_id: "wasmcloud:httpserver"
          link_name: default
          values:
            PORT: 8080
    ```

3. Visit `http://localhost:8080` to see the result:

    ```bash
    curl http://localhost:8080/
    ```

    That is it, you are running a highly secure HTTP server powered by a WebAssembly module.

4. Now lets build our own actor and swap with the remote one:

    ```bash
    $ cargo build --release
    ```

5. To be able to use it, we need to sign it:
    
    ```bash
    $ wash claims sign ./target/wasm32-unknown-unknown/release/echo.wasm -c wasmcloud:httpserver --name "echo" --ver 0.2.2
    ```

    Read more on the security aspects at https://wasmcloud.dev/reference/host-runtime/security/.

6. Actors are signed with unique module keys, so the echo actor ID will be different for everyone. We'll need to inspect the actor to find its public key (module) for the next step:

    ```bash
    $ wash claims inspect ./target/wasm32-unknown-unknown/release/echo_s.wasm
    ```

    This will output something like:

    ```bash
    Account      ACPVJRHMJ5GM2EOQWRPAQVKL75NBOCTZC52TAYPDCDA2GLXJ6TVJOPDN 
    Module       MCUDTMOOZCVAM5EBNN4U3X2OGHNIY3BKPEW66HY4RTCYYVWXOE7ESVDQ 
    Expires                                                         never 
    Can Be Used                                               immediately 
    Version                                                     0.2.2 (0) 
    Call Alias                                                  (Not set) 
    ```

    We are after the module ID: `MCUDTMOOZCVAM5EBNN4U3X2OGHNIY3BKPEW66HY4RTCYYVWXOE7ESVDQ`

7. Swap the remote actor with our newly created one in the manifest file and don't forget to change the actor ID:

    ```yaml
    actors:
        - "./target/wasm32-unknown-unknown/release/echo_s.wasm"
    # ...
    actor: ${ECHO_ACTOR:MCUDTMOOZCVAM5EBNN4U3X2OGHNIY3BKPEW66HY4RTCYYVWXOE7ESVDQ}
    # ...
    ```

8. Now it's time to run our actor:

    ```bash
    $ wasmcloud -m manifest.yaml
    ```

9. As a final step we will send another request to `http://localhost:8080`:

    ```bash
    curl http://localhost:8080/
    ```

## Using `wash` for better developer experience

Previously we were using a declarative deployment model that did not allow us to modify resources at runtime. In this section, we will use `NATS` to allow `wasmcloud` to utilize self-managing [lattice network](https://wasmcloud.dev/reference/lattice/), and we can use `wash` to interact with the `wasmcloud` host via the lattice control interface (`ctl`).

We will also use a local docker registry to store our echo actor to illustrate interacting with a local development version of an OCI registry.

1. Since we are going to use `docker-compose` to run NATS server and  image registry, you need to have `wasmcloud`, `wash`, `docker` and `docker-compose` installed on your machine.

2. You can either use `docker-compose.yml` file in the project's directory or create one with the following content:

    ```yaml
    version: "3"
    services:
      registry:
        image: registry:2
        ports:
        - "5000:5000"
      nats:
        image: nats:2.1.9
        ports:
          - "6222:6222"
          - "4222:4222"
          - "8222:8222"
    ```

    We will run our containers in detached mode to reduce clutter.

    ```bash
    $ docker-compose up -d
    ```

    Start a REPL session using `wash up` command:

    ```bash
    $ wash up
    ```

    Check if session is running:

    ```bash
    $ ctl get hosts
    ```

    It will output something like:

    ```bash
    Host ID                                                   Uptime (seconds)
    NDGPVEOZKAYT6JRL2C4BDVSW3PBBOYTWRLC2QFCSLXWDX4KCIRHM6WCZ  1357
    ```

3. Please open a new terminal, as the REPL is running in the current one. We will build our actor first, then will sign it:

    ```bash
    $ cargo build --release
    $ wash claims sign ./target/wasm32-unknown-unknown/release/echo.wasm -c wasmcloud:httpserver --name "echo" --ver 0.2.2
    ```

    Or you can use `make` file in project's directory for convenience:

    ```bash
    make release VERSION=0.2.2
    ```

    Now we have our actor, `echo.wasm` and its signed version `echo_s.wasm` in `./target/wasm32-unknown-unknown/release` directory.

3. Now it's time to push signed our actor into to local registry using `wash`:

    ```bash
    wash reg push localhost:5000/echo:0.2.2 ./target/wasm32-unknown-unknown/release/echo_s.wasm --insecure
    ```

4. Now, please go back to REPL terminal and run the actor from the local image registry:

    ```bash
    ctl start actor localhost:5000/echo:0.2.2
    ```

    The steps from now on are independent of our current example, but we've kept it for completeness:

5. Start your capability provider that runs an HTML server:

    ```bash
    ctl start provider wasmcloud.azurecr.io/httpserver:0.11.1
    ```

    Check out https://wasmcloud.dev/app-dev/create-provider for how to write your own:

6. Link the actor with capability using `wash`:

    ```bash
    // Get actor and capability provider IDs using:
    // ctl get inventory «HOST_ID»

    $ ctl get inventory NDGPVEOZKAYT6JRL2C4BDVSW3PBBOYTWRLC2QFCSLXWDX4KCIRHM6WCZ

    // Establish a contract between them using link command
    // Pass along environment variables required by the capability provider using:
    // ctl link «ACTOR_ID» «PROVIDER_ID» «CONTRACT_ID» PORT=8080

    $ ctl link MCUDTMOOZCVAM5EBNN4U3X2OGHNIY3BKPEW66HY4RTCYYVWXOE7ESVDQ VAG3QITQQ2ODAOWB5TTQSDJ53XK3SHBEIFNK4A YJ5RKAX2UNSCAPHA5M wasmcloud:httpserver PORT=8080 
    ```

7. You can use `ctl call` command in the REPL window to test out our actor:
    
    ```bash
    ctl call «ACTOR_ID» HandleRequest {"method": "GET", "path": "/", "body": "", "queryString":"","header":{}}
    ```
    This will output response JSON along with some raw bytes which our REPL terminal does not know how to interpret:

    ```bash
    Call response (raw): ��statusCode�Ȧstatus�OK�header��body�D{"method":"GET","path":"/","query_string":"","headers":{},"body":[]}
    ```

    Or you can query `http://localhost:8080` via `curl` from a seperate terminal window:

    ```bash
    curl http://localhost:8080/
    ```

    Or just visit `http://localhost:8080/` address via browser: