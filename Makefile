build: sonic-pi-send
	@echo All done!

sonic-pi-send:
	go build -o build/sonic-pi-send src/sonic_pi_send.go

test:
	go test ./test/...

.PHONY: clean test build
