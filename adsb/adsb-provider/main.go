package main

import (
	"bufio"
	"encoding/hex"
	"errors"
	"fmt"
	"strconv"
	"strings"

	"github.com/google/uuid"
	"github.com/reiver/go-telnet"
	"github.com/wasmCloud/provider-sdk-go"
	core "github.com/wasmcloud/interfaces/core/tinygo"
	msgpack "github.com/wasmcloud/tinygo-msgpack"
	"kreklow.us/go/go-adsb/adsb"
)

const adsbActor = "MCAUGMQU2SSHZOSS65BBRFQOAERNCJ5QYLWWV2I2WTRDZ424RAD7CX5V"
const maxCapacity = 112 * 8

var (
	p             *provider.WasmcloudProvider
	conn          *telnet.Conn
	close         bool = false
	closeFinished chan struct{}
)

func main() {
	var err error

	p, err = provider.New(
		"jordanrash:adsb",
		provider.WithNewLinkFunc(handleNewLink),
		provider.WithDelLinkFunc(handleDelLink),
	)
	if err != nil {
		panic(err)
	}

	err = p.Start()
	if err != nil {
		panic(err)
	}
}

func handleDelLink(_ core.LinkDefinition) error {
	defer conn.Close()
	close = true
	<-closeFinished
	p.Logger.Info("connection to local dump1090 server closed")
	return nil
}

func handleNewLink(linkdef core.LinkDefinition) error {
	var err error

	fmt.Println("received new link request from: " + linkdef.ActorId)
	stationLat := linkdef.Values["station_latitude"]
	stationLong := linkdef.Values["station_longitude"]
	stationName := linkdef.Values["station_name"]
	dump1090Ip := linkdef.Values["dump1090_ip"]
	dump1090Port := linkdef.Values["dump1090_port"]
	stationId := uuid.NewString()

	if dump1090Port == "" {
		dump1090Port = "30002"
	}

	if stationLat == "" || stationLong == "" || dump1090Ip == "" {
		return errors.New("invalid link definiation values")
	}

	conn, err = telnet.DialTo(dump1090Ip + ":" + dump1090Port)
	if err != nil {
		return err
	}

	lat, err := strconv.ParseFloat(stationLat, 64)
	if err != nil {
		return err
	}
	long, err := strconv.ParseFloat(stationLong, 64)
	if err != nil {
		return err
	}

	buf := make([]byte, maxCapacity)
	scanner := bufio.NewScanner(conn)
	scanner.Buffer(buf, maxCapacity)
	scanner.Split(bufio.ScanLines)

	for scanner.Scan() {
		msg := scanner.Text()
		msg = strings.TrimSpace(msg)
		msg = strings.TrimPrefix(msg, "*")
		msg = strings.TrimSuffix(msg, ";")

		station := Station{Id: stationId, Name: stationName, Latitude: lat, Longitude: long}
		adsbMsg, err := decodeMsg(msg, station)
		if err != nil {
			// This error fails a lot on non-adsb messages, so make it a debug log instead
			p.Logger.V(1).Info("failed to decode msg", "msg", msg, "error", err.Error())
			continue
		}

		// encode struct for traversing lattice
		var sizer msgpack.Sizer
		size_enc := &sizer
		err = adsbMsg.MEncode(size_enc)
		if err != nil {
			p.Logger.Error(err, "failed to size encode msg", "msg", msg)
			continue
		}

		buf := make([]byte, sizer.Len())
		encoder := msgpack.NewEncoder(buf)
		enc := &encoder
		err = adsbMsg.MEncode(enc)
		if err != nil {
			p.Logger.Error(err, "failed to encode msg", "msg", msg)
			continue
		}

		_, err = p.ToActor(adsbActor, buf, "Adsb.HandleAdsbMsg")
		if err != nil {
			p.Logger.Error(err, "failed to send msg to actor", "msg", msg)
			continue
		}

		if close {
			closeFinished <- struct{}{}
			return nil
		}
	}

	return scanner.Err()
}

func decodeMsg(msgRaw string, station Station) (*AdsbMsg, error) {
	msgHex, err := hex.DecodeString(msgRaw)
	if err != nil {
		return nil, err
	}

	rm := new(adsb.Message)
	err = rm.UnmarshalBinary(msgHex)
	if err != nil {
		return nil, err
	}

	cpr, err := rm.CPR()
	if err != nil {
		return nil, err
	}

	if cpr != nil {
		ret := AdsbMsg{
			Station: &station,
		}

		icao, err := rm.ICAO()
		if err != nil {
			return nil, err
		}
		ret.Icao = icao

		alt, err := rm.Alt()
		if err != nil {
			return nil, err
		}
		ret.Altitude = uint32(alt)

		point, err := cpr.DecodeLocal([]float64{station.Latitude, station.Longitude})
		if err != nil {
			return nil, err
		}
		ret.Latitude = point[0]
		ret.Longitude = point[1]

		callSign, err := rm.Call()
		if err == nil {
			ret.CallSign = callSign
		}

		sqk, err := rm.Sqk()
		if err == nil {
			ret.Squawk = string(sqk)
		}

		return &ret, nil
	}

	return nil, errors.New("failed to decode message")
}
