# zerOS

Minimal operating system written in Rust

| **Getting Started** |

## Build Process

### Set up the environment

1. Make the `install-deps.sh` script executable:

   ```sh
   chmod +x install-deps.sh
   ```

2. Run the `install-deps.sh` script to install dependencies and build tools:

   ```sh
   ./install-deps.sh
   ```

## Build Binary-ELF OS

Run the following command to build the bootable bootloader binary and kernel:

```sh
cargo make start
```
