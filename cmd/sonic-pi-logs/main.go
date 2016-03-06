package main

import (
	"fmt"
	"github.com/hypebeast/go-osc/osc"
)

func main() {
	server := &osc.Server{Addr: "localhost:4558"}

	server.Handle("/info", handle_info)
	server.Handle("/multi_message", handle_multi_message)
	// server.Handle("*", handle)

	server.ListenAndServe()
}

func handle_info(msg *osc.Message) {
	fmt.Println("=>", msg.Arguments[0])
}

func handle_multi_message(msg *osc.Message) {
	run := msg.Arguments[0]
	time := msg.Arguments[2]
	info := msg.Arguments[5]
	fmt.Printf("\n[Run %d, Time %s]\n", run, time)
	fmt.Printf(" └ %s\n", info)
}

func handle(msg *osc.Message) {
	fmt.Println(msg)
}
