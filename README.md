Sonic Pi Tool
=============

`sonic-pi-tool` is a handy command line utility for playing with the Sonic Pi
server. It can be used instead of the Sonic Pi GUI for all your music making
needs :)

## Usage

```sh
sonic-pi-tool help                         # Print the help
sonic-pi-tool eval-file path/to/music.rb   # Run a file of Sonic Pi code
echo "play :c4" | sonic-pi-tool eval-stdin # Run Sonic Pi code from stdin
sonic-pi-tool stop                         # Stop currently playing music
```


## Other tools

In addition to `sonic-pi-tool` there are two other programs in this project,
detailed below. Soon they will be deprecated and replaced by `sonic-pi-tool`.

### sonic-pi-pipe

`sonic-pi-pipe` allows you to control Sonic Pi from the command line.

```sh
# Install it
go get github.com/lpil/sonic-pi-tools/old/cmd/sonic-pi-pipe

sonic-pi-pipe check             # Check the Sonic Pi server is up
echo "play 64" | sonic-pi-pipe  # Send code to Sonic Pi
sonic-pi-pipe stop              # Stop any running jobs
```

It's ideal for use with [sonicpi.vim][sonicpi.vim].
[sonicpi.vim]: https://github.com/dermusikman/sonicpi.vim


### sonic-pi-logs

`sonic-pi-logs` gives you a way of viewing Sonic Pi logs from the command
line.

```sh
# Install it
go get github.com/lpil/sonic-pi-tools/old/cmd/sonic-pi-logs

# Run it
sonic-pi-logs
```

It works by listening on the same port that the Sonic Pi GUI uses, so it
cannot be used while the GUI is running. Instead it is intended to be used
at the same time as the Sonic Pi server alone.

*Note:* You can run the Sonic Pi server without GUI by executing the *SONIC_PI_PATH/app/server/bin/sonic-pi-server.rb* script.
