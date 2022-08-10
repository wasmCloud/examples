# Policy Actor

This actor implements a rudimentary policy service as described in [this RFC](https://github.com/wasmCloud/wasmcloud-otp/issues/439). It simply ensures that each actor and provider started in the host are from the official wasmCloud issuer account. With this scaffolding you should be able to see how you could restrict resources to only actors and providers that are released by trusted issuers.

This actor only needs the `wasmcloud:messaging` capability in order to receive evaluate policy requests.

To start the actor you'll need [wash](https://github.com/wasmcloud/wash) and an accessible wasmCloud host which you can get from our [installation guide](https://wasmcloud.dev/overview/installation/):
```shell
wash ctl start wasmcloud.azurecr.io/example_policy:0.1.0
wash ctl start wasmcloud.azurecr.io/nats_messaging:0.14.2
wash ctl link put MAX4HKZIMZ2E47QNET7ZUP43AIDBGHK5LRAGU3ZGYDMHF74U2UYIELYG VADNMSIML2XGO2X4TPIONTIC55R2UUQGPPDZPAVSC2QD7E76CR77SPW7 wasmcloud:messaging SUBSRIPTION=wasmcloud.policy.evaluator
```