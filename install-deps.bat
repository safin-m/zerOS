@echo off
cargo-make --version >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo cargo-make could not be found, installing...
    cargo install cargo-make --version 0.37.16
) else (
    echo cargo-make is already installed
)
cargo install cargo-binutils
rustup component add llvm-tools-preview