# Echo Example

An actor is the smallest unit of deployable, portable compute within the wasmcloud ecosystem.

Actors are small WebAssembly modules that can handle messages delivered to them by the host runtime and can invoke functions on capability providers, provided they have been granted the appropriate privileges.

Check out https://wasmcloud.dev/reference/host-runtime/actors/ for more.

In this example our actor responds to all incoming HTTP requests by returning it as JSON object.

We will have two part tutorial starting with running a precompiled actor stored in a remote repo, then we will write our own actor and swap it with the remote one. We will use a manifest file to run our actor decleratively via `wasmcloud`.

In the second part we will spin custom NATS server and a local Docker registry to store our WebAssembly module. In this part we will use wasmcloud shell, `wash`, a feature-rich cli tool that provides interactive repl environment to run our actor and interact with it.

## Running an echo server

1. Please install `wasmcloud` and `wash` binaries if you are not already did. `wasmcloud` will provide the runtime environment for our actor while `wash` will be used for development later in the tutorial:

    https://wasmcloud.dev/overview/installation/

    ```bash
    $ wasmcloud --version
    $ wash --version
    ```

2. Locate manifest.yaml file here in the project folder and run it with `--manifest` flag:

    ```bash
    $ wasmcloud -m manifest.yaml
    ```

    Here is the content of the file:

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

    To read more on the security aspects at https://wasmcloud.dev/reference/host-runtime/security/.

6. We need actor id to use in our manifest.yml:

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

    We are after module id: `MCUDTMOOZCVAM5EBNN4U3X2OGHNIY3BKPEW66HY4RTCYYVWXOE7ESVDQ`

7. Swap remote actor with our newly created one in the manifest file and don't forget to change the actor id:

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

## Using `wash` for better developer experience

In this second part, we will spin a NATS server and a local Docker registry to spice things up.

1. We will run NATS servers and Docker's local image registry as our prerequisites.

    NATS is a message broker used by wasmcloud’s self-managing lattice network. The image registry is for to store our WebAssembly modules.

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

    You can find `docker-compose.yml` in the project’s directory. We will run our containers in detached mode to reduce clutter.

    ```bash
    $ docker-compose up -d
    ```

    Start a repl session using `wash up` command:

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

2. Build .wasm file and sign it using make file:

    ```bash
    $ cargo build --release
    $ wash claims sign ./target/wasm32-unknown-unknown/release/echo.wasm -c wasmcloud:httpserver --name "echo" --ver 0.2.2
    ```

    Or you can use project’s make file:

    ```bash
    make release VERSION=0.2.2
    ```

    This will produce `echo.wasm` actor and its signed version `echo_s.wasm`.

3. Push signed actor into to local registry using `wash`:

    ```bash
    wash reg push localhost:5000/echo:0.2.2 ./target/wasm32-unknown-unknown/release/echo_s.wasm --insecure
    ```

4. Run it:

    ```bash
    ctl start actor localhost:5000/echo:0.2.2
    ```

    Steps from now on is independent of our current example but we keep it for completeness:

5. Start your capability that runs an html server:

    ```bash
    ctl start provider wasmcloud.azurecr.io/httpserver:0.11.1
    ```

    Check out https://wasmcloud.dev/app-dev/create-provider for how to write your own:

6. Link actor with capability using `wash`:

    ```bash
    // Get actor and capability provider ids using:
    // ctl get inventory «HOST_ID»

    $ ctl get inventory NDGPVEOZKAYT6JRL2C4BDVSW3PBBOYTWRLC2QFCSLXWDX4KCIRHM6WCZ

    // Establish a contract between them using link command
    // Pass along environment variables required by the capability provider using:
    // ctl link «ACTOR_ID» «PROVIDER_ID» «CONTRACT_ID» PORT=8080

    $ ctl link MCUDTMOOZCVAM5EBNN4U3X2OGHNIY3BKPEW66HY4RTCYYVWXOE7ESVDQ VAG3QITQQ2ODAOWB5TTQSDJ53XK3SHBEIFNK4A YJ5RKAX2UNSCAPHA5M wasmcloud:httpserver PORT=8080 
    ```

7. Visit `http://localhost:8080` or send a request:

    ```bash
    curl http://localhost:8080/
    ```

    You can use `wash` to test your actor:

    ```bash
    ctl call «ACTOR_ID» HandleRequest {"method": "GET", "path": "/", "body": "", "queryString":"","header":{}}
    ```
    This will output response JSON along with some raw bytes which our repl terminal does not know how to interpret:

    ```bash
    Call response (raw): ��statusCode�Ȧstatus�OK�header��body�D{"method":"GET","path":"/","query_string":"","headers":{},"body":[]}
    ```

