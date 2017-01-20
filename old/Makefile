build: build/sonic-pi-pipe build/sonic-pi-logs
	@echo All done!

build/sonic-pi-pipe:
	go build -o build/sonic-pi-pipe cmd/sonic-pi-pipe/main.go

build/sonic-pi-logs:
	go build -o build/sonic-pi-logs cmd/sonic-pi-logs/main.go

clean:
	rm -rf build

.PHONY: clean build
