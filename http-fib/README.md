# HTTP Fibonnaci Example

This example showcases a common performance metric, the recursive fibonnaci calculation, in both cached and uncached forms.

Because this example works well with a few capability providers and can be iterated on, it's also a good example to try out the "watch" feature of `wash` where an actor can be hot-reloaded as you iterate.

## Hot reloading setup
The [tasks.json](./.vscode/tasks.json) file can be used to configure a default build task in VSCode (see [tasks](https://code.visualstudio.com/docs/editor/tasks)) which just runs the `make build` step when you run the default build task. On MacOS, the hotkey is `CMD+Shift+B`.

If you prefer to not setup the build task in VSCode, or you use a different IDE, you can always manually run `make build` in a separate terminal window to trigger the reload.

## Running this example
To start, run `make run`. This will build and sign your actor, and then start a wash REPL with arguments to hotwatch the `http_fib` actor and launch the `httpserver` and `inmemory-keyvalue` providers.

The actor has a basic `/hello` handler to start, and we're going to iterate on it to demonstrate hot reloading. Now that you're in the REPL, run `ctl get claims` in order to view your actor's claims information. In the following `ctl` commands, you'll want to replace the `ACTOR_KEY` with the 56 character `Subject`, which will look like `MDVUB2IKGSH634XUWJSKJPSCLLLUCPOZL3HTYYCE6IC34A42UOHWTQ4P`. This is because your private key is different than what was used to sign the actor while writing this tutorial.

To test the basic `/hello` endpoint, run:
```shell
> ctl call ACTOR_KEY HandleRequest {"method": "GET", "body":"", "queryString":"", "header":{}, "path": "/hello"}
# Response in OUTPUT window:
Call response (raw): ��statusCode�Ȧstatus�OK�header��body�"world"
```

Next, let's test our fibonacci function. Uncomment the lines of code indicated by `Step 1` in [src/lib.rs](./src/lib.rs), which will be the `/fib` handler and the `fib` function, then save. Build the actor by running `make build` or by using the VSCode build task, but keep the REPL running!

Once you have rebuilt your actor, you'll see this message in the Output window:
```
Actor ACTOR_KEY updated to target/wasm32-unknown-unknown/debug/http_fib_s.wasm
```
This means your actor has successfully live updated.

```shell
> ctl call ACTOR_KEY HandleRequest {"method": "GET", "body":"", "queryString":"", "header":{}, "path": "/fib/5"}
# Response in OUTPUT window:
Call response (raw): ��statusCode�Ȧstatus�OK�header��body�8
```

Feel free to try out other values, but note that values over 32 may exceed the deadline, and 46 is the maximum possible value that doesn't overflow a `u32`.

Next, let's take advantage of caching to improve our calculation speed and prevent timeouts. Uncomment the lines indicated by `Step 2`, save, and build your project again.

```shell
> ctl call ACTOR_KEY HandleRequest {"method": "GET", "body":"", "queryString":"", "header":{}, "path": "/cachefib/30"}
# Response in OUTPUT window:
Call response (raw): ��statusCode�Ȧstatus�OK�header��body�832040
```

In the `Tui Log` window, you should see a few wapc host calls for GET and SET, that's where we're interacting with the inmemory-keyvalue database. Now you should be able to compute fibonacci numbers with cache up to 46 in a fraction of the time!

!!NOTE!!: This entire time we have been _directly_ invoking the `HandleRequest` handler with this actor, we haven't used the http server at all. The manifest specifies an active link definition with the httpserver provider, so you can open another terminal at any time and run:
```shell
curl localhost:8080/cachefib/30
```
to test your actor with an active running provider.

## Conclusion
Using a default build task and the watch functionality of `wash` greatly improves development iteration time. Using this, you'll hardly realize your code is being compiled to WebAssembly and live-updated due to how quickly it takes place.