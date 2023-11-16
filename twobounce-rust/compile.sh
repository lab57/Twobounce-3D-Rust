#!/bin/bash

# Name of your executable
EXECUTABLE_NAME="rust"

# Destination path where you want to copy the executable
DESTINATION_PATH="../twobounce"

# Build the project using Cargo
# Uncomment the appropriate line depending on whether you want a debug or release build
# cargo build --release
cargo build --release 

# Check if build succeeded
if [ $? -eq 0 ]; then
    echo "Build successful."

    # Copy the executable to the destination path
    # Uncomment the appropriate line depending on your build type
    # cp target/release/$EXECUTABLE_NAME $DESTINATION_PATH
    cp target/release/$EXECUTABLE_NAME $DESTINATION_PATH

    # Check if copy succeeded
    if [ $? -eq 0 ]; then
        echo "Executable copied to parent directory"
    else
        echo "Failed to copy the executable."
    fi
else
    echo "Build failed."
fi
