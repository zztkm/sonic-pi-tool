build: sonic-pi-pipe sonic-pi-logs
	@echo All done!

sonic-pi-pipe:
	go build -o build/sonic-pi-pipe cmd/sonic-pi-pipe/main.go

sonic-pi-logs:
	go build -o build/sonic-pi-logs cmd/sonic-pi-logs/main.go

test:
	go test ./test/...

clean:
	rm -r build

clean-deps:
	rm -r vendor

.PHONY: clean clean-deps test build
