package main

import (
	"fmt"
	"github.com/hypebeast/go-osc/osc"
	"io/ioutil"
	"net"
	"os"
	"strconv"
)

const port = 4557
const usage = `sonic-pi-pipe commands:
	[No args]: Send piped code to the Sonic Pi server
	check:     Check if the Sonic Pi server is listening
	stop:      Stop any running Sonic Pi jobs
`

func main() {
	args := os.Args[1:]

	if len(args) == 0 {
		pipeToSonicPi()
		os.Exit(0)
	}

	switch args[0] {
	case "check":
		checkSonicPiListening()
	case "stop":
		sendSonicPiStop()
	default:
		fmt.Print(usage)
		os.Exit(1)
	}
}

func pipeToSonicPi() {
	data, _ := ioutil.ReadAll(os.Stdin)
	code := string(data)
	message := osc.NewMessage("/run-code")
	message.Append("SONIC_PI_PIPE")
	message.Append(code)
	osc.NewClient("localhost", port).Send(message)
}

func sendSonicPiStop() {
	message := osc.NewMessage("/stop-all-jobs")
	message.Append("SONIC_PI_PIPE")
	osc.NewClient("localhost", port).Send(message)
}

// Send gui-heartbeat message?
func checkSonicPiListening() {
	p := strconv.Itoa(port)
	address, _ := net.ResolveUDPAddr("udp", ":"+p)
	_, err := net.ListenUDP("udp", address)
	if err == nil {
		fmt.Println("Error: Sonic Pi not listening on " + p)
		os.Exit(1)
	} else {
		fmt.Println("Sonic Pi listening on " + p)
		os.Exit(0)
	}
}
