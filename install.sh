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
        echo "  --uninstall: Uninstall tobi"
        echo ""
        echo "Example: ./install.sh --install-dir=/opt/scripts"
        echo "         ./install.sh --uninstall"
        exit 0
    fi

    if [[ $arg == --uninstall ]]; then
        echo "Uninstalling tobi..."
        UNINSTALL=true
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

    if [[ $arg == --purge ]]; then
        PURGE=true
    fi
done

# get rc file dir for current shell env var
shell=".$(echo $SHELL | awk -F'/' '{print $3}')rc"
RC_FILE="$HOME/$shell"

echo "[+] Found shell rc file: $RC_FILE"

# check if UNINSTALL is set

if [ -n "$UNINSTALL" ]; then
    # get install dir by searching in rc file
    install_dir=$(grep -e "source.*tobirc.sh" $RC_FILE | sed -e 's/source //' -e 's/tobirc.sh//')
    # remove last character if it's a slash
    install_dir=$(echo $install_dir | sed 's/.$//')

    # remove this line from the rc file
    echo "[+] Removing tobi wrapper from $RC_FILE"

    esc_install_dir=$(echo $install_dir | sed 's/\//\\\//g')
    # check if it's macos or linux
    if [[ "$OSTYPE" == "darwin"* ]]; then
        sudo sed -i '' "/source $esc_install_dir\/tobirc\.sh/d" $RC_FILE
    else
        sudo sed -i "/source $esc_install_dir\/tobirc\.sh/d" $RC_FILE
    fi

    # remove the tobi-cli and tobirc.sh
    echo "[+] Removing tobi-cli from $install_dir"
    sudo rm $install_dir/tobi-cli
    echo "[+] Removing tobirc.sh from $install_dir"
    sudo rm $install_dir/tobirc.sh
    echo "[+] Done!"
    exit 0
fi

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
echo "[!] NOTE: Add your install-dir to your PATH variable if it's not already added"