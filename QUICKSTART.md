# Dx-CLI Quick Start Guide

## 🎯 What is Dx-CLI?

**Dx-CLI** is a blazingly fast, Rust-powered command-line interface that provides an enhanced development experience with AI-powered tools, intelligent shell features, and comprehensive developer utilities.

## ✨ Key Features

### 🤖 AI Integration
- **Friday AI Assistant**: Integrated AI chat and agents with persistent memory
- **Smart Code Generation**: AI-powered component and project scaffolding
- **Intelligent Suggestions**: Context-aware command recommendations

### 🐚 Enhanced Shell
- **File Icons**: Beautiful file type indicators in `ls` output
- **Smart Autocomplete**: Fuzzy matching for commands and files
- **Command Memory**: Persistent history with intelligent search
- **Help Hints**: Automatic suggestions for command usage
- **Directory Navigation**: Quick jump to frequent directories with `j <query>`

### 🎨 Developer Tools
- **Style System**: Advanced CSS generation and optimization
- **UI Components**: React/Vue/Svelte component library
- **Icon Management**: Lucide and Heroicons integration
- **Font Management**: Web font optimization
- **Media Processing**: Image/video optimization
- **i18n Support**: Translation management with TTS
- **Auth System**: Authentication provider integration

## 🚀 Installation

### Prerequisites
- Rust 1.93+ (nightly)
- Git

### Build from Source

```bash
# Clone the repository
git clone https://github.com/dx-platform/dx-cli.git
cd dx-cli

# Build release binary
cargo build --release

# The binary will be at: target/release/dx-cli.exe (Windows) or target/release/dx-cli (Unix)
```

### Install Shell Enhancements

```bash
# Enable shell enhancements
./target/release/dx-cli shell --enable

# Run the installation script
bash ~/.dx/shell/install.sh

# Restart your shell
exec $SHELL
```

## 📖 Usage Examples

### Shell Commands

```bash
# View available commands
dx help

# Show shell enhancements
dx shell

# Enable enhanced shell features
dx shell --enable

# Disable shell features
dx shell --disable
```

### Enhanced Shell Features (After Installation)

```bash
# Enhanced ls with icons and file sizes
ls
# Output:
# 📄 README.md          (5.2 KB)
# 📁 src/               (4.0 KB)
# ⚙️  Cargo.toml         (1.3 KB)

# Jump to frequent directory
j dx-cli

# Search command history
h cargo build

# Get frequent commands
dx-frequent

# View command statistics
dx-stats
```

### AI Chat (Coming Soon)

```bash
# Start interactive AI chat
dx chat

# Send a single message
dx chat "How do I create a React component?"
```

### Code Generation (Coming Soon)

```bash
# Generate a UI component
dx ui add button
dx ui add accordion --variant outline

# Generate project structure
dx generate project next-app --name my-app

# Generate documentation
dx generate readme --detailed

# Generate tests
dx generate test --type unit --for src/lib.rs
```

### Agent Execution (Coming Soon)

```bash
# Run an AI agent
dx agent run code-review --task "Review my PR"

# List available agents
dx agent list

# Create custom agent
dx agent create --name my-agent --description "Custom automation"
```

## 🎨 Shell Enhancement Features

### File Icons in ls

The enhanced `ls` command shows file type icons:

- 📄 Text files (.txt, .md, .log)
- 🐧 Shell scripts (.sh, .bash, .zsh)
- 🦀 Rust files (.rs)
- 📜 JavaScript/TypeScript (.js, .ts, .jsx, .tsx)
- 🎨 Style files (.css, .scss)
- 📦 Package files (package.json, Cargo.toml)
- 📁 Directories
- 🔧 Config files (.json, .yaml, .toml)

### Command Memory

Every command you run is stored with context:

```bash
# View most used commands
dx-frequent

# Get command suggestions based on current directory
dx-suggest

# Search history
h <query>
```

### Directory Navigation

```bash
# Jump to directory by partial name
j docs        # Jumps to ~/projects/dx-cli/docs
j src/cli     # Jumps to ~/projects/dx-cli/crates/dx-cli/src/cli

# Navigate and auto-ls
cd src        # Automatically shows directory contents
```

### Smart Suggestions

When you mistype a command, Dx suggests corrections:

```bash
$ cargoo build
Command not found: cargoo
Did you mean: cargo?
```

## 📁 Project Structure

```
dx-cli/
├── crates/
│   ├── dx-cli/          # Main CLI binary
│   ├── dx-core/         # Core shared functionality
│   ├── dx-tui/          # Terminal UI components
│   ├── dx-ai/           # Friday AI integration
│   ├── dx-shell/        # Shell enhancements
│   ├── dx-generator/    # Code generation
│   ├── dx-style/        # CSS system
│   ├── dx-ui/           # UI components
│   ├── dx-icons/        # Icon management
│   ├── dx-fonts/        # Font management
│   ├── dx-auth/         # Authentication
│   ├── dx-media/        # Media processing
│   ├── dx-i18n/         # Internationalization
│   ├── dx-forge/        # VCS & orchestration
│   └── dx-lsp/          # LSP server
├── assets/              # ASCII art, templates
├── docs/                # Documentation
└── target/              # Build output
```

## ⚙️ Configuration

Dx uses a single configuration file: `dx.toml` or `dx.json`

Example `dx.toml`:

```toml
[dx]
version = "0.1.0"

[cli]
color = "auto"
verbose = false

[ai]
provider = "friday"
model = "friday-pro"
temperature = 0.7

[shell]
autocomplete = true
fuzzy_threshold = 0.6
history_size = 10000
ai_suggestions = true

[ui]
components_path = "src/components"
framework = "react"
```

## 🔧 Development

### Build Commands

```bash
# Build all workspace crates
cargo build --workspace

# Build specific crate
cargo build -p dx-cli

# Build release
cargo build --release

# Run tests
cargo test --workspace

# Run clippy
cargo clippy --workspace

# Format code
cargo fmt --all
```

### Testing Shell Enhancements

```bash
# Test without installing
source ~/.dx/shell/dx-shell-init.sh

# Test individual features
source ~/.dx/shell/dx-shell-commands.sh
source ~/.dx/shell/dx-shell-memory.sh

# Run demo
bash ~/.dx/shell/demo.sh
```

## 🐛 Troubleshooting

### Shell detection issues

If shell detection fails, manually set it:

```bash
export DX_SHELL=bash  # or zsh, fish, powershell
```

### Shell enhancements not working

1. Verify installation:
   ```bash
   ls -la ~/.dx/shell/
   ```

2. Check shell rc file:
   ```bash
   grep -n "Dx Shell" ~/.bashrc  # or ~/.zshrc
   ```

3. Manually source:
   ```bash
   source ~/.dx/shell/dx-shell-init.sh
   ```

### Build errors

1. Ensure Rust 1.93+ nightly:
   ```bash
   rustc --version
   # Should show: rustc 1.93.0-nightly
   ```

2. Update toolchain:
   ```bash
   rustup update nightly
   rustup default nightly
   ```

3. Clean build:
   ```bash
   cargo clean
   cargo build --workspace
   ```

## 📚 Documentation

- [Architecture Overview](docs/architecture.md)
- [Shell Enhancements Guide](SHELL_ENHANCEMENTS.md)
- [Contributing Guidelines](docs/CONTRIBUTING.md)
- [Changelog](docs/CHANGELOG.md)

## 🤝 Contributing

Contributions are welcome! Please read our [Contributing Guide](docs/CONTRIBUTING.md) for details.

## 📄 License

MIT OR Apache-2.0

## 🌟 Roadmap

### Phase 1: Core CLI ✅
- [x] Project structure
- [x] CLI argument parsing
- [x] Banner display
- [x] Command routing
- [x] Configuration loading

### Phase 2: Shell Integration ✅
- [x] Autocomplete engine
- [x] Shell hooks (bash, zsh, fish, powershell)
- [x] History management
- [x] Enhanced ls with icons
- [x] Command suggestions
- [x] Directory navigation

### Phase 3: AI Integration 🚧
- [ ] Friday AI client
- [ ] Chat interface (TUI)
- [ ] Context management
- [ ] Memory persistence
- [ ] Agent orchestrator

### Phase 4: Code Generation 📋
- [ ] Template engine
- [ ] Component generator
- [ ] Project generator
- [ ] Documentation generator
- [ ] Test generator

### Phase 5: DXP & Tools 📋
- [ ] DX Protocol implementation
- [ ] Style system
- [ ] UI component library
- [ ] Icon management
- [ ] Font optimization
- [ ] Media processing
- [ ] i18n system
- [ ] Authentication providers

---

**Built with ❤️ using Rust 1.93**

For more information, visit: https://dx.dev
