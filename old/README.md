### sonic-pi-pipe

`sonic-pi-pipe` allows you to control Sonic Pi from the command line.

```sh
# Install it
go get github.com/lpil/sonic-pi-tools/old/cmd/sonic-pi-pipe

sonic-pi-pipe check             # Check the Sonic Pi server is up
echo "play 64" | sonic-pi-pipe  # Send code to Sonic Pi
sonic-pi-pipe stop              # Stop any running jobs
```



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
