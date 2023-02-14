# WASM-AIR
A FlightAware clone that leverages [dump1090](https://github.com/antirez/dump1090)

> This is an orginal wasmCloud example that dates back to the beginning and updated to work in a >.60 world. Read a lot about the project [here](https://github.com/wasmcloud/wasm-air).

## Architecture 

### dump1090 Provider 

### Processor Actor 

### API Gateway Actor 

### UI Actor (Vue3 App)

### ADSB Interface

## Build Instructions 

> You will need [dump1090](https://github.com/wasmcloud/wasm-air) and [wash](https://github.com/wasmcloud/wash) installed.  

#### Build the interface

From the root of the repository, run 

```
wash gen
```

this will drop generated files in the provider and processer repositories.  Without running this generation, the rest of the components will fail to build.

#### Build the provider 

###### Build the Go binary
```
cd adsb-provider
```

We have included a [goreleaser]() configuration file to make this step easy.  If you have cli installed, simple run 

```
goreleaser build --clean --snapshot
```

This will build a provider archive with the 5 most common architecture/os combinations.

To manually create the provider archive, first run 

```
go build -o build/adsb
```
###### Create the Provider archive

```
wash par create --arch x86_64-macos \ # make sure to replace with your arch here
  --binary build/adsb \
  --capid myorg:adsb \
  --name "ADSB - dump1090" \
  --vendor "myorg" \
  --version 0.1.0 \
  --revision 1 \
  --destination build/adsb.par.gz \
  --compress
```

This will create an artifact, `build/adsb.par.gz` that can be deployed into wasmCloud.

If you have additional os/architecture pairs you would like to add (you will need to cross-compile the provider for every architucture you'd like it to run), 
then you can add them with `wash par insert` one at a time.

#### Build the processor actor

From inside the `adsb-processor` directory, you will build the actor with `wash`

```
wash build
```

#### Build the API Gateway actor

From inside the `adsb-api` directory, you will build the actor with `wash`

```
wash build
```

#### Build the UI actor

From inside the `adsb-ui` directory, you will build the actor with `wash`

```
wash build
```

