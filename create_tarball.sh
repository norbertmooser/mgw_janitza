#!/bin/bash

# Define the project name and tarball name
PROJECT_NAME="mgw_janitza"
TARBALL_NAME="$PROJECT_NAME.tar.gz"

# Create a temporary directory for packaging
TMP_DIR=$(mktemp -d)

# Copy necessary files and directories to the temporary directory
cp -r Cargo.lock Cargo.toml meter_config.json README.md src $TMP_DIR/

# Change to the temporary directory
cd $TMP_DIR

# Create the tarball
tar -czf $TARBALL_NAME *

# Move the tarball to the original directory
mv $TARBALL_NAME $OLDPWD/

# Clean up the temporary directory
cd $OLDPWD
rm -rf $TMP_DIR

echo "Tarball $TARBALL_NAME created successfully."
