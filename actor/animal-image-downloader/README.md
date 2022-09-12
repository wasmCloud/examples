# Animal Image Downloader Actor

This actor accepts requests over a message broker to download a random image of a specified animal (supported are dogs, cats, and a fallback for unknown animal types).

## Required Capability Claims
1. `wasmcloud:httpclient` to request and download an animal picture
1. `wasmcloud:messaging` to receive messages on a topic
1. `wasmcloud:blobstore` to save the image to a blob

## Running this example
This example requires capability providers that fulfill the above contracts. The wasmCloud [HTTP Client](), [NATS Messaging](), and [Filesystem]() capability providers implement this functionality but you're welcome to use any implementation.

Once you've installed **wash** and ran wasmCloud after following the [installation guide](https://wasmcloud.dev/overview/installation/), you can run this example actor and the wasmCloud providers with the following commands:
```
wash ctl start actor wasmcloud.azurecr.io/animal-image-downloader:0.1.0
# If you use a locally build actor, replace the actor ID below with your own
wash ctl link put MDBIB35BEIFT552CBSJXY3TOQYGIDAWZMMX4TKD5AGMRSJUDYSDCDWDF VBBQNNCGUKIXEWLL5HL5XJE57BS3GU5DMDOKZS6ROEWPQFHEDP6NGVZM wasmcloud:blobstore "ROOT=/tmp"
wash ctl link put MDBIB35BEIFT552CBSJXY3TOQYGIDAWZMMX4TKD5AGMRSJUDYSDCDWDF VCCVLH4XWGI3SGARFNYKYT2A32SUYA2KVAIV2U2Q34DQA7WWJPFRKIKM wasmcloud:httpclient
wash ctl link put MDBIB35BEIFT552CBSJXY3TOQYGIDAWZMMX4TKD5AGMRSJUDYSDCDWDF VADNMSIML2XGO2X4TPIONTIC55R2UUQGPPDZPAVSC2QD7E76CR77SPW7 wasmcloud:messaging "SUBSCRIPTION=wasmcloud.animal.*"
wash ctl start provider wasmcloud.azurecr.io/blobstore-fs:0.1.0 --skip-wait
wash ctl start provider wasmcloud.azurecr.io/httpclient:0.5.3 --skip-wait
wash ctl start provider wasmcloud.azurecr.io/nats_messaging:0.14.5 --skip-wait
```

After running the above, you can use the [NATS CLI](https://github.com/nats-io/natscli) to request a dog, cat, or mystery image:
```
nats req wasmcloud.animal.dog '{}'
nats req wasmcloud.animal.cat '{}'
nats req wasmcloud.animal.idk '{}'
```