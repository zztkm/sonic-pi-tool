package main

import (
	"github.com/hypebeast/go-osc/osc"
	"io/ioutil"
	"os"
)

// const RUN_ADDRESS = "/run-code"
// const STOP_ADDRESS = "/stop-all-jobs"

func main() {
	data, _ := ioutil.ReadAll(os.Stdin)
	pi_code := string(data)
	message := osc.NewMessage("/run-code")
	message.Append("SONIC_PI_SEND")
	message.Append(pi_code)
	osc.NewClient("localhost", 4557).Send(message)
}
