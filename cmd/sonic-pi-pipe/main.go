package main

import (
	"github.com/hypebeast/go-osc/osc"
	"io/ioutil"
	"strconv"
	"os"
	"fmt"
	"net"
)

const port = 4557
const usage = `sonic-pi-pipe commands:
	[No arguments]: Send piped code to the Sonic Pi server
	check:          Check if the Sonic Pi server is listening
`

func main() {
	args := os.Args[1:]

	if len(args) == 0 {
		pipe_to_sonic_pi()
		os.Exit(0)
	}

	switch args[0] {
	case "check":
		check_server_listening()
	default:
		fmt.Print(usage)
		os.Exit(1)
	}
}

func pipe_to_sonic_pi() {
	data, _ := ioutil.ReadAll(os.Stdin)
	pi_code := string(data)
	message := osc.NewMessage("/run-code")
	message.Append("SONIC_PI_PIPE")
	message.Append(pi_code)
	osc.NewClient("localhost", port).Send(message)
}

func check_server_listening() {
	p := strconv.Itoa(port)
	address, _ := net.ResolveUDPAddr("udp", ":" + p)
	_, err := net.ListenUDP("udp", address)
	if err == nil {
		fmt.Println("Error: Sonic Pi not listening on " + p)
		os.Exit(1)
	} else {
		fmt.Println("Sonic Pi listening on " + p)
		os.Exit(0)
	}
}
