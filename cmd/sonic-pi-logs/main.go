package main

import (
	"fmt"
	"github.com/hypebeast/go-osc/osc"
	"github.com/lpil/sonic-pi-tools/log"
)

func main() {
	server := &osc.Server{Addr: "localhost:4558"}

	server.Handle("/info", func(msg *osc.Message) {
		fmt.Print(log.Info(msg.Arguments))
	})
	server.Handle("/multi_message", func(msg *osc.Message) {
		fmt.Print(log.MultiMessage(msg.Arguments))
	})
	server.Handle("/syntax_error", func(msg *osc.Message) {
		fmt.Print(log.SyntaxError(msg.Arguments))
	})
	server.Handle("/error", func(msg *osc.Message) {
		fmt.Print(log.Error(msg.Arguments))
	})

	// server.Handle("*", func(msg *osc.Message) {
	// 	fmt.Println(msg)
	// })

	server.ListenAndServe()
}
