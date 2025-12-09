# 🐚 Dx Shell Enhancements

Transform your terminal into an intelligent, AI-powered development environment.

## ✨ Features

### 📊 **Enhanced `ls` Command**
Beautiful file listings with:
- **File type icons** (📁 folders, 📄 files, 🔗 symlinks, 🦀 Rust files, etc.)
- **Human-readable sizes** (KB, MB, GB)
- **Automatic directory grouping**
- **Color-coded permissions**

```bash
$ ls -la
📁  .git
📁  src
🦀  Cargo.toml
📝  README.md
⚙️   install.sh
```

### 💡 **Command Help Hints** (Ctrl+H)
Get instant help for any command as you type:

```bash
$ git <Ctrl+H>

● Common git commands:
  git status    - Show working tree status
  git add       - Add files to staging
  git commit    - Commit staged changes
  git push      - Push commits to remote
  git pull      - Fetch and merge from remote
```

### 🎯 **Smart Command Suggestions**
When you type a wrong command, get intelligent suggestions:

```bash
$ gti status
✗ Command not found: gti

● Did you mean:
  git
  gzip
  
● Popular commands:
  ls, cd, cat, grep, find, git, docker, npm, cargo, dx
```

### 🧠 **Persistent Shell Memory**
Your shell remembers:
- **Command history** across sessions
- **Frequently used commands**
- **Directory navigation patterns**
- **Context-aware suggestions**

```bash
$ dx-freq                # Show most frequent commands
  45  git
  32  cargo
  28  npm
  15  dx

$ dx-suggest             # Get context-aware suggestions
● Smart suggestions for current context:
  Rust project detected:
    cargo build   - Build project
    cargo run     - Run project
    cargo test    - Run tests
    dx generate   - Generate code with Dx
```

### 🚀 **Global Fuzzy Autocomplete**
Intelligent completion for:
- Commands from history
- File paths
- Git branches
- npm scripts
- Dx subcommands

### 📍 **Smart Directory Jumping**
Jump to frequently visited directories:

```bash
$ j pro              # Jumps to ~/projects
$ j dx-c             # Jumps to ~/Code/dx-cli
```

### 🎨 **Enhanced Prompt**
Clean, modern prompt with:
- Status indicator (● green when ready)
- Current directory
- Git branch info (coming soon)

```bash
● ~/Code/dx-cli $
```

## 🚀 Installation

### Quick Install
```bash
# Install Dx CLI first
cargo install dx-cli

# Enable shell enhancements
dx shell --enable

# Run the installation script
bash ~/.dx/shell/install.sh

# Restart your shell
exec $SHELL
```

### Manual Installation

1. **Download scripts:**
```bash
dx shell --enable
```

2. **Add to your shell RC file:**

**Bash** (~/.bashrc):
```bash
if [ -f "$HOME/.dx/shell/dx-shell-init.sh" ]; then
    source "$HOME/.dx/shell/dx-shell-init.sh"
fi
```

**Zsh** (~/.zshrc):
```bash
if [ -f "$HOME/.dx/shell/dx-shell-init.sh" ]; then
    source "$HOME/.dx/shell/dx-shell-init.sh"
fi
```

**Fish** (~/.config/fish/config.fish):
```fish
if test -f "$HOME/.dx/shell/dx-shell-init.sh"
    source "$HOME/.dx/shell/dx-shell-init.sh"
end
```

3. **Reload your shell:**
```bash
source ~/.bashrc  # or ~/.zshrc
```

## 📚 Usage Guide

### Enhanced Commands

| Command | Description |
|---------|-------------|
| `ls` / `ll` | Enhanced file listing with icons |
| `cd <dir>` | Change directory + auto-ls |
| `h [pattern]` | Search command history |
| `j <pattern>` | Jump to frequent directory |

### Memory Functions

| Command | Description |
|---------|-------------|
| `dx-freq [N]` | Show N most frequent commands (default: 10) |
| `dx-local [N]` | Show recent commands in current directory |
| `dx-stats <cmd>` | Show success rate for a command |
| `dx-suggest` | Get context-aware suggestions |

### Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+H` | Show command help hints |
| `Tab` | Autocomplete with fuzzy matching |
| `Ctrl+R` | Reverse history search |
| `Ctrl+K` | Clear activity log (custom) |

## 🔧 Configuration

### Environment Variables

```bash
# Disable auto-ls after cd
export DX_NO_AUTO_LS=1

# Set memory file location
export DX_MEMORY_DB="$HOME/.dx/shell.db"
```

### Aliases

The following aliases are automatically created:

```bash
alias ll='ls -lah'
alias la='ls -A'
alias l='ls -CF'
alias ..='cd ..'
alias ...='cd ../..'
alias ....='cd ../../..'

# Dx shortcuts
alias dxui='dx ui'
alias dxgen='dx generate'
alias dxchat='dx chat'
alias dxtui='dx tui'
```

## 📁 File Structure

```
~/.dx/
├── shell/
│   ├── dx-shell-init.sh        # Main initialization
│   ├── dx-shell-commands.sh    # Enhanced commands
│   ├── dx-shell-memory.sh      # Memory system
│   └── install.sh              # Installation script
├── history/
│   ├── commands.log            # Command history
│   ├── dirs.log                # Directory history
│   └── context.json            # Context data
├── env.sh                      # Persistent environment
└── shell.db                    # Memory database
```

## 🎯 Advanced Features

### Context-Aware Suggestions

The shell automatically detects:
- **Git repositories** → git commands
- **Node.js projects** → npm/yarn commands
- **Rust projects** → cargo commands
- **Python projects** → pip/python commands

### Command Success Tracking

Track which commands succeed/fail:

```bash
$ dx-stats cargo build
Command: cargo build
  Executions: 47
  Success rate: 89%
```

### Persistent Environment

Save environment variables across sessions:

```bash
$ export MY_API_KEY="secret"
$ dx_shell_save_env MY_API_KEY
✓ Saved MY_API_KEY to persistent environment

# Variable available in new shell sessions
```

## 🐛 Troubleshooting

### Shell enhancements not loading

1. Check if scripts exist:
```bash
ls -la ~/.dx/shell/
```

2. Verify shell RC file:
```bash
grep -i "dx-shell" ~/.bashrc  # or ~/.zshrc
```

3. Manually source:
```bash
source ~/.dx/shell/dx-shell-init.sh
```

### Icons not showing

Install a Nerd Font for proper icon display:
- **Recommended:** JetBrains Mono Nerd Font, Fira Code Nerd Font
- Download from: https://www.nerdfonts.com/

### Disable enhancements

```bash
dx shell --disable

# Or remove from shell RC file:
# Delete or comment out the dx-shell-init.sh source line
```

## 🤝 Contributing

Contributions welcome! See [CONTRIBUTING.md](../../CONTRIBUTING.md)

## 📄 License

MIT OR Apache-2.0

---

<div align="center">
  <b>Built with ❤️ by the Dx Team</b>
  <br>
  <sub>Making terminal work magical</sub>
</div>
