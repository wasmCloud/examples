# WASM-AIR
A FlightAware clone that leverages [dump1090](https://github.com/antirez/dump1090)

> This is an orginal wasmCloud example that dates back to the beginning and updated to work in a >.60 world. Read a lot about the project [here](https://github.com/wasmcloud/wasm-air).

## Architecture 

### dump1090 Provider 

This provider creates a telnet session to a local instance of dump1090 and reads the data in a stream.  In order for this provider to operate correctly, you much first start dump1090 with its net flag -> `dump1090 --net`

In order to configure the provider, the following link variables are checked.

| Variable          | Type   | Default        | Description                                                                                 | Required |
| --------          | ----   | -------        | -----------                                                                                 | -------- |
| station_latitude  | string | -              | The latitude of where the RTL-SDR is located. This is used to calculate aircraft position.  | yes      |
| station_longitude | string | -              | The longitude of where the RTL-SDR is located. This is used to calculate aircraft position. | yes      |
| station_name      | string | Generated UUID | Simple name for displaying on map UI.                                                       | no       |
| dump1090_ip       | string | -              | Address of dump1090 telnet server                                                           | yes      |
| dump1090_port     | int    | 30002          | Port of dump1090 telnet server                                                              | no       |

Once a link is made to the Processor Actor, contacts will start being processed.

### Processor Actor 

This is a simple actor that takes `Adsb.HandleAdsbMsg` messages from a dump1090 provider, creates JSON data types, and stores it in a keyvalue store.  Services like the API Gateway and UI use the keyvalue store to access the data.

Each contact and station is stored with its ICAO number and UUID, respectively.  All keys are stored with a 10minute TTL to ensure stall contacts eventually fall off.

### API Gateway Actor 

The API Gateway actor is a way to access the ADSB data via http queries.  

| Endpoint  | Method | Description                                                                                             |
| --------  | ------ | -----------                                                                                             |
| /contacts | GET    | Returns the air contacts as identified by all stations                                                  |
| /stations | GET    | Returns a list of stations and their location                                                           |
| /geojson  | GET    | Returns a combination contacts and stations in [geojson](https://www.rfc-editor.org/rfc/rfc7946) format |

Any other query to the API should return a 404 - `{"error":"invalid_request"}`

### UI Actor (Vue3 App)

The UI actor embeds a Vue3 webapp that is used to plot contacts and stations on a map for a visual.  It utilizes the API Gateway as its datasource.

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

