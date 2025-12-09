# 🚀 Dx-CLI - Enhanced Development Experience

<div align="center">

```
 ____       
|  _ \__  __
| | | \ \/ /
| |_| |>  <
|____//_/\_\

Enhanced Development Experience
```

**A blazingly fast, AI-powered command-line interface for modern developers**

[![Rust Version](https://img.shields.io/badge/rust-1.93.0--nightly-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](#)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)](#)

[Features](#-features) • [Quick Start](#-quick-start) • [Installation](#-installation) • [Documentation](#-documentation)

</div>

---

## ✨ Features

### 🤖 **AI Chat Interface** ✅ NEW!
- 💬 **Interactive AI** - Run `dx` to chat with Google Gemini AI
- 🎯 **Command Execution** - Run shell/dx commands from chat
- 🔑 **Free API** - Uses Google Gemini with free tier (1500 requests/day)
- ⌨️ **Smart Detection** - Auto-detects commands vs AI queries
- 💾 **Conversation History** - Maintains context during session

### 🐚 **Intelligent Shell** ✅ WORKING
- 📄 **Enhanced ls** with file type icons
- 🔍 **Smart autocomplete** with fuzzy matching
- 💾 **Command memory** - remembers every command with context
- 💡 **Typo suggestions** - "Did you mean: cargo?"
- 🚀 **Quick navigation** - Jump to directories with `j <query>`
- 📖 **Help hints** - Automatic command usage suggestions

### 🎯 **Professional UX** ✅ COMPLETE
- 🎨 **Modern TUI** - Vercel-inspired terminal interface
- 🌍 **Cross-platform** - Windows, macOS, Linux, Termux
- ⚡ **Zero config** - Works out of the box
- 🦀 **Lightning fast** - Built with Rust for maximum performance

### 🤖 **AI Integration** (Coming Soon)
- Friday AI Assistant
- Smart code generation
- Context-aware suggestions
- Agent orchestration

### 🛠️ **Developer Tools** (Coming Soon)
- Style System, UI Components
- Icon & Font Management
- Media Processing, i18n
- Auth & VCS Integration

---

## 🚀 Quick Start

```bash
# Build
cargo build --release

# Start AI chat (no arguments = interactive chat!)
./target/release/dx
# OR
dx

# Ask AI anything or run commands:
#   "How do I use tokio?"
#   "ls -la"
#   "dx shell --enable"

# Set your own API key (optional, has default)
dx config set-api-key YOUR_GEMINI_API_KEY

# Install shell enhancements
dx shell --enable
bash ~/.dx/shell/install.sh
exec $SHELL

# Try enhanced features
ls              # Enhanced ls with icons
j dx-cli        # Jump to directory
h cargo         # Search history
```

---

## 📖 Usage

### AI Chat Interface

```bash
# Launch interactive AI chat
dx

# In the chat, you can:
#   - Ask questions: "How do I create a React component?"
#   - Run commands: "cargo build"
#   - Use dx tools: "dx ui add button"
#   - Exit: Ctrl+C

# Set your own API key
dx config set-api-key YOUR_KEY
dx config get-api-key
```

### Basic Commands
```bash
dx --help                # Show all commands
dx --version             # Show version
dx shell                 # View shell features
dx shell --enable        # Install enhancements
```

### Enhanced Shell Features
```bash
ls                       # 📄 README.md (5.2 KB)
j dx-cli                 # Jump to directory
h cargo build            # Search history
dx-frequent              # Most used commands
dx-stats                 # Command statistics
```

---

## 🏗️ Architecture

```
dx-cli/
├── crates/
│   ├── dx-cli/          # Main CLI binary ✅
│   ├── dx-tui/          # Terminal UI ✅
│   ├── dx-shell/        # Shell enhancements ✅
│   ├── dx-core/         # Core functionality ✅
│   ├── dx-ai/           # AI integration 🚧
│   └── 10+ more...      # Developer tools 📋
├── assets/              # Banner, templates
└── docs/                # Documentation
```

**Technology Stack:**
Rust 1.93 • Clap 4.5 • Ratatui 0.29 • Tokio 1.42 • Sled 0.34

---

## 📚 Documentation

- 📖 [Quick Start Guide](QUICKSTART.md) - Get started in 5 minutes
- 🐚 [Shell Enhancements](SHELL_ENHANCEMENTS.md) - All shell features
- 📊 [Project Status](PROJECT_STATUS.md) - Current implementation
- 🏗️ [Architecture](docs/architecture.md) - System design

---

## 🎯 Roadmap

### ✅ Phase 1 & 2: Core + Shell (COMPLETED)
- [x] Professional CLI with Vercel-inspired TUI
- [x] Enhanced shell (ls, autocomplete, memory, navigation)
- [x] Cross-platform support (Windows/Mac/Linux/Termux)

### ✅ Phase 3: AI Integration (COMPLETED)
- [x] Google Gemini AI client
- [x] Interactive chat interface
- [x] Command execution from chat
- [x] API key management

### 📋 Phase 4 & 5: Tools (PLANNED)
- [ ] Code generation (components, projects, docs)
- [ ] Developer tools (style, ui, icons, fonts, media, i18n, auth)
- [ ] Agent orchestrator

---

## 🎨 Shell Features

### File Icons
📄 Text • 🦀 Rust • 🐧 Shell • 📜 JS/TS • 🎨 CSS • 📦 Configs • 📁 Dirs

### Persistent Memory
Every command stored with context (dir, exit code, timestamp, duration)

### Smart Navigation
```bash
j <query>      # Jump to matching directory
cd src         # Auto-ls after cd
```

---

## 🌍 Platform Support

| Platform | Status | Shells |
|----------|--------|--------|
| Windows | ✅ | PowerShell, Git Bash, WSL |
| macOS | ✅ | Bash, Zsh |
| Linux | ✅ | Bash, Zsh, Fish |
| Termux | ✅ | Bash, Zsh |

---

## 🛠️ Development

```bash
cargo build --workspace      # Build all
cargo test --workspace       # Test all
cargo clippy --workspace     # Lint all
cargo fmt --all              # Format all
```

---

## 📄 License

MIT OR Apache-2.0

---

<div align="center">

**Built with ❤️ using Rust 1.93**

*Version 0.1.0*

[⬆ Back to Top](#-dx-cli---enhanced-development-experience)

</div>
