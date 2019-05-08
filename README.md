# fortune

`fortune` is a program that displays a pseudorandom message from a database of quotations that first appeared in Version 7 Unix.[1][irrelevant citation] The most common version on modern systems is the BSD fortune, originally written by Ken Arnold.[2] Distributions of fortune are usually bundled with a collection of themed files, containing sayings like those found on fortune cookies (hence the name), quotations from famous people, jokes, or poetry.

You can install fortune with:

```shell
wapm install -g fortune
```

## Running

```bash
$ fortune
Opportunity knocks, but doesn't always answer to its name.
    -- Mason Cooley
```

Print help

```shell
$ fortune -h
available commands:
	-h                        this screen right here.
	-o <short,medium,long>    output short,medium or long quotes only.
```

## Building from Source

First, you will need the WASI target installed in your Rust system:

```shell
rustup target add wasm32-unknown-wasi --toolchain nightly
```

Once WASI is available, you can build the WebAssembly binary by yourself with:

```shell
cargo +nightly build --release --target wasm32-unknown-wasi
```

This will create a new file located at `target/wasm32-unknown-wasi/release/fortune.wasm`.

When the wasm file is created you can upload it to wapm or execute it with wasmer:

```shell
wapm publish
# OR
wasmer run  target/wasm32-unknown-wasi/release/fortune.wasm
```

You can also build a native executable with

```shell
cargo build
```
