#!/bin/bash

# This script compiles tobi-cli and installs it and it's helper bash function to a given directory

# iterate through the arguments
for arg in "$@"
do
    # check if the argument is --help
    if [ "$arg" == "--help" ] || [ "$arg" == "-h" ]; then
        # print the help message
        echo "Usage: ./install.sh"
        echo "This script compiles tobi and installs it and it's helper bash function to a given directory"
        echo "Options:"
        echo "  --help, -h: Display this help message"
        echo "  --install-dir=<directory>: Install tobi to the specified directory"
        echo "  --rc-file=<file>: Install the helper bash function to the specified file"
        echo "  --purge: Remove tobi from the specified install directory(rc-file restore is not implemented)"
        echo ""
        echo "Example: ./install.sh --install-dir=/opt/scripts --rc-file=$(realpath ~/.zshrc)"
        exit 0
    fi

    # check if the argument starts with --install-dir
    if [[ $arg == --install-dir=* ]]; then
        # extract the directory from the argument
        install_dir=${arg#--install-dir=}

        # check if the directory exists
        if [ ! -d "$install_dir" ]; then
            echo "Error: Directory $install_dir does not exist"
            exit 1
        fi

        INSTALL_DIR=$install_dir
    fi

    if [[ $arg == --rc-file=* ]]; then
        # extract the file from the argument
        rc_file=${arg#--rc-file=}

        # check if the file exists
        if [ ! -f "$rc_file" ]; then
            echo "Error: File $rc_file does not exist"
            exit 1
        fi

        RC_FILE=$rc_file
    fi

    if [[ $arg == --purge ]]; then
        PURGE=true
    fi
done

# check if INSTALL_DIR and RC_FILE are not set
if [ -z "$INSTALL_DIR" ]; then
    echo "Error: --install-dir=<directory> is required"
    exit 1
fi

# purge the installation
if [ -n "$PURGE" ]; then
    # remove the tobi-cli and tobirc.sh
    echo "[+] Removing tobi-cli from $INSTALL_DIR"
    sudo rm $INSTALL_DIR/tobi-cli
    echo "[+] Removing tobirc.sh from $INSTALL_DIR"
    sudo rm $INSTALL_DIR/tobirc.sh
    echo "[+] Done!"
    exit 0
fi

if [ -z "$RC_FILE" ]; then
    echo "Error: --rc-file=<file> is required"
    exit 1
fi

echo "[+] Building tobi..."
cargo build --release
echo "[+] Installing tobi-cli(main binary) to $INSTALL_DIR"
sudo cp target/release/tobi-cli $INSTALL_DIR/tobi-cli
echo "[+] Installing tobirc.sh(wrapper function) to $INSTALL_DIR"
sudo cp ./tobirc.sh $INSTALL_DIR/tobirc.sh
sudo chmod +x $INSTALL_DIR/tobirc.sh $INSTALL_DIR/tobi-cli

# check if ~/.zshrc contains the source command for tobi
if grep -q "source $INSTALL_DIR/tobirc.sh" "$RC_FILE"; then
    echo "[+] tobi wrapper is already installed in $RC_FILE"
else
    echo "[+] Adding tobi wrapper to $RC_FILE"
    echo "source $INSTALL_DIR/tobirc.sh" >> $RC_FILE
fi
echo "[+] Done! Make sure to restart your shell to apply the changes"