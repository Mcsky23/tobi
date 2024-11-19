#!/bin/bash

# Define the tobi function
function tobi() {
    # Run tobi with the arguments passed to this function
    RM_SET=0
    unameOut="$(uname)"
    if [[ $unameOut == "Darwin" ]]; then
        script_cmd='script -q /dev/null tobi-cli "$@"'
    else
        script_cmd='script -q -c "tobi-cli $*" /dev/null'
    fi
    if [[ $1 == "settings" ]] || [[ $1 == "rm" ]]; then
        # check if I am on macOS or Linux by running uname
        eval $script_cmd
        # if [[ $1 == "rm" ]]; then
        #     # Switch to context directory

        if [[ $1 == "rm" ]]; then
            RM_SET=1
        else
            return
        fi
    fi

    # check if output is empty
    if [[ $RM_SET -eq 1 ]]; then
        output=$(tobi-cli "ctf" "NO_UNDO")
    else
        # output=$(tobi-cli "$@")
        output=$(eval $script_cmd)
    fi

    # Check if the output contains a line starting with "CHANGE_DIR"
    # iterate over each line in output var
    while IFS= read -r line; do
        # Check if the line starts with "CHANGE_DIR"
        if [[ $line == "^CHANGE_DIR^"* ]]; then
            # Extract the tring before ^CHANGE_DIR^
            # dir=${line#"^CHANGE_DIR^ "}
            dir=$(echo $line | cut -d'^' -f3)
            # Change the directory
            cd "$dir"
        fi
    done <<< "$output"

    # Print output with the directory line removed
    echo -e "$output" | grep -v "\^CHANGE_DIR\^"
}

function _tobi_completions() {
    # complete the ctf name by looking in the workspace dir
    local cur buf
    cur="${COMP_WORDS[COMP_CWORD]}"

    buf=$(tobi-cli _autocomplete "$((COMP_CWORD))" "${COMP_WORDS[@]:1}")
    COMPREPLY=($(compgen -W "$buf" -- "$cur"))
}

complete -F _tobi_completions tobi