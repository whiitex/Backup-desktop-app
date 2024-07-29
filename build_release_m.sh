#!/bin/bash

rm -rf "release/macos"

# Build the project with cargo in release mode for the entire workspace
cargo build --release --workspace

# Create the release/macos directory if it does not exist
if [ ! -d "release/macos" ]; then
    mkdir -p release/macos
fi

# Copy assets to the release/macos/assets directory
if [ -d "target/release/assets" ]; then
    cp -r target/release/assets release/macos/assets
fi

# Copy the executables to the release/macos directory
if [ -f "target/release/Group13" ]; then
    cp target/release/Group13 release/macos/Group13
fi

if [ -f "target/release/spawn_gui" ]; then
    cp target/release/spawn_gui release/macos/spawn_gui
fi

if [ -f "target/release/spawn_popup" ]; then
    cp target/release/spawn_popup release/macos/spawn_popup
fi

echo "Build and copy process completed successfully."
