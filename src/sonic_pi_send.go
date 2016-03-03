package main

import (
	"fmt"
	"github.com/hypebeast/go-osc/osc"
)

func main() {
	client := osc.NewClient("localhost", 8765)
	msg := osc.NewMessage("/osc/address")
	msg.Append(int32(111))
	msg.Append(true)
	msg.Append("hello")
	client.Send(msg)
	fmt.Println("hello world")
}
