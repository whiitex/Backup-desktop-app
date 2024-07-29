#!/bin/bash

# Build the project with cargo in release mode for the entire workspace
cargo build --release --workspace

# Create the release/linux directory if it does not exist
if [ ! -d "release/linux" ]; then
    mkdir -p release/linux
fi

# Copy assets to the release/linux/assets directory
cp -r target/release/assets release/linux/assets

# Copy the executables to the release/linux directory
cp target/release/Group13 release/linux/Group13
cp target/release/spawn_gui release/linux/spawn_gui
cp target/release/spawn_popup release/linux/spawn_popup

echo "Build and copy process completed successfully."