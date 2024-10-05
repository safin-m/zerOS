#!/bin/sh


command_exists() {
    command -v "$1" >/dev/null 2>&1
}


if ! command_exists rustup; then
    echo "Rust could not be found, installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
else
    echo "Rust is already installed on your system"
fi


if rustup show active-toolchain | grep -q "nightly"; then
    echo "Rust is already set to nightly"
else
    echo "Setting Rust to nightly..."
    rustup override set nightly
fi


rustup component add rust-src


if ! command_exists cargo-make; then
    echo "cargo-make could not be found, installing..."
    cargo install cargo-make --version 0.37.16
else
    echo "cargo-make is already installed"
fi


if ! rustup component list --installed | grep -q "llvm-tools"; then
    echo "llvm-tools-preview component could not be found, adding..."
    rustup component add llvm-tools-preview
else
    echo "llvm-tools-preview component is already added"
fi


if ! command_exists nasm; then
    echo "nasm could not be found, installing..."
    brew install nasm
else
    echo "nasm is already installed"
fi


if ! command_exists x86_64-elf-gcc; then
    echo "Installing x86_64-elf-gcc..."
    brew install x86_64-elf-gcc
else
    echo "x86_64-elf-gcc is already installed."
fi


if ! command_exists x86_64-elf-ld; then
    echo "Installing x86_64-elf-binutils..."
    brew install x86_64-elf-binutils
else
    echo "x86_64-elf-binutils is already installed."
fi

if ! command_exists cargo bootimage; then
    echo "Adding bootimage..."
    cargo install bootimage
else 
    echo "Bootimage is already installed."
fi

if ! command_exists qemu; then
    echo "Installing qemu"
    cargo install qemu
else 
    echo "Qemu is already installed."
fi

brew tap nativeos/i386-elf-toolchain
brew install nativeos/i386-elf-toolchain/i386-elf-binutils
brew install nativeos/i386-elf-toolchain/i386-elf-gcc
brew install i386-elf-grub
brew install xorriso

