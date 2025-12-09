#!/usr/bin/env bash
# Dx Enhanced Shell Commands

# Enhanced ls command with icons and better formatting
ls() {
    local use_icons=true
    local show_hidden=false
    local long_format=false
    
    # Parse arguments
    local args=()
    for arg in "$@"; do
        case "$arg" in
            -a|--all)
                show_hidden=true
                args+=("$arg")
                ;;
            -l|--long)
                long_format=true
                args+=("$arg")
                ;;
            --no-icons)
                use_icons=false
                ;;
            *)
                args+=("$arg")
                ;;
        esac
    done
    
    # Run standard ls first
    if command -v eza &> /dev/null; then
        # Use eza if available (modern ls replacement)
        eza --icons --group-directories-first "${args[@]}"
    elif command -v exa &> /dev/null; then
        # Use exa if available
        exa --icons --group-directories-first "${args[@]}"
    else
        # Fallback to enhanced output with standard ls
        if $long_format; then
            command ls -lh --color=auto --group-directories-first "${args[@]}" | \
            awk 'NR==1{print; next} {
                # Add file type icons
                icon="📄"
                if ($1 ~ /^d/) icon="📁"
                else if ($1 ~ /^l/) icon="🔗"
                else if ($9 ~ /\.(sh|bash|zsh)$/) icon="⚙️ "
                else if ($9 ~ /\.(js|ts|jsx|tsx)$/) icon="📜"
                else if ($9 ~ /\.(rs|toml)$/) icon="🦀"
                else if ($9 ~ /\.(py)$/) icon="🐍"
                else if ($9 ~ /\.(md|txt)$/) icon="📝"
                else if ($9 ~ /\.(jpg|png|gif|svg)$/) icon="🖼️ "
                else if ($9 ~ /\.(zip|tar|gz)$/) icon="📦"
                
                printf "%s  %s\n", icon, $0
            }'
        else
            # Simple column format with colors
            command ls --color=auto --group-directories-first "${args[@]}"
        fi
    fi
}

# Enhanced cd with directory history
cd() {
    builtin cd "$@" || return
    
    # Remember directory
    echo "$(pwd)" >> "${DX_CONFIG_DIR}/history/dirs.log"
    
    # Auto-ls after cd
    if [ -z "$DX_NO_AUTO_LS" ]; then
        ls -la
    fi
}

# Command help hint system
_dx_show_help_hint() {
    local cmd="$1"
    
    case "$cmd" in
        git)
            echo -e "\n\033[38;5;39m●\033[0m Common git commands:"
            echo "  git status    - Show working tree status"
            echo "  git add       - Add files to staging"
            echo "  git commit    - Commit staged changes"
            echo "  git push      - Push commits to remote"
            echo "  git pull      - Fetch and merge from remote"
            ;;
        docker)
            echo -e "\n\033[38;5;39m●\033[0m Common docker commands:"
            echo "  docker ps     - List running containers"
            echo "  docker images - List images"
            echo "  docker run    - Run a container"
            echo "  docker stop   - Stop a container"
            echo "  docker logs   - View container logs"
            ;;
        npm|yarn|pnpm)
            echo -e "\n\033[38;5;39m●\033[0m Common $cmd commands:"
            echo "  $cmd install  - Install dependencies"
            echo "  $cmd run      - Run a script"
            echo "  $cmd test     - Run tests"
            echo "  $cmd build    - Build project"
            echo "  $cmd dev      - Start dev server"
            ;;
        cargo)
            echo -e "\n\033[38;5;39m●\033[0m Common cargo commands:"
            echo "  cargo build   - Compile project"
            echo "  cargo run     - Build and run"
            echo "  cargo test    - Run tests"
            echo "  cargo check   - Check for errors"
            echo "  cargo update  - Update dependencies"
            ;;
        dx)
            echo -e "\n\033[38;5;39m●\033[0m Dx commands:"
            echo "  dx ui         - Manage UI components"
            echo "  dx style      - Compile styles"
            echo "  dx generate   - Generate code"
            echo "  dx chat       - Chat with Friday AI"
            echo "  dx tui        - Launch interactive UI"
            ;;
    esac
}

# Command suggestion for typos
command_not_found_handle() {
    local cmd="$1"
    shift
    
    echo -e "\033[38;5;196m✗\033[0m Command not found: \033[1m$cmd\033[0m"
    echo ""
    
    # Try to suggest similar commands
    local suggestions=($(compgen -c | grep -i "^${cmd:0:3}" | head -5))
    
    if [ ${#suggestions[@]} -gt 0 ]; then
        echo -e "\033[38;5;39m●\033[0m Did you mean:"
        for suggestion in "${suggestions[@]}"; do
            echo "  $suggestion"
        done
    fi
    
    # Show popular commands
    echo ""
    echo -e "\033[38;5;39m●\033[0m Popular commands:"
    echo "  ls, cd, cat, grep, find, git, docker, npm, cargo, dx"
    
    return 127
}

# Enhanced history search
h() {
    if [ -z "$1" ]; then
        # Show recent history
        history | tail -20
    else
        # Search history
        history | grep -i "$1"
    fi
}

# Quick directory jumper
j() {
    local target="$1"
    
    if [ -z "$target" ]; then
        echo "Usage: j <directory_pattern>"
        return 1
    fi
    
    # Find matching directory in recent history
    local found=$(tail -100 "${DX_CONFIG_DIR}/history/dirs.log" 2>/dev/null | \
                  grep -i "$target" | \
                  tail -1)
    
    if [ -n "$found" ]; then
        cd "$found"
    else
        echo "No matching directory found in history"
        return 1
    fi
}

# Show command info as you type (for bash with readline)
if [ -n "$BASH_VERSION" ]; then
    bind -x '"\C-h": _dx_show_help_hint "${READLINE_LINE%% *}"'
fi

# Enhanced aliases
alias ll='ls -lah'
alias la='ls -A'
alias l='ls -CF'
alias ..='cd ..'
alias ...='cd ../..'
alias ....='cd ../../..'
alias grep='grep --color=auto'
alias df='df -h'
alias du='du -h'
alias free='free -h'
alias ports='netstat -tulanp 2>/dev/null || ss -tulanp'
alias myip='curl -s https://api.ipify.org; echo'

# Dx-specific aliases
alias dxui='dx ui'
alias dxgen='dx generate'
alias dxchat='dx chat'
alias dxtui='dx tui'
