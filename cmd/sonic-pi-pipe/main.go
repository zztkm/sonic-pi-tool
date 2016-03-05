package main

import (
	"github.com/hypebeast/go-osc/osc"
	"io/ioutil"
	"os"
	"fmt"
)

const usage = `sonic-pi-pipe commands:
	[No arguments]: Send piped code to the Sonic Pi server
`

func main() {
	args := os.Args[1:]

	if len(args) == 0 {
		pipe_to_sonic_pi()
		os.Exit(0)
	}

	switch args[0] {
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
	osc.NewClient("localhost", 4557).Send(message)
}
