# roku_cli

A CLI for controlling your Roku TV.

## Usage
```shell
# Creates a binary at $HOME/.cargo/bin/roku_cli
cargo install --path .

roku_cli
```

[External Control Protocol](https://developer.roku.com/docs/developer-program/debugging/external-control-api.md)
(ECP) enables a Roku device to be controlled over a local area network by
providing a number of external control services. The Roku devices offering these
external control services are discoverable using SSDP (Simple Service Discovery
Protocol). ECP is a simple RESTful API that can be accessed by programs in
virtually any programming environment.