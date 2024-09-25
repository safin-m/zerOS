#!/bin/sh
if ! command -v cargo-make &> /dev/null
then
    echo "cargo-make could not be found, installing..."
    cargo install cargo-make --version 0.37.16
else
    echo "cargo-make is already installed"
fi