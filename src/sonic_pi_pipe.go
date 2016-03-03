package main

import (
	"github.com/hypebeast/go-osc/osc"
	"io/ioutil"
	"os"
)

const RUN_ADDRESS = "/run-code"
const STOP_ADDRESS = "/stop-all-jobs"
const SERVER = "localhost"
const PORT = 4557

func main() {
	data, _ := ioutil.ReadAll(os.Stdin)
	pi_code := string(data)
	message := osc.NewMessage(RUN_ADDRESS)
	message.Append(pi_code)
	osc.NewClient(SERVER, PORT).Send(message)
}
