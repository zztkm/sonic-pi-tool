package main

import (
	"github.com/hypebeast/go-osc/osc"
	"io/ioutil"
	"os"
)

func main() {
	data, _ := ioutil.ReadAll(os.Stdin)
	pi_code := string(data)
	send(pi_code)
}

func send(code string) {
	message := osc.NewMessage("/run-code")
	message.Append("SONIC_PI_SEND")
	message.Append(code)
	osc.NewClient("localhost", 4557).Send(message)
}
