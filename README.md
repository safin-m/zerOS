# zerOS

Minimal operating system written in Rust

| **Getting Started** |

## Install Rust

Install Rust via rustup with this command:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Or follow the instructions here [Rust Installation Guide](https://www.rust-lang.org/tools/install)

### Unix-based Systems

1. Make the `install-deps.sh` script executable:

   ```sh
   chmod +x install-deps.sh
   ```

2. Run the `install-deps.sh` script to install dependencies:

   ```sh
   ./install-deps.sh
   ```

### Windows

1. Run the `install-deps.bat` script to install dependencies:

   ```cmd
   install-deps.bat
   ```

### Fetch Rust Dependencies

Run the following command to fetch the Rust dependencies:

```sh
cargo fetch
```

### Build Boot Binary

Run the following command to build the bootloader binary:

```sh
cargo make build_boot
```
