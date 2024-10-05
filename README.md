# zerOS

Minimal operating system written in Rust

## Set up the environment

To set up the environment run the following commands:

```sh
   chmod +x ./install-deps.sh
```

```sh
   ./install-deps.sh
```

## Build and Run

### Build using rust-based bootloader

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

### Build using custom bootloader

1. Add the following lines to your ./Cargo.toml

   ```text
      [lib]
      path = "src/main.rs"
      crate-type = ["staticlib"]
   ```

2. Generate ISO using the following command

   ```sh
      cargo make generate_iso
   ```

3. Run:

   ```sh
      cargo make run
   ```

For debugging you could run

```sh
   cargo make dbg
```

and then in another terminal (make sure lldb is installed)

```sh
   lldb
   gdb-remote 1234
```
