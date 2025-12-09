#!/usr/bin/env bash
# Dx Shell Memory System
# Tracks commands, context, and provides intelligent suggestions

DX_MEMORY_FILE="${DX_CONFIG_DIR}/history/commands.log"
DX_CONTEXT_FILE="${DX_CONFIG_DIR}/history/context.json"

# Initialize memory files
mkdir -p "${DX_CONFIG_DIR}/history"
touch "$DX_MEMORY_FILE"

# Remember a command execution
dx_shell_remember_command() {
    local cmd="$1"
    local exit_code="${2:-0}"
    local timestamp=$(date +%s)
    local pwd=$(pwd)
    
    # Skip empty commands
    [ -z "$cmd" ] && return
    
    # Skip internal commands
    [[ "$cmd" =~ ^_dx_ ]] && return
    
    # Log format: timestamp|pwd|exit_code|command
    echo "${timestamp}|${pwd}|${exit_code}|${cmd}" >> "$DX_MEMORY_FILE"
    
    # Keep only last 10000 commands
    if [ $(wc -l < "$DX_MEMORY_FILE") -gt 10000 ]; then
        tail -10000 "$DX_MEMORY_FILE" > "${DX_MEMORY_FILE}.tmp"
        mv "${DX_MEMORY_FILE}.tmp" "$DX_MEMORY_FILE"
    fi
}

# Get frequently used commands
dx_shell_frequent_commands() {
    local limit="${1:-10}"
    
    if [ -f "$DX_MEMORY_FILE" ]; then
        awk -F'|' '{print $4}' "$DX_MEMORY_FILE" | \
        awk '{print $1}' | \
        sort | uniq -c | sort -rn | head -n "$limit" | \
        awk '{printf "%3d  %s\n", $1, $2}'
    fi
}

# Get recent commands in current directory
dx_shell_local_history() {
    local pwd=$(pwd)
    local limit="${1:-20}"
    
    if [ -f "$DX_MEMORY_FILE" ]; then
        grep "|${pwd}|" "$DX_MEMORY_FILE" | \
        tail -n "$limit" | \
        awk -F'|' '{print $4}'
    fi
}

# Get command success rate
dx_shell_command_stats() {
    local cmd="$1"
    
    if [ -f "$DX_MEMORY_FILE" ]; then
        local total=$(grep -c "|${cmd}" "$DX_MEMORY_FILE")
        local success=$(grep "|0|${cmd}" "$DX_MEMORY_FILE" | wc -l)
        
        if [ $total -gt 0 ]; then
            local rate=$((success * 100 / total))
            echo "Command: $cmd"
            echo "  Executions: $total"
            echo "  Success rate: ${rate}%"
        else
            echo "No history for command: $cmd"
        fi
    fi
}

# Smart command suggestions based on context
dx_shell_suggest() {
    local current_dir=$(pwd)
    local git_repo=false
    local node_project=false
    local rust_project=false
    
    # Detect project type
    [ -d ".git" ] && git_repo=true
    [ -f "package.json" ] && node_project=true
    [ -f "Cargo.toml" ] && rust_project=true
    
    echo -e "\n\033[38;5;39m●\033[0m Smart suggestions for current context:"
    
    if $git_repo; then
        echo "  Git repository detected:"
        echo "    git status    - Check repository status"
        echo "    git log       - View commit history"
        echo "    git diff      - See changes"
    fi
    
    if $node_project; then
        echo "  Node.js project detected:"
        echo "    npm install   - Install dependencies"
        echo "    npm run dev   - Start dev server"
        echo "    npm test      - Run tests"
    fi
    
    if $rust_project; then
        echo "  Rust project detected:"
        echo "    cargo build   - Build project"
        echo "    cargo run     - Run project"
        echo "    cargo test    - Run tests"
        echo "    dx generate   - Generate code with Dx"
    fi
    
    # Show frequently used commands in this directory
    echo ""
    echo "  Recent commands here:"
    dx_shell_local_history 5 | sed 's/^/    /'
}

# Export memory functions
alias dx-stats='dx_shell_command_stats'
alias dx-freq='dx_shell_frequent_commands'
alias dx-suggest='dx_shell_suggest'
alias dx-local='dx_shell_local_history'

# Intelligent autocomplete enhancement
if [ -n "$BASH_VERSION" ]; then
    # Enhanced completion for common commands
    _dx_enhanced_complete() {
        local cur="${COMP_WORDS[COMP_CWORD]}"
        local prev="${COMP_WORDS[COMP_CWORD-1]}"
        
        # Get suggestions from history
        local suggestions=$(dx_shell_frequent_commands 20 | awk '{print $2}')
        
        # Add to completion
        COMPREPLY=($(compgen -W "$suggestions" -- "$cur"))
    }
    
    # Apply to common commands
    complete -F _dx_enhanced_complete -o default cd
fi

# Persistent environment variables
DX_ENV_FILE="${DX_CONFIG_DIR}/env.sh"

dx_shell_save_env() {
    local var_name="$1"
    local var_value="${!var_name}"
    
    if [ -n "$var_value" ]; then
        echo "export ${var_name}=\"${var_value}\"" >> "$DX_ENV_FILE"
        echo "✓ Saved $var_name to persistent environment"
    fi
}

dx_shell_load_env() {
    if [ -f "$DX_ENV_FILE" ]; then
        source "$DX_ENV_FILE"
    fi
}

# Load saved environment on startup
dx_shell_load_env

# Export functions
export -f dx_shell_remember_command
export -f dx_shell_frequent_commands
export -f dx_shell_local_history
export -f dx_shell_command_stats
export -f dx_shell_suggest
export -f dx_shell_save_env
export -f dx_shell_load_env
