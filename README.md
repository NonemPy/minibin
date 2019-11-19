# minibin

A minimal pastebin.

**minibin** is a slightly-tweaked fork of [bin](https://github.com/w4/bin) by [Jordan Doyle](https://github.com/w4) with syntax highlighting removed to make it *even more* minimal.

## Features

1. In-memory, no database.
2. Create and view pastes from the browser or the command line.

## Install

### Quickstart

Install `docker` and `docker-compose`, then adapt `docker-compose.yml` to your liking and:

```sh
$ docker-compose build
$ docker-compose up -d
```

Should be accessible at <http://127.0.0.1:8000>.

### Build manually

Install [rustup](https://rustup.rs/), then:

```sh
$ git clone https://github.com/tylerlm/minibin.git
$ cd minibin
$ cargo +nightly build --release
   Compiling minibin v0.0.1 (/home/user/code/minibin)
    Finished release [optimized] target(s) in 5.55s
```

Find the resulting binary at `./target/release/minibin` -- just run it.

## Settings

**minibin** uses [rocket](https://rocket.rs) so you can add a [rocket config file](https://api.rocket.rs/v0.3/rocket/config/) if you like.

### Environment variables

- `ROCKET_PORT`: change the default port (default: 8000)
- `PASTEBIN_MAX_PASTES`: how many pastes to keep (default: 1000)

## `curl` examples

```sh
$ curl -X PUT --data 'example text' https://bin.example.com
https://bin.example.com/abcdefghij

$ echo 'example file' > file.txt
$ curl -X PUT --data-binary @file.txt https://bin.example.com
https://bin.example.com/klmnopqrst

$ echo 'example stdin' | curl -X PUT --data-binary @- https://bin.example.com
https://bin.example.com/uvwxyzabcd

$ curl https://bin.example.com/abcdefghij
example text
$ curl https://bin.example.com/klmnopqrst
example file
$ curl https://bin.example.com/uvwxyzabcd
example stdin
```
