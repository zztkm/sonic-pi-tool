Sonic Pi Tools
==============

## sonic-pi-pipe

`sonic-pi-pipe` allows you to control Sonic Pi from the command line.

```sh
# Install it
go get github.com/lpil/sonic-pi-tools/cmd/sonic-pi-pipe

sonic-pi-pipe check             # Check the Sonic Pi server is up
echo "play 64" | sonic-pi-pipe  # Send code to Sonic Pi
sonic-pi-pipe stop              # Stop any running jobs
```

It's ideal for use with [sonicpi.vim][sonicpi.vim].
[sonicpi.vim]: https://github.com/dermusikman/sonicpi.vim


## sonic-pi-logs

`sonic-pi-logs` gives you a way of viewing Sonic Pi logs from the command
line.

```sh
# Install it
go get github.com/lpil/sonic-pi-tools/cmd/sonic-pi-logs

# Run it
sonic-pi-logs
```

It works by listening on the same port that the Sonic Pi GUI uses, so it
cannot be used while the GUI is running. Instead it is intended to be used
at the same time as the Sonic Pi server alone.


## Developer instructions

```sh
# Install go and the glide package manager.
brew install go glide

# Install the deps
glide install

make test       # Run the tests
make build      # Build the binaries
make clean      # Clean the build dir
make clean-deps # Clean the vendor dir
```
