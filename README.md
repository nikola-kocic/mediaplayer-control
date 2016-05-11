# mediaplayer-control
Controls Media Player using DBus protocol

## Installation
Project builds with the Rust stable version, using the Cargo build system.

`cargo build --release`

Resulting binary is at `./target/release/mediaplayer-control`

## Usage
```
USAGE:
    mediaplayer-control [FLAGS] [OPTIONS] --player <player> --command <command>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --arg <arg>            Sets the command argument
    -c, --command <command>    Sets the command to execute: Next, Previous, Pause, PlayPause, Stop, Play, Seek(nanoseconds), OffsetVolume(0.0 - 1.0)
    -p, --player <player>      Sets the player to use

Examples:
    mediaplayer-control --player audacious --command Next
    mediaplayer-control --player audacious --command Seek --arg="-5000000"
    mediaplayer-control -p audacious -c OffsetVolume -a="-0.025"
```
