# Build the example wasm modules, sign them with the example host keys
# and copy them to the .assets directory

# Echo 
cd echo
cargo build --release 
cd ..
wascap sign echo/target/wasm32-unknown-unknown/release/echo.wasm ../wascc-host/examples/.assets/echo.wasm --issuer ../wascc-host/examples/.assets/act.nk --subject ../wascc-host/examples/.assets/mod1.nk -s --name "Echo Actor"

# Echo 2 (Copy)
wascap sign echo/target/wasm32-unknown-unknown/release/echo.wasm ../wascc-host/examples/.assets/echo2.wasm --issuer ../wascc-host/examples/.assets/act.nk --subject ../wascc-host/examples/.assets/mod2.nk -s --name "Echo Actor 2"

# Extras
cd extras
cargo build --release
cd ..
wascap sign extras/target/wasm32-unknown-unknown/release/extras.wasm ../wascc-host/examples/.assets/extras.wasm --issuer ../wascc-host/examples/.assets/act.nk --subject ../wascc-host/examples/.assets/mod_extras.nk -s -z --name "Extras Demo"

# Subscriber
cd subscriber
cargo build --release
cd ..
wascap sign subscriber/target/wasm32-unknown-unknown/release/subscriber.wasm ../wascc-host/examples/.assets/subscriber.wasm --issuer ../wascc-host/examples/.assets/act.nk --subject ../wascc-host/examples/.assets/mod_sub.nk -g --name "Subscriber Demo"
wascap sign subscriber/target/wasm32-unknown-unknown/release/subscriber.wasm ../wascc-host/examples/.assets/subscriber2.wasm --issuer ../wascc-host/examples/.assets/act.nk --subject ../wascc-host/examples/.assets/mod_sub2.nk -g --name "Subscriber Demo"

# K/V Counter 
cd kvcounter
cargo build --release
cd ..
wascap sign kvcounter/target/wasm32-unknown-unknown/release/kvcounter.wasm ../wascc-host/examples/.assets/kvcounter.wasm --issuer ../wascc-host/examples/.assets/act.nk --subject ../wascc-host/examples/.assets/mod_kvcounter.nk -s -k --name "Key Value Counter"

# WASI
cd wasi-provider
make release
cp ./target/wasm32-wasi/release/wasi_provider_signed.wasm ../../wascc-host/examples/.assets/wasi_provider.wasm
cd ..
cd wasi-consumer
make release
cp ./target/wasm32-unknown-unknown/release/wasi_consumer_signed.wasm ../../wascc-host/examples/.assets/wasi_consumer.wasm
cd ..
