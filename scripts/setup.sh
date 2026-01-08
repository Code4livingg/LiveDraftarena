#!/bin/bash

echo "Setting up LiveDraft Arena..."

# Check if Xcode command line tools are installed
if ! xcode-select -p &> /dev/null; then
    echo "Installing Xcode command line tools..."
    xcode-select --install
    echo "Please complete the Xcode installation and run this script again."
    exit 1
fi

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source ~/.cargo/env
fi

# Build the project
echo "Building project..."
cargo build

echo "Setup complete!"