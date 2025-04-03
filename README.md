> [!IMPORTANT]
> このソフトウェアは https://github.com/lpil/sonic-pi-tool を fork し、改変したものです。
> 元のソフトウェアのライセンスは MPL 2.0 のため、改変したソフトウェアも同じライセンスになります。
> 独自に書いた部分もありますが、面倒なので、全体を MPL 2.0 として公開します。

Sonic Pi Tool
=============

`sonic-pi-tool` is a handy command line utility for playing with the Sonic Pi
server. It can be used instead of the Sonic Pi GUI for all your music making
needs :)

It's ideal for use with [sonicpi.vim][sonicpi.vim].

[sonicpi.vim]: https://github.com/dermusikman/sonicpi.vim


## Installation

### From source

If you have the [Rust programming language][rust-install] installed 
sonic-pi-tool can be installed like so:

```sh
cargo install --git https://github.com/lpil/sonic-pi-tool/ --force
```

Note that while Rust needs to be installed to compile sonic-pi-tool, it is not 
needed to run it. You may uninstall Rust or copy the sonic-pi-tool binary from
`~/.cargo/bin` to another computer with the same processor architecture and operating 
system if you wish.

[rust-install]: https://www.rust-lang.org/en-US/install.html

`sonic-pi-tool` may not build on older versions of Rust. Please see [the CI
configuration](.travis.yml) for which versions are explicitly supported.

### Homebrew

Sonic Pi Tool can be installed like so using the Homebrew package manager for OSX:

```sh
$ brew tap Cj-bc/sonic-pi-tool
$ brew install sonic-pi-tool
# or
$ brew install Cj-bc/sonic-pi-tool/sonic-pi-tool --HEAD
```

Homebrew formula is here: [Cj-bc/homebrew-sonic-pi-tool](https://github.com/Cj-bc/homebrew-sonic-pi-tool)


## Usage

- [check](#check)
- [eval](#eval)
- [eval-file](#eval-file)
- [eval-stdin](#eval-stdin)
- [stop](#stop)
- [logs](#logs)
- [start-server](#start-server)
- [record](#record)

### `check`

```sh
sonic-pi-tool check
# => Sonic Pi server listening on port 4560
```

Used to check if the Sonic Pi server is running. If the server isn't running
many of the tool's commands (such as `eval`) will not work.

This command returns a non-zero exit code if the server is not running.


### `eval`

```sh
sonic-pi-tool eval "play :C4"
# *ding*
```

Take a string Sonic Pi code and send it to the Sonic Pi server to be
played.


### `eval-file`

```sh
sonic-pi-tool eval-file path/to/code.rb
# *music*
```

Read Sonic Pi code from a file and send it to the Sonic Pi server to be
played.


### `eval-stdin`

```sh
echo "play :C4" | sonic-pi-tool eval-stdin
# *ding*
```

Read Sonic Pi code from standard in and send it to the Sonic Pi server to be
played.


### `stop`

Stop all jobs running on the Sonic Pi server, stopping the music.

```sh
sonic-pi-tool stop
# *silence*
```


### `logs`

Prints out log messages emitted by the Sonic Pi server.

This command won't succeed if the Sonic Pi GUI is running as it will be
consuming the logs already.

```sh
sonic-pi-tool logs
#
# [Run 2, Time 32.7]
#  └ synth :beep, {note: 65.0, release: 0.1, amp: 0.9741}
#
# [Run 2, Time 32.8]
#  └ synth :beep, {note: 39.0, release: 0.1, amp: 0.9727}
```


### `start-server`

Attempts start the Sonic Pi server, if the executable can be found.

Not supported on Windows.

```sh
sonic-pi-tool start-server
# Sonic Pi server booting...
# Using protocol: udp
# Detecting port numbers...
# ...
```

### `record`

Record the audio output of a Sonic Pi session to a local file.
Stop and save the recording when the Enter key is pressed.

```sh
sonic-pi-tool record /tmp/output.wav
# Recording started, saving to /tmp/output.wav.
# Press Enter to stop the recording...
```

## Other tools

In addition to `sonic-pi-tool` this project contains `sonic-pi-pipe` and
`sonic-pi-logs`. These two programs came first and are written in Go rather
than Rust.

Everything they can do can be done with the newer `sonic-pi-tool`, and as a
result they are no longer actively maintained. :)

Read more about these tools [here][old].

[old]: https://github.com/lpil/sonic-pi-tool/tree/master/old


## MPL 2.0 Licence

## References

- https://github.com/gkvoelkl/python-sonic
  - for Sonic Pi v4
