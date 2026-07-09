#!/bin/bash

echo "Running post-installation tasks..."

# Generowanie dokumentacji
echo "Generating documentation..."
cargo doc --no-deps

# Konfiguracja projektu
echo "Configuring project..."
if [ ! -f "config.toml" ]; then
    cp examples/pag.conf config.toml
    echo "Created default configuration file: config.toml"
fi

echo "Post-installation tasks completed."