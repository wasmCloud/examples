package main

import (
	"strconv"

	"github.com/wasmcloud/actor-tinygo"
	keyvalue "github.com/wasmcloud/interfaces/keyvalue/tinygo"
)

func main() {
	me := AsdbProcessor{}
	actor.RegisterHandlers(AdsbHandler(&me))
}

type AsdbProcessor struct{}

func (e *AsdbProcessor) HandleAdsbMsg(ctx *actor.Context, arg AdsbMsg) error {
	stationLat := strconv.FormatFloat(arg.Station.Latitude, 'g', -1, 64)
	stationLong := strconv.FormatFloat(arg.Station.Longitude, 'g', -1, 64)
	lat := strconv.FormatFloat(arg.Latitude, 'g', -1, 64)
	long := strconv.FormatFloat(arg.Longitude, 'g', -1, 64)
	alt := strconv.FormatInt(int64(arg.Altitude), 10)
	icao := strconv.FormatUint(arg.Icao, 10)

	geojson_contact := "{\"type\":\"Feature\",\"properties\":{\"type\":\"contact\",\"icao\":" + icao + ",\"altitude\":" + alt + ",\"callsign\":\"" + arg.CallSign + "\",\"squawk\":\"" + arg.Squawk + "\"},\"geometry\":{\"type\":\"Point\",\"coordinates\":[" + lat + "," + long + "]}}"
	geojson_station := "{\"type\":\"Feature\",\"properties\":{\"type\":\"station\",\"id\":\"" + arg.Station.Id + "\",\"name\":\"" + arg.Station.Name + "\"},\"geometry\":{\"type\":\"Point\",\"coordinates\":[" + stationLat + "," + stationLong + "]}}"

	// station := "{\"latitude\":" + stationLat + ",\"longitude\":" + stationLong + "}"
	// value := "{\"callsign\":\"" + arg.CallSign + "\",\"latitude\":" + lat + ",\"longitude\":" + long + ",\"altitude\":" + alt + ",\"squawk\":\"" + arg.Squawk + "\"}"

	sender := keyvalue.NewProviderKeyValue()

	lAS := keyvalue.SetAddRequest{
		SetName: "stations",
		Value:   arg.Station.Id,
	}

	lAC := keyvalue.SetAddRequest{
		SetName: "contacts",
		Value:   icao,
	}

	setStation := keyvalue.SetRequest{
		Key:     arg.Station.Id,
		Value:   geojson_station,
		Expires: 60 * 10, // expires after 10 minutes
	}

	setContact := keyvalue.SetRequest{
		Key:     icao,
		Value:   geojson_contact,
		Expires: 60 * 10, // expires after 10 minutes
	}

	err := sender.Set(ctx, setStation)
	if err != nil {
		return err
	}

	err = sender.Set(ctx, setContact)
	if err != nil {
		return err
	}

	_, err = sender.SetAdd(ctx, lAS)
	if err != nil {
		return err
	}

	_, err = sender.SetAdd(ctx, lAC)
	if err != nil {
		return err
	}

	return nil
}
