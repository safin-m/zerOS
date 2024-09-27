#!/bin/sh


command_exists() {
    command -v "$1" >/dev/null 2>&1
}


if ! command_exists cargo-make; then
    echo "cargo-make could not be found, installing..."
    cargo install cargo-make --version 0.37.16
else
    echo "cargo-make is already installed"
fi


if ! command_exists cargo-binutils; then
    echo "cargo-binutils could not be found, installing..."
    cargo install cargo-binutils
else
    echo "cargo-binutils is already installed"
fi


if ! rustup component list | grep "llvm-tools-preview.*(installed)" >/dev/null 2>&1; then
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


brew tap nativeos/i386-elf-toolchain


if ! command_exists i386-elf-binutils; then
    echo "Installing i386-elf-binutils..."
    brew install nativeos/i386-elf-toolchain/i386-elf-binutils
else
    echo "i386-elf-binutils is already installed."
fi


if ! command_exists i386-elf-gcc; then
    echo "Installing i386-elf-gcc..."
    brew install nativeos/i386-elf-toolchain/i386-elf-gcc
else
    echo "i386-elf-gcc is already installed."
fi