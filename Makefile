build: sonic-pi-pipe
	@echo All done!

sonic-pi-pipe:
	go build -o build/sonic-pi-pipe src/sonic_pi_pipe.go

test:
	go test ./test/...

clean:
	rm -r build

clean-deps:
	rm -r vendor

.PHONY: clean clean-deps test build
