#!/bin/bash

# Define the tobi function
function tobi() {
    # Run tobi with the arguments passed to this function
    if [[ $1 == "settings" ]]; then
        # check if I am on macOS or Linux by running uname
        unameOut="$(uname)"
        if [[ $unameOut == "Darwin" ]]; then
            script -q /dev/null tobi-cli "$@"
        else
            script -q -c "tobi-cli $*" /dev/null
        fi
        return
    fi

    output=$(tobi-cli "$@")

    # Check if the output contains a line starting with "CHANGE_DIR"
    # iterate over each line in output var
    while IFS= read -r line; do
        # Check if the line starts with "CHANGE_DIR"
        if [[ $line == "CHANGE_DIR: "* ]]; then
            # Extract the directory from the line
            dir=${line#"CHANGE_DIR: "}
            # Change the directory
            cd "$dir"
        fi
    done <<< "$output"

    # Print output with the directory line removed
    echo "$output" | grep -v "CHANGE_DIR: "
}

function _tobi_completions() {
    # complete the ctf name by looking in the workspace dir
    local cur buf
    cur="${COMP_WORDS[COMP_CWORD]}"

    buf=$(tobi _autocomplete "$((COMP_CWORD))" "${COMP_WORDS[@]:1}")
    COMPREPLY=($(compgen -W "$buf" -- "$cur"))
}

complete -F _tobi_completions tobi