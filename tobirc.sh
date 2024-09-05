#!/bin/bash

# Define the tobi function
function tobi() {
    # Run tobi with the arguments passed to this function
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

    # check if the first arg is ctf or context
    # if [[ ${COMP_WORDS[1]} == "ctf" ]] || [[ ${COMP_WORDS[1]} == "context" ]]; then
    #     # check if I am at ctf argument or challenge argument
    #     if [[ $COMP_CWORD -eq 2 ]]; then
    #         # complete ctf name
    #         COMPREPLY=($(compgen -W "$(tobi list ctfs)" -- "$cur"))
    #     elif [[ $COMP_CWORD -eq 3 ]]; then
    #         # complete challenge name
    #         ctf=${COMP_WORDS[2]}

    #         challenges=$(tobi list "$ctf" | grep -v "$ctf")
    #         # split every line by space and get the last element
    #         challenges=$(echo "$challenges" | awk '{print $NF}')

    #         COMPREPLY=($(compgen -W "$challenges" -- "$cur"))
    #     fi
    # fi

    buf=$(tobi _autocomplete "$((COMP_CWORD))" "${COMP_WORDS[@]:1}")
    COMPREPLY=($(compgen -W "$buf" -- "$cur"))


}

complete -F _tobi_completions tobi