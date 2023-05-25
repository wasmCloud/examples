# cross-actor-call
this example implements:
- a caller actor that implements the runner interface can be called with `wash`
- a responder actor that receives the cross-actor call
- a simple library to store the trait we need for wasmbus-rpc to derive our actor call method

We demonstrate the minimal amount of code required to make a cross actor call (with an alias) in order to show the minimum viable actor

# steps to run
```
cd responder
wash build && wash ctl start actor file://$PWD/build/responder_s.wasm

cd ../caller
wash build && wash ctl start actor file://$PWD/build/responder_s.wasm

wash call $CALLER_ACTOR_ID Runner.Run "[]"
```
