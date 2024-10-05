# zerOS

Minimal operating system written in Rust

## Set up the environment

To set up the environment and to build your own bootloader or use the bootimage tool run the following commands

```sh
   chmod +x ./install-deps.sh
```

```sh
   ./install-deps.sh
```

## Build and Run

The runner is set up to compile, build and run the OS

To build run the following commands:

```sh
   cargo bootimage
```

and then,

```sh
   qemu-system-x86_64 -drive format=raw,file=target/x86_64-unknown-none/debug/bootimage-zer_os.bin
```

To build and run:

```sh
   cargo run
```
