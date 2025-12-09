#!/usr/bin/env bash
# Dx Shell Initialization
# Loaded when shell starts

export DX_SHELL_DIR="${HOME}/.dx/shell"
export DX_CONFIG_DIR="${HOME}/.dx"
export DX_MEMORY_DB="${DX_CONFIG_DIR}/shell.db"

# Load enhanced commands
if [ -f "${DX_SHELL_DIR}/dx-shell-commands.sh" ]; then
    source "${DX_SHELL_DIR}/dx-shell-commands.sh"
fi

# Load memory system
if [ -f "${DX_SHELL_DIR}/dx-shell-memory.sh" ]; then
    source "${DX_SHELL_DIR}/dx-shell-memory.sh"
fi

# Load completions
if [ -f "${DX_CONFIG_DIR}/dx.bash" ]; then
    source "${DX_CONFIG_DIR}/dx.bash"
fi

# Initialize command tracking
_dx_track_command() {
    local cmd="$1"
    local exit_code="$?"
    
    # Store in memory
    dx_shell_remember_command "$cmd" "$exit_code"
}

# Hook into command execution
if [ -n "$BASH_VERSION" ]; then
    # Bash: use DEBUG trap
    trap '_dx_last_command=$BASH_COMMAND' DEBUG
    
    PROMPT_COMMAND="_dx_track_command \"\$_dx_last_command\"; ${PROMPT_COMMAND}"
    
    # Enhanced prompt with Dx indicator
    export PS1="\[\033[38;5;39m\]●\[\033[0m\] \[\033[1m\]\w\[\033[0m\] \$ "
    
elif [ -n "$ZSH_VERSION" ]; then
    # Zsh: use preexec/precmd hooks
    autoload -Uz add-zsh-hook
    
    _dx_preexec() {
        _dx_last_command="$1"
    }
    
    _dx_precmd() {
        _dx_track_command "$_dx_last_command"
    }
    
    add-zsh-hook preexec _dx_preexec
    add-zsh-hook precmd _dx_precmd
    
    # Enhanced prompt
    export PS1="%F{39}●%f %B%~%b %# "
fi

# Welcome message
echo -e "\033[38;5;39m●\033[0m Dx Shell Enhancements active"
