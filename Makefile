build: sonic-pi-send
	@echo All done!

sonic-pi-send:
	go build -o build/sonic-pi-send src/sonic_pi_send.go

test:
	go test ./test/...

clean:
	rm -r build

clean-deps:
	rm -r vendor

.PHONY: clean clean-deps test build
