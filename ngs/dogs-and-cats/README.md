# DogsAndCats Actor

This actor implements a simple piece of functionality: source and display a random image of either a cat or a dog.

This actor uses the following capabilities:
1. `wasmcloud:httpserver` to receive HTTP requests
1. `wasmcloud:httpclient` to make HTTP requests to public APIs for animal image URLs, and then query that URL for the picture
1. `wasmcloud:builtin:numbergen` to generate a random number to decide if a cat or dog shoud be fetched

The following commands can be run to start this actor, two capability provider implementations that satisfy the contracts, and link them together. Note that the actor public key here (starting with `MCUCZ`) assumes you're using the wasmCloud published version of this actor.
```bash
wash ctl start actor wasmcloud.azurecr.io/dogsandcats:0.1.0
wash ctl link put MCUCZ7KMLQBRRWAREIBQKTJ64MMQ5YKEGTCRGPPV47N4R72W2SU3EYMU VAG3QITQQ2ODAOWB5TTQSDJ53XK3SHBEIFNK4AYJ5RKAX2UNSCAPHA5M wasmcloud:httpserver ADDRESS=0.0.0.0:8081
wash ctl link put MCUCZ7KMLQBRRWAREIBQKTJ64MMQ5YKEGTCRGPPV47N4R72W2SU3EYMU VCCVLH4XWGI3SGARFNYKYT2A32SUYA2KVAIV2U2Q34DQA7WWJPFRKIKM wasmcloud:httpclient
wash ctl start provider wasmcloud.azurecr.io/httpserver:0.15.0
wash ctl start provider wasmcloud.azurecr.io/httpclient:0.4.0
```