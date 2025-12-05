# .github/copilot-instructions.md

Before doing anything please create proper planning with tasklist and the complete those task step by step systemitically:

# Dx-CLI - GitHub Copilot Agent Instructions

> This document provides comprehensive instructions for GitHub Copilot to assist in building Dx-CLI, a high-performance Rust-based command-line interface for the Dx Enhanced Development Experience platform.

---

## 📋 Table of Contents

1. [Project Overview](#project-overview)
2. [Architecture Guidelines](#architecture-guidelines)
3. [Rust Best Practices](#rust-best-practices)
4. [Recommended Crates](#recommended-crates)
5. [Project Structure](#project-structure)
6. [Core Modules](#core-modules)
7. [Feature Implementation Guidelines](#feature-implementation-guidelines)
8. [AI Integration (Friday)](#ai-integration-friday)
9. [DXP Protocol](#dxp-protocol)
10. [Terminal Enhancement](#terminal-enhancement)
11. [Generator/Templater System](#generatortemplater-system)
12. [Agent Orchestrator](#agent-orchestrator)
13. [Configuration System](#configuration-system)
14. [Error Handling Strategy](#error-handling-strategy)
15. [Testing Guidelines](#testing-guidelines)
16. [Performance Optimization](#performance-optimization)
17. [Security Guidelines](#security-guidelines)
18. [Documentation Standards](#documentation-standards)
19. [Code Style Guide](#code-style-guide)
20. [Release and Distribution](#release-and-distribution)

---

## 🎯 Project Overview

### What is Dx-CLI?

Dx-CLI is a blazingly fast, Rust-powered command-line interface that serves as the primary interaction point for the Dx Enhanced Development Experience platform. It combines:

- **AI-Powered Terminal**: Integrated Friday AI chat and agents with persistent memory
- **Intelligent Shell**: Global autocomplete, command intelligence, and shell memory
- **Tool Orchestration**: Unified access to all Dx tools (style, ui, icons, fonts, auth, media, i18n, forge)
- **Code Generation**: Smart templating system for generating code, projects, and documentation
- **DXP Protocol**: Dx Protocol - superior alternative to MCP for AI-tool communication
- **Package Management**: Transparent, secure package management with version control

### Core Philosophy

```
"Every command should feel magical - fast, intelligent, and transparent"
```

**Key Principles:**
1. **Speed First**: Leverage Rust's zero-cost abstractions for maximum performance
2. **Intelligence Everywhere**: AI-enhanced suggestions, completions, and automation
3. **Transparency**: Users see and own all generated code
4. **Single Config**: One dx.toml/dx.json file for everything
5. **Offline Capable**: Core functionality works without network

---

## 🏗️ Architecture Guidelines

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              DX-CLI APPLICATION                              │
├─────────────────────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐ │
│  │   TUI Layer │  │  CLI Parser │  │  Shell Hook │  │  LSP Integration    │ │
│  │  (ratatui)  │  │   (clap)    │  │  (rustyline)│  │  (tower-lsp)        │ │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘  └──────────┬──────────┘ │
│         │                │                │                     │           │
│         └────────────────┴────────────────┴─────────────────────┘           │
│                                    │                                         │
│  ┌─────────────────────────────────▼─────────────────────────────────────┐  │
│  │                        COMMAND ROUTER & DISPATCHER                     │  │
│  │                    (async command execution engine)                    │  │
│  └─────────────────────────────────┬─────────────────────────────────────┘  │
│                                    │                                         │
│  ┌──────────┬──────────┬──────────┼──────────┬──────────┬──────────┐       │
│  ▼          ▼          ▼          ▼          ▼          ▼          ▼       │
│ ┌────┐   ┌────┐   ┌────┐   ┌──────────┐  ┌────┐   ┌────┐   ┌────────┐     │
│ │Style│   │ UI │   │Icon│   │Generator │  │Auth│   │Media│  │  Forge │     │
│ │Core │   │Core│   │Core│   │ Engine   │  │Core│   │Core │  │  (VCS) │     │
│ └──┬──┘   └──┬─┘   └──┬─┘   └────┬─────┘  └──┬─┘   └──┬──┘  └────┬───┘     │
│    │         │        │          │           │        │          │          │
│  ┌─┴─────────┴────────┴──────────┴───────────┴────────┴──────────┴───────┐ │
│  │                         DX CORE RUNTIME                                │ │
│  │  ┌───────────┐ ┌───────────┐ ┌───────────┐ ┌───────────┐ ┌──────────┐ │ │
│  │  │  Config   │ │   Cache   │ │  Storage  │ │  Network  │ │  Friday  │ │ │
│  │  │  Manager  │ │  Manager  │ │  Layer    │ │  Client   │ │ AI Core  │ │ │
│  │  └───────────┘ └───────────┘ └───────────┘ └───────────┘ └──────────┘ │ │
│  └────────────────────────────────────────────────────────────────────────┘ │
│                                    │                                         │
│  ┌─────────────────────────────────▼─────────────────────────────────────┐  │
│  │                           DXP PROTOCOL LAYER                           │  │
│  │              (Dx Protocol - AI Tool Communication Protocol)            │  │
│  └────────────────────────────────────────────────────────────────────────┘  │
│                                    │                                         │
│  ┌─────────────────────────────────▼─────────────────────────────────────┐  │
│  │                        AGENT ORCHESTRATOR                              │  │
│  │     (Task scheduling, parallel execution, workflow management)         │  │
│  └────────────────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Layered Architecture Pattern

When generating code, Copilot MUST follow this layered architecture:

```rust
// Layer 1: Presentation (CLI/TUI)
// - Command parsing, TUI rendering, user interaction
// - Located in: src/cli/, src/tui/

// Layer 2: Application (Use Cases)
// - Business logic, command handlers, workflows
// - Located in: src/commands/, src/workflows/

// Layer 3: Domain (Core Business)
// - Domain models, business rules, validations
// - Located in: src/domain/, src/core/

// Layer 4: Infrastructure (External Services)
// - File system, network, database, AI services
// - Located in: src/infrastructure/, src/services/
```

### Module Communication Rules

1. **Upper layers CAN depend on lower layers**
2. **Lower layers MUST NOT depend on upper layers**
3. **Same-level modules communicate through defined interfaces**
4. **Cross-cutting concerns (logging, metrics) use traits**

---

## 🦀 Rust Best Practices

### MUST Follow These Rust Idioms

#### 1. Ownership and Borrowing

```rust
// ✅ CORRECT: Use references when you don't need ownership
fn process_command(cmd: &Command) -> Result<Output> { ... }

// ✅ CORRECT: Use owned types when you need to store/move data
fn store_command(cmd: Command) -> Result<()> { ... }

// ❌ AVOID: Unnecessary cloning
fn bad_process(cmd: Command) -> Result<Output> {
    let cloned = cmd.clone(); // Unnecessary!
    ...
}
```

#### 2. Error Handling

```rust
// ✅ CORRECT: Use thiserror for library errors
#[derive(Debug, thiserror::Error)]
pub enum DxError {
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),
    
    #[error("AI service unavailable: {0}")]
    AiService(String),
    
    #[error("Template rendering failed: {source}")]
    Template {
        #[from]
        source: tera::Error,
    },
}

// ✅ CORRECT: Use anyhow for application-level error handling
pub async fn run_command(args: Args) -> anyhow::Result<()> {
    let config = Config::load().context("Failed to load dx configuration")?;
    ...
}
```

#### 3. Async/Await Patterns

```rust
// ✅ CORRECT: Use tokio for async runtime
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dx_cli::run().await
}

// ✅ CORRECT: Use async traits with async-trait crate
#[async_trait::async_trait]
pub trait AiProvider: Send + Sync {
    async fn complete(&self, prompt: &str) -> Result<String>;
    async fn chat(&self, messages: &[Message]) -> Result<Response>;
}

// ✅ CORRECT: Use tokio::spawn for concurrent tasks
let handles: Vec<_> = tasks
    .into_iter()
    .map(|task| tokio::spawn(async move { task.execute().await }))
    .collect();

let results = futures::future::join_all(handles).await;
```

#### 4. Type Safety

```rust
// ✅ CORRECT: Use newtypes for domain concepts
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ComponentId(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectPath(PathBuf);

// ✅ CORRECT: Use builder pattern for complex structs
#[derive(Debug, Builder)]
#[builder(setter(into))]
pub struct GeneratorConfig {
    template: String,
    output_dir: PathBuf,
    #[builder(default)]
    overwrite: bool,
    #[builder(default)]
    dry_run: bool,
}
```

#### 5. Zero-Cost Abstractions

```rust
// ✅ CORRECT: Use generics for zero-cost polymorphism
pub fn process<T: Command>(cmd: T) -> Result<T::Output> {
    cmd.validate()?;
    cmd.execute()
}

// ✅ CORRECT: Use iterators instead of collecting into Vec
pub fn filter_components<'a>(
    components: &'a [Component],
    predicate: impl Fn(&Component) -> bool + 'a,
) -> impl Iterator<Item = &'a Component> {
    components.iter().filter(move |c| predicate(c))
}
```

#### 6. Memory Efficiency

```rust
// ✅ CORRECT: Use Cow for flexible string handling
use std::borrow::Cow;

pub fn normalize_path<'a>(path: &'a str) -> Cow<'a, str> {
    if path.contains("..") {
        Cow::Owned(path.replace("..", ""))
    } else {
        Cow::Borrowed(path)
    }
}

// ✅ CORRECT: Use Arc for shared ownership across threads
use std::sync::Arc;

pub struct SharedConfig {
    inner: Arc<ConfigData>,
}
```

### Concurrency Patterns

```rust
// ✅ CORRECT: Use rayon for CPU-bound parallel processing
use rayon::prelude::*;

pub fn process_files_parallel(files: &[PathBuf]) -> Vec<Result<Output>> {
    files.par_iter()
        .map(|file| process_file(file))
        .collect()
}

// ✅ CORRECT: Use tokio for I/O-bound async operations
pub async fn fetch_components_async(ids: &[ComponentId]) -> Vec<Result<Component>> {
    let futures: Vec<_> = ids.iter()
        .map(|id| fetch_component(id))
        .collect();
    
    futures::future::join_all(futures).await
}

// ✅ CORRECT: Use channels for communication
use tokio::sync::mpsc;

pub async fn spawn_worker(rx: mpsc::Receiver<Task>) {
    while let Some(task) = rx.recv().await {
        task.process().await;
    }
}
```

---

## 📦 Recommended Crates

### Core CLI & TUI

| Crate | Version | Purpose | Priority |
|-------|---------|---------|----------|
| `clap` | `4.5+` | Command-line argument parsing with derive macros | **REQUIRED** |
| `clap_complete` | `4.5+` | Shell completion generation | **REQUIRED** |
| `ratatui` | `0.29+` | Terminal UI framework (successor to tui-rs) | **REQUIRED** |
| `crossterm` | `0.28+` | Cross-platform terminal manipulation | **REQUIRED** |
| `indicatif` | `0.17+` | Progress bars and spinners | **REQUIRED** |
| `console` | `0.15+` | Terminal styling and utilities | **REQUIRED** |
| `dialoguer` | `0.11+` | Interactive prompts and selections | **REQUIRED** |

### Async Runtime & Concurrency

| Crate | Version | Purpose | Priority |
|-------|---------|---------|----------|
| `tokio` | `1.40+` | Async runtime with full features | **REQUIRED** |
| `tokio-util` | `0.7+` | Additional tokio utilities | **REQUIRED** |
| `rayon` | `1.10+` | Data parallelism for CPU-bound tasks | **REQUIRED** |
| `futures` | `0.3+` | Future combinators and utilities | **REQUIRED** |
| `async-trait` | `0.1+` | Async trait methods | **REQUIRED** |
| `parking_lot` | `0.12+` | Faster synchronization primitives | RECOMMENDED |

### Serialization & Configuration

| Crate | Version | Purpose | Priority |
|-------|---------|---------|----------|
| `serde` | `1.0+` | Serialization framework | **REQUIRED** |
| `serde_json` | `1.0+` | JSON serialization | **REQUIRED** |
| `toml` | `0.8+` | TOML configuration parsing | **REQUIRED** |
| `serde_yaml` | `0.9+` | YAML support | **REQUIRED** |
| `config` | `0.14+` | Layered configuration management | **REQUIRED** |
| `figment` | `0.10+` | Configuration extraction | RECOMMENDED |

### HTTP & Networking

| Crate | Version | Purpose | Priority |
|-------|---------|---------|----------|
| `reqwest` | `0.12+` | HTTP client with async support | **REQUIRED** |
| `tokio-tungstenite` | `0.24+` | WebSocket client for real-time AI | **REQUIRED** |
| `hyper` | `1.4+` | Low-level HTTP (for custom servers) | RECOMMENDED |
| `tower` | `0.5+` | Service abstractions | RECOMMENDED |

### Error Handling & Logging

| Crate | Version | Purpose | Priority |
|-------|---------|---------|----------|
| `anyhow` | `1.0+` | Application error handling | **REQUIRED** |
| `thiserror` | `2.0+` | Library error definitions | **REQUIRED** |
| `tracing` | `0.1+` | Structured logging and diagnostics | **REQUIRED** |
| `tracing-subscriber` | `0.3+` | Log subscribers and formatters | **REQUIRED** |
| `tracing-appender` | `0.2+` | File appending for logs | RECOMMENDED |

### File System & Storage

| Crate | Version | Purpose | Priority |
|-------|---------|---------|----------|
| `walkdir` | `2.5+` | Recursive directory traversal | **REQUIRED** |
| `notify` | `6.1+` | File system watching | **REQUIRED** |
| `dirs` | `5.0+` | Platform-specific directories | **REQUIRED** |
| `tempfile` | `3.12+` | Temporary file handling | **REQUIRED** |
| `fs_extra` | `1.3+` | Extended file operations | RECOMMENDED |
| `glob` | `0.3+` | Glob pattern matching | **REQUIRED** |
| `ignore` | `0.4+` | Gitignore-style filtering | **REQUIRED** |

### Template Engine & Code Generation

| Crate | Version | Purpose | Priority |
|-------|---------|---------|----------|
| `tera` | `1.20+` | Jinja2-like template engine | **REQUIRED** |
| `handlebars` | `6.1+` | Alternative template engine | OPTIONAL |
| `minijinja` | `2.3+` | Lightweight Jinja templates | RECOMMENDED |
| `askama` | `0.12+` | Type-safe compile-time templates | RECOMMENDED |
| `syn` | `2.0+` | Rust code parsing (for codegen) | RECOMMENDED |
| `quote` | `1.0+` | Rust code generation | RECOMMENDED |
| `proc-macro2` | `1.0+` | Procedural macro utilities | RECOMMENDED |

### Database & Caching

| Crate | Version | Purpose | Priority |
|-------|---------|---------|----------|
| `sled` | `0.34+` | Embedded key-value database | **REQUIRED** |
| `rusqlite` | `0.32+` | SQLite bindings | RECOMMENDED |
| `cached` | `0.54+` | Caching utilities | **REQUIRED** |
| `moka` | `0.12+` | Concurrent cache | RECOMMENDED |

### Shell & Terminal Intelligence

| Crate | Version | Purpose | Priority |
|-------|---------|---------|----------|
| `rustyline` | `14.0+` | Readline implementation | **REQUIRED** |
| `rustyline-derive` | `0.10+` | Derive macros for rustyline | **REQUIRED** |
| `nucleo` | `0.5+` | High-performance fuzzy matching | **REQUIRED** |
| `fuzzy-matcher` | `0.3+` | Alternative fuzzy matching | OPTIONAL |
| `skim` | `0.10+` | Fuzzy finder library | OPTIONAL |
| `shell-words` | `1.1+` | Shell word splitting | **REQUIRED** |
| `shlex` | `1.3+` | Shell lexing | RECOMMENDED |

### AI & LLM Integration

| Crate | Version | Purpose | Priority |
|-------|---------|---------|----------|
| `reqwest` | `0.12+` | HTTP for AI API calls | **REQUIRED** |
| `async-openai` | `0.24+` | OpenAI-compatible API client (adapt for Friday) | REFERENCE |
| `tiktoken-rs` | `0.6+` | Token counting | RECOMMENDED |
| `tokenizers` | `0.20+` | HuggingFace tokenizers | OPTIONAL |
| `llm-chain` | `0.13+` | LLM chain abstractions | REFERENCE |

### Compression & Encoding

| Crate | Version | Purpose | Priority |
|-------|---------|---------|----------|
| `flate2` | `1.0+` | Gzip/Deflate compression | **REQUIRED** |
| `zip` | `2.2+` | ZIP archive handling | **REQUIRED** |
| `tar` | `0.4+` | TAR archive handling | **REQUIRED** |
| `base64` | `0.22+` | Base64 encoding | **REQUIRED** |
| `blake3` | `1.5+` | Fast hashing | **REQUIRED** |
| `sha2` | `0.10+` | SHA-256 hashing | **REQUIRED** |

### Testing & Development

| Crate | Version | Purpose | Priority |
|-------|---------|---------|----------|
| `tokio-test` | `0.4+` | Async test utilities | **REQUIRED** |
| `assert_cmd` | `2.0+` | Command-line testing | **REQUIRED** |
| `predicates` | `3.1+` | Test predicates | **REQUIRED** |
| `insta` | `1.40+` | Snapshot testing | **REQUIRED** |
| `proptest` | `1.5+` | Property-based testing | RECOMMENDED |
| `criterion` | `0.5+` | Benchmarking | **REQUIRED** |
| `fake` | `2.10+` | Fake data generation | RECOMMENDED |
| `mockall` | `0.13+` | Mocking framework | RECOMMENDED |
| `wiremock` | `0.6+` | HTTP mocking | RECOMMENDED |

### Utility Crates

| Crate | Version | Purpose | Priority |
|-------|---------|---------|----------|
| `once_cell` | `1.20+` | Lazy statics (std now has LazyLock) | **REQUIRED** |
| `lazy_static` | `1.5+` | Lazy initialization | OPTIONAL |
| `derive_builder` | `0.20+` | Builder pattern derive | **REQUIRED** |
| `derive_more` | `1.0+` | Additional derive macros | RECOMMENDED |
| `strum` | `0.26+` | Enum utilities | **REQUIRED** |
| `itertools` | `0.13+` | Iterator extensions | **REQUIRED** |
| `chrono` | `0.4+` | Date and time handling | **REQUIRED** |
| `uuid` | `1.10+` | UUID generation | **REQUIRED** |
| `regex` | `1.11+` | Regular expressions | **REQUIRED** |
| `url` | `2.5+` | URL parsing | **REQUIRED** |
| `semver` | `1.0+` | Semantic versioning | **REQUIRED** |
| `humantime` | `2.1+` | Human-readable durations | RECOMMENDED |
| `bytesize` | `1.3+` | Human-readable byte sizes | RECOMMENDED |
| `which` | `6.0+` | Find executables in PATH | **REQUIRED** |

### LSP & Language Integration

| Crate | Version | Purpose | Priority |
|-------|---------|---------|----------|
| `tower-lsp` | `0.20+` | LSP server implementation | **REQUIRED** |
| `lsp-types` | `0.97+` | LSP type definitions | **REQUIRED** |
| `tree-sitter` | `0.23+` | Incremental parsing | RECOMMENDED |

---

## 📁 Project Structure

### Copilot MUST generate this directory structure:

```
dx-cli/
├── Cargo.toml                    # Workspace root
├── Cargo.lock
├── rust-toolchain.toml           # Rust version pinning
├── .cargo/
│   └── config.toml               # Cargo configuration
├── deny.toml                     # cargo-deny configuration
├── clippy.toml                   # Clippy configuration
├── rustfmt.toml                  # Rustfmt configuration
├── .github/
│   ├── copilot-instructions.md   # This file
│   ├── workflows/
│   │   ├── ci.yml                # CI pipeline
│   │   ├── release.yml           # Release pipeline
│   │   └── security.yml          # Security audits
│   └── CODEOWNERS
├── docs/
│   ├── architecture.md
│   ├── CONTRIBUTING.md
│   ├── CHANGELOG.md
│   └── api/
├── assets/
│   ├── banner.txt                # ASCII art banner
│   └── templates/                # Built-in templates
├── benches/
│   └── performance.rs
├── tests/
│   ├── integration/
│   └── e2e/
│
├── crates/
│   ├── dx-cli/                   # Main CLI binary crate
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs
│   │       ├── lib.rs
│   │       ├── cli/
│   │       │   ├── mod.rs
│   │       │   ├── args.rs       # Clap argument definitions
│   │       │   ├── commands/     # Command implementations
│   │       │   │   ├── mod.rs
│   │       │   │   ├── ui.rs
│   │       │   │   ├── style.rs
│   │       │   │   ├── icon.rs
│   │       │   │   ├── font.rs
│   │       │   │   ├── auth.rs
│   │       │   │   ├── media.rs
│   │       │   │   ├── i18n.rs
│   │       │   │   ├── forge.rs
│   │       │   │   ├── generate.rs
│   │       │   │   ├── chat.rs
│   │       │   │   ├── agent.rs
│   │       │   │   └── config.rs
│   │       │   └── completions.rs
│   │       └── banner.rs
│   │
│   ├── dx-core/                  # Core shared functionality
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── config/
│   │       │   ├── mod.rs
│   │       │   ├── loader.rs
│   │       │   ├── schema.rs
│   │       │   └── validation.rs
│   │       ├── error.rs
│   │       ├── types.rs
│   │       └── utils.rs
│   │
│   ├── dx-tui/                   # Terminal UI components
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── app.rs
│   │       ├── components/
│   │       │   ├── mod.rs
│   │       │   ├── banner.rs
│   │       │   ├── chat.rs
│   │       │   ├── command_palette.rs
│   │       │   ├── file_browser.rs
│   │       │   ├── progress.rs
│   │       │   └── status_bar.rs
│   │       ├── layouts/
│   │       ├── themes/
│   │       └── events.rs
│   │
│   ├── dx-ai/                    # Friday AI integration
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── friday/           # Friday AI client
│   │       │   ├── mod.rs
│   │       │   ├── client.rs
│   │       │   ├── models.rs
│   │       │   └── streaming.rs
│   │       ├── agents/
│   │       │   ├── mod.rs
│   │       │   ├── orchestrator.rs
│   │       │   ├── executor.rs
│   │       │   └── memory.rs
│   │       ├── dxp/              # DX Protocol
│   │       │   ├── mod.rs
│   │       │   ├── protocol.rs
│   │       │   ├── tools.rs
│   │       │   ├── resources.rs
│   │       │   └── transport.rs
│   │       └── context/
│   │           ├── mod.rs
│   │           └── manager.rs
│   │
│   ├── dx-generator/             # Template & code generation
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── engine.rs
│   │       ├── templates/
│   │       │   ├── mod.rs
│   │       │   ├── registry.rs
│   │       │   └── loader.rs
│   │       ├── generators/
│   │       │   ├── mod.rs
│   │       │   ├── component.rs
│   │       │   ├── project.rs
│   │       │   ├── readme.rs
│   │       │   ├── test.rs
│   │       │   └── docs.rs
│   │       └── snippets/
│   │
│   ├── dx-shell/                 # Shell enhancements
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── autocomplete/
│   │       │   ├── mod.rs
│   │       │   ├── engine.rs
│   │       │   ├── fuzzy.rs
│   │       │   └── suggestions.rs
│   │       ├── memory/
│   │       │   ├── mod.rs
│   │       │   ├── history.rs
│   │       │   └── context.rs
│   │       ├── hooks/
│   │       │   ├── mod.rs
│   │       │   ├── bash.rs
│   │       │   ├── zsh.rs
│   │       │   ├── fish.rs
│   │       │   └── powershell.rs
│   │       └── intelligence/
│   │
│   ├── dx-forge/                 # VCS & orchestration
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── vcs/
│   │       │   ├── mod.rs
│   │       │   ├── repository.rs
│   │       │   ├── commits.rs
│   │       │   ├── branches.rs
│   │       │   └── merge.rs
│   │       ├── orchestrator/
│   │       │   ├── mod.rs
│   │       │   ├── pipeline.rs
│   │       │   └── scheduler.rs
│   │       └── package/
│   │
│   ├── dx-style/                 # CSS generation
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── parser.rs
│   │       ├── compiler.rs
│   │       └── optimizer.rs
│   │
│   ├── dx-ui/                    # UI components
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── components/
│   │       ├── registry.rs
│   │       └── installer.rs
│   │
│   ├── dx-icons/                 # Icon management
│   │   ├── Cargo.toml
│   │   └── src/
│   │
│   ├── dx-fonts/                 # Font management
│   │   ├── Cargo.toml
│   │   └── src/
│   │
│   ├── dx-auth/                  # Authentication
│   │   ├── Cargo.toml
│   │   └── src/
│   │
│   ├── dx-media/                 # Media processing
│   │   ├── Cargo.toml
│   │   └── src/
│   │
│   ├── dx-i18n/                  # Internationalization
│   │   ├── Cargo.toml
│   │   └── src/
│   │
│   └── dx-lsp/                   # LSP server
│       ├── Cargo.toml
│       └── src/
│
└── xtask/                        # Build automation
    ├── Cargo.toml
    └── src/
        └── main.rs
```

### Workspace Cargo.toml Template

```toml
[workspace]
resolver = "2"
members = [
    "crates/*",
    "xtask",
]

[workspace.package]
version = "0.1.0"
edition = "2021"
rust-version = "1.80"
license = "MIT OR Apache-2.0"
repository = "https://github.com/dx-platform/dx-cli"
homepage = "https://dx.dev"
documentation = "https://docs.dx.dev"
authors = ["Dx Team"]
keywords = ["cli", "developer-tools", "ai", "productivity"]
categories = ["command-line-utilities", "development-tools"]

[workspace.dependencies]
# Internal crates
dx-core = { path = "crates/dx-core" }
dx-cli = { path = "crates/dx-cli" }
dx-tui = { path = "crates/dx-tui" }
dx-ai = { path = "crates/dx-ai" }
dx-generator = { path = "crates/dx-generator" }
dx-shell = { path = "crates/dx-shell" }
dx-forge = { path = "crates/dx-forge" }
dx-style = { path = "crates/dx-style" }
dx-ui = { path = "crates/dx-ui" }
dx-icons = { path = "crates/dx-icons" }
dx-fonts = { path = "crates/dx-fonts" }
dx-auth = { path = "crates/dx-auth" }
dx-media = { path = "crates/dx-media" }
dx-i18n = { path = "crates/dx-i18n" }
dx-lsp = { path = "crates/dx-lsp" }

# CLI & TUI
clap = { version = "4.5", features = ["derive", "env", "unicode", "wrap_help"] }
clap_complete = "4.5"
ratatui = { version = "0.29", features = ["all-widgets"] }
crossterm = { version = "0.28", features = ["event-stream"] }
indicatif = { version = "0.17", features = ["rayon", "tokio"] }
console = "0.15"
dialoguer = { version = "0.11", features = ["fuzzy-select"] }

# Async
tokio = { version = "1.40", features = ["full", "tracing"] }
tokio-util = { version = "0.7", features = ["full"] }
futures = "0.3"
async-trait = "0.1"
rayon = "1.10"

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
toml = "0.8"
serde_yaml = "0.9"

# Error handling
anyhow = "1.0"
thiserror = "2.0"

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
tracing-appender = "0.2"

# HTTP
reqwest = { version = "0.12", features = ["json", "stream", "gzip", "brotli"] }
tokio-tungstenite = { version = "0.24", features = ["native-tls"] }

# File system
walkdir = "2.5"
notify = { version = "6.1", features = ["serde"] }
dirs = "5.0"
tempfile = "3.12"
glob = "0.3"
ignore = "0.4"

# Templates
tera = "1.20"
minijinja = { version = "2.3", features = ["loader"] }

# Database
sled = "0.34"
rusqlite = { version = "0.32", features = ["bundled"] }

# Shell
rustyline = { version = "14.0", features = ["derive"] }
nucleo = "0.5"
shell-words = "1.1"

# Utilities
once_cell = "1.20"
derive_builder = "0.20"
strum = { version = "0.26", features = ["derive"] }
itertools = "0.13"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.10", features = ["v4", "v7", "serde"] }
regex = "1.11"
url = "2.5"
semver = { version = "1.0", features = ["serde"] }
which = "6.0"

# Compression
flate2 = "1.0"
zip = "2.2"
tar = "0.4"
base64 = "0.22"
blake3 = "1.5"

# Testing
tokio-test = "0.4"
assert_cmd = "2.0"
predicates = "3.1"
insta = { version = "1.40", features = ["yaml"] }
criterion = { version = "0.5", features = ["async_tokio"] }

[profile.release]
lto = "thin"
codegen-units = 1
strip = true
panic = "abort"

[profile.dev]
opt-level = 0
debug = true

[profile.dev.package."*"]
opt-level = 3
```

---

## 🔧 Core Modules

### Module: CLI Entry Point (dx-cli)

```
Purpose: Main binary entry point, command routing, TUI rendering
Dependencies: clap, ratatui, crossterm, dx-core, dx-tui
Key Files:
  - main.rs: Entry point, async runtime setup
  - cli/args.rs: Command-line argument definitions
  - cli/commands/: Individual command handlers
  - banner.rs: ASCII art banner display
```

**Implementation Requirements:**

1. **Banner Display**: On startup, show ASCII art "Dx - Enhanced Development Experience"
2. **Command Structure**: Use clap's derive macros for type-safe argument parsing
3. **Subcommand Pattern**: Each Dx tool should be a subcommand (ui, style, icon, font, auth, media, i18n, forge, generate, chat, agent)
4. **Global Options**: Version, verbose, quiet, config path, color mode
5. **Shell Completions**: Generate completions for bash, zsh, fish, powershell

### Module: Terminal UI (dx-tui)

```
Purpose: Rich terminal user interface with ratatui
Dependencies: ratatui, crossterm, tokio
Key Files:
  - app.rs: Main application state machine
  - components/: Reusable UI components
  - layouts/: Screen layouts
  - events.rs: Event handling
```

**Implementation Requirements:**

1. **Responsive Layout**: Adapt to terminal size changes
2. **Component Library**: Banner, chat window, command palette, file browser, progress indicators
3. **Keyboard Navigation**: Full keyboard support with vim-like bindings
4. **Theme System**: Support for multiple color themes
5. **Async Rendering**: Non-blocking UI updates

### Module: AI Integration (dx-ai)

```
Purpose: Friday AI integration, agent orchestration, DXP protocol
Dependencies: reqwest, tokio, serde, tokio-tungstenite
Key Files:
  - friday/: Friday AI API client
  - agents/: Agent definitions and orchestrator
  - dxp/: DX Protocol implementation
  - context/: Context management for AI
```

**Implementation Requirements:**

1. **Friday Client**: HTTP/WebSocket client for Friday AI API
2. **Streaming Support**: Real-time streaming responses
3. **Agent System**: Define, execute, and orchestrate AI agents
4. **Memory Management**: Persistent context and conversation history
5. **DXP Protocol**: Implement DX Protocol for tool communication

### Module: Generator Engine (dx-generator)

```
Purpose: Template-based code and project generation
Dependencies: tera, minijinja, walkdir, serde
Key Files:
  - engine.rs: Core generation engine
  - templates/: Template management
  - generators/: Specific generators
```

**Implementation Requirements:**

1. **Template Engine**: Tera/MiniJinja for flexible templating
2. **Generator Types**: Component, project, readme, test, documentation
3. **Variable Interpolation**: Context-aware variable substitution
4. **Dry Run Mode**: Preview changes before applying
5. **Diff Display**: Show what will be generated/modified

### Module: Shell Enhancement (dx-shell)

```
Purpose: Intelligent shell features, autocomplete, history
Dependencies: rustyline, nucleo, sled
Key Files:
  - autocomplete/: Autocomplete engine
  - memory/: History and context storage
  - hooks/: Shell integration hooks
  - intelligence/: AI-powered suggestions
```

**Implementation Requirements:**

1. **Fuzzy Completion**: Fast fuzzy matching with nucleo
2. **History Search**: Searchable command history
3. **Context Awareness**: Suggest commands based on current directory/project
4. **Shell Hooks**: Integration scripts for bash, zsh, fish, powershell
5. **AI Suggestions**: Optional AI-powered command suggestions

---

## 🤖 AI Integration (Friday)

### Friday AI Client Design

Copilot MUST implement the Friday AI client following this design:

```rust
// NOTE: This is the interface design, not implementation
pub trait FridayClient: Send + Sync {
    /// Send a chat message and receive a response
    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse>;
    
    /// Stream a chat response in real-time
    async fn chat_stream(&self, request: ChatRequest) -> Result<impl Stream<Item = Result<ChatChunk>>>;
    
    /// Execute an agent task
    async fn execute_agent(&self, agent: AgentDefinition, task: Task) -> Result<AgentResult>;
    
    /// Get available models
    async fn list_models(&self) -> Result<Vec<Model>>;
}

pub struct ChatRequest {
    pub messages: Vec<Message>,
    pub model: Option<String>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub tools: Option<Vec<DxpTool>>,
}

pub struct Message {
    pub role: Role,
    pub content: Content,
}

pub enum Role {
    System,
    User,
    Assistant,
    Tool,
}

pub enum Content {
    Text(String),
    ToolCall(ToolCall),
    ToolResult(ToolResult),
}
```

### AI Memory System

```rust
// Persistent memory for AI context
pub trait AiMemory: Send + Sync {
    /// Store a conversation
    async fn store_conversation(&self, id: ConversationId, messages: &[Message]) -> Result<()>;
    
    /// Retrieve conversation history
    async fn get_conversation(&self, id: ConversationId) -> Result<Option<Vec<Message>>>;
    
    /// Store context (current project, files, etc.)
    async fn store_context(&self, context: &Context) -> Result<()>;
    
    /// Get relevant context for a query
    async fn get_relevant_context(&self, query: &str, limit: usize) -> Result<Vec<ContextChunk>>;
    
    /// Clear memory
    async fn clear(&self) -> Result<()>;
}
```

---

## 📡 DXP Protocol

### DX Protocol Specification

DXP (DX Protocol) is the communication protocol between Dx-CLI, AI agents, and external tools. It is designed to be:

1. **Faster than MCP**: Binary serialization, efficient transport
2. **Type-Safe**: Strong typing with Rust structs
3. **Extensible**: Plugin system for custom tools
4. **Bidirectional**: Two-way communication between client and tools

### DXP Message Types

```rust
// DXP Protocol Message Types
pub enum DxpMessage {
    // Tool discovery
    ListTools,
    ToolsResponse(Vec<ToolDefinition>),
    
    // Tool execution
    CallTool(ToolCall),
    ToolResult(ToolResult),
    ToolError(ToolError),
    
    // Resource management
    ListResources,
    ResourcesResponse(Vec<ResourceDefinition>),
    ReadResource(ResourceId),
    ResourceContent(ResourceContent),
    
    // Subscriptions
    Subscribe(Subscription),
    Notification(Notification),
    Unsubscribe(SubscriptionId),
    
    // Context
    UpdateContext(Context),
    ContextUpdated,
}

pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub input_schema: JsonSchema,
    pub output_schema: JsonSchema,
    pub capabilities: Vec<Capability>,
}

pub struct ToolCall {
    pub id: CallId,
    pub name: String,
    pub arguments: serde_json::Value,
}

pub struct ToolResult {
    pub call_id: CallId,
    pub output: serde_json::Value,
    pub metadata: Option<Metadata>,
}
```

### DXP Transport

```rust
// DXP supports multiple transports
pub trait DxpTransport: Send + Sync {
    async fn send(&self, message: DxpMessage) -> Result<()>;
    async fn receive(&self) -> Result<DxpMessage>;
    async fn close(&self) -> Result<()>;
}

// Implementations:
// - StdioTransport: For subprocess communication
// - TcpTransport: For network communication
// - WebSocketTransport: For browser/web clients
// - InProcessTransport: For in-process tools
```

---

## 🐚 Terminal Enhancement

### Shell Hook System

Copilot MUST generate shell hooks for all major shells:

```bash
# Bash hook example structure
dx_hook() {
    local cmd="$1"
    # Pre-command processing
    __dx_pre_cmd "$cmd"
    # Execute command
    eval "$cmd"
    local exit_code=$?
    # Post-command processing
    __dx_post_cmd "$cmd" "$exit_code"
    return $exit_code
}

# Initialize dx shell enhancements
__dx_init() {
    # Set up autocomplete
    # Set up history integration
    # Set up prompt customization
}
```

### Autocomplete Engine

```rust
// Autocomplete configuration
pub struct AutocompleteConfig {
    pub enabled: bool,
    pub fuzzy_threshold: f64,
    pub max_suggestions: usize,
    pub include_history: bool,
    pub include_files: bool,
    pub include_commands: bool,
    pub ai_suggestions: bool,
}

// Autocomplete sources
pub enum CompletionSource {
    History,
    FileSystem,
    Commands,
    DxTools,
    AiSuggestions,
    Custom(Box<dyn CompletionProvider>),
}
```

---

## 🔄 Generator/Templater System

### Template Structure

Templates should follow this structure:

```
templates/
├── components/
│   ├── react/
│   │   ├── button.tsx.tera
│   │   ├── accordion.tsx.tera
│   │   └── ...
│   ├── vue/
│   └── svelte/
├── projects/
│   ├── next-app/
│   │   ├── template.json
│   │   └── files/
│   └── rust-cli/
├── readme/
│   ├── basic.md.tera
│   └── detailed.md.tera
├── tests/
│   ├── unit.rs.tera
│   └── integration.rs.tera
└── docs/
    └── api.md.tera
```

### Generator Interface

```rust
pub trait Generator: Send + Sync {
    /// Get generator name
    fn name(&self) -> &str;
    
    /// Get generator description
    fn description(&self) -> &str;
    
    /// Get required variables
    fn variables(&self) -> Vec<Variable>;
    
    /// Generate output
    async fn generate(&self, context: &GeneratorContext) -> Result<GeneratorOutput>;
    
    /// Preview output (dry run)
    async fn preview(&self, context: &GeneratorContext) -> Result<GeneratorPreview>;
}

pub struct GeneratorOutput {
    pub files: Vec<GeneratedFile>,
    pub imports: Vec<Import>,
    pub dependencies: Vec<Dependency>,
    pub post_actions: Vec<PostAction>,
}

pub struct GeneratedFile {
    pub path: PathBuf,
    pub content: String,
    pub overwrite: bool,
}
```

### Usage Examples

```bash
# Generate a UI component
dx ui add accordion
dx ui add button --variant outline

# Generate project structure  
dx generate project next-app --name my-app

# Generate documentation
dx generate readme --template detailed

# Generate tests
dx generate test --type unit --for src/lib.rs
```

---

## 👥 Agent Orchestrator

### Agent Definition

```rust
pub struct AgentDefinition {
    pub id: AgentId,
    pub name: String,
    pub description: String,
    pub system_prompt: String,
    pub tools: Vec<ToolBinding>,
    pub capabilities: Vec<Capability>,
    pub constraints: AgentConstraints,
}

pub struct AgentConstraints {
    pub max_steps: u32,
    pub max_tokens: u32,
    pub allowed_tools: Option<Vec<String>>,
    pub denied_tools: Option<Vec<String>>,
    pub require_approval: Vec<String>,
}
```

### Orchestrator Design

```rust
pub trait AgentOrchestrator: Send + Sync {
    /// Register an agent
    async fn register_agent(&self, agent: AgentDefinition) -> Result<()>;
    
    /// Execute a task with an agent
    async fn execute(&self, agent_id: &AgentId, task: Task) -> Result<TaskResult>;
    
    /// Execute with multiple agents (delegation)
    async fn execute_multi(&self, task: Task, agents: &[AgentId]) -> Result<TaskResult>;
    
    /// Stream execution progress
    async fn execute_stream(&self, agent_id: &AgentId, task: Task) 
        -> Result<impl Stream<Item = ExecutionEvent>>;
    
    /// Cancel a running task
    async fn cancel(&self, task_id: &TaskId) -> Result<()>;
}

pub enum ExecutionEvent {
    Started { task_id: TaskId },
    Step { step: u32, action: Action },
    ToolCall { tool: String, args: Value },
    ToolResult { result: Value },
    Thinking { content: String },
    Response { content: String },
    Completed { result: TaskResult },
    Error { error: String },
}
```

---

## ⚙️ Configuration System

### Single Configuration File

Dx-CLI uses a single configuration file for all tools:

```toml
# dx.toml - Single configuration for all Dx tools

[dx]
version = "0.1.0"

[cli]
color = "auto"  # auto, always, never
verbose = false
parallel_jobs = 4

[ai]
provider = "friday"
model = "friday-pro"
temperature = 0.7
max_tokens = 4096
memory_enabled = true
memory_path = ".dx/memory"

[shell]
autocomplete = true
fuzzy_threshold = 0.6
history_size = 10000
ai_suggestions = true

[style]
config_path = "src/styles"
output_path = "dist/styles"
minify = true

[ui]
components_path = "src/components"
framework = "react"  # react, vue, svelte

[icons]
sets = ["lucide", "heroicons"]
output_format = "svg"

[fonts]
output_path = "public/fonts"
formats = ["woff2", "woff"]

[auth]
providers = ["google", "github"]
output_path = "src/auth"

[media]
input_path = "assets"
output_path = "public/media"
optimize = true
quality = 85

[i18n]
source_locale = "en"
locales = ["en", "es", "fr", "de", "ja", "zh"]
output_path = "locales"
tts_enabled = true

[forge]
auto_commit = false
branch_prefix = "dx/"
```

### Configuration Loading

```rust
// Configuration loading with environment override
pub struct ConfigLoader {
    paths: Vec<PathBuf>,
    env_prefix: String,
}

impl ConfigLoader {
    pub fn load(&self) -> Result<DxConfig> {
        // 1. Load defaults
        // 2. Load from dx.toml/dx.json/dx.yaml/.dx
        // 3. Override with environment variables (DX_*)
        // 4. Override with CLI arguments
    }
}
```

---

## ❌ Error Handling Strategy

### Error Types

```rust
// Core error type using thiserror
#[derive(Debug, thiserror::Error)]
pub enum DxError {
    #[error("Configuration error: {message}")]
    Config { message: String, path: Option<PathBuf> },
    
    #[error("AI service error: {0}")]
    Ai(#[from] AiError),
    
    #[error("Template error: {0}")]
    Template(#[from] tera::Error),
    
    #[error("File system error: {path}")]
    FileSystem { 
        path: PathBuf, 
        #[source] 
        source: std::io::Error 
    },
    
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    
    #[error("DXP protocol error: {0}")]
    Dxp(String),
    
    #[error("Command failed: {command}")]
    Command { command: String, exit_code: i32 },
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Not found: {resource_type} '{name}'")]
    NotFound { resource_type: String, name: String },
}

// Result type alias
pub type DxResult<T> = Result<T, DxError>;
```

### Error Handling Patterns

```rust
// ✅ Use context for better error messages
let config = Config::load()
    .context("Failed to load dx configuration")?;

// ✅ Use specific error variants
if !path.exists() {
    return Err(DxError::NotFound {
        resource_type: "template".into(),
        name: path.display().to_string(),
    });
}

// ✅ Chain errors properly
fn load_template(name: &str) -> DxResult<Template> {
    let path = find_template(name)?;
    let content = std::fs::read_to_string(&path)
        .map_err(|e| DxError::FileSystem { path: path.clone(), source: e })?;
    Template::parse(&content)
        .map_err(|e| DxError::Template(e))
}
```

---

## 🧪 Testing Guidelines

### Test Organization

```
tests/
├── unit/           # Unit tests (in src/ files)
├── integration/    # Integration tests
│   ├── cli_test.rs
│   ├── generator_test.rs
│   └── ai_test.rs
├── e2e/           # End-to-end tests
│   └── workflows_test.rs
└── fixtures/      # Test fixtures
    ├── configs/
    └── templates/
```

### Testing Patterns

```rust
// Unit test example
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_command() {
        let cmd = parse_command("dx ui add accordion").unwrap();
        assert_eq!(cmd.tool, "ui");
        assert_eq!(cmd.action, "add");
        assert_eq!(cmd.target, "accordion");
    }
    
    #[tokio::test]
    async fn test_async_operation() {
        let result = async_operation().await;
        assert!(result.is_ok());
    }
}

// Integration test example
#[test]
fn test_cli_ui_add() {
    let mut cmd = Command::cargo_bin("dx").unwrap();
    cmd.args(["ui", "add", "button", "--dry-run"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Would create"));
}

// Snapshot test example
#[test]
fn test_generate_component() {
    let output = generate_component("Button", &Default::default()).unwrap();
    insta::assert_snapshot!(output);
}
```

### Benchmark Guidelines

```rust
use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark_template_rendering(c: &mut Criterion) {
    let engine = TemplateEngine::new();
    let context = create_test_context();
    
    c.bench_function("render_component", |b| {
        b.iter(|| engine.render("button", &context))
    });
}

criterion_group!(benches, benchmark_template_rendering);
criterion_main!(benches);
```

---

## ⚡ Performance Optimization

### Optimization Rules

1. **Lazy Loading**: Load components/templates only when needed
2. **Caching**: Cache parsed templates, configs, and AI responses
3. **Parallelism**: Use rayon for CPU-bound, tokio for I/O-bound
4. **Memory Efficiency**: Use Cow, Arc, avoid unnecessary clones
5. **Binary Size**: Use LTO, strip symbols, panic=abort in release

### Performance Patterns

```rust
// Lazy initialization
use once_cell::sync::Lazy;

static CONFIG: Lazy<Config> = Lazy::new(|| {
    Config::load().expect("Failed to load config")
});

// Efficient caching
use moka::future::Cache;

pub struct TemplateCache {
    cache: Cache<String, Arc<Template>>,
}

impl TemplateCache {
    pub async fn get_or_load(&self, name: &str) -> Arc<Template> {
        self.cache.get_with(name.to_string(), async {
            Arc::new(load_template(name).await.unwrap())
        }).await
    }
}

// Parallel processing
use rayon::prelude::*;

pub fn process_files(files: &[PathBuf]) -> Vec<Result<Output>> {
    files.par_iter()
        .map(|f| process_file(f))
        .collect()
}
```

---

## 🔒 Security Guidelines

### Security Requirements

1. **Input Validation**: Validate all user input
2. **Path Traversal**: Prevent directory traversal attacks
3. **Dependency Audit**: Regular `cargo audit`
4. **Secrets Management**: Never log secrets, use secure storage
5. **Sandboxing**: Isolate agent execution when possible

### Security Patterns

```rust
// Path validation
use std::path::{Path, PathBuf};

pub fn safe_path(base: &Path, user_path: &str) -> Result<PathBuf> {
    let full_path = base.join(user_path);
    let canonical = full_path.canonicalize()?;
    
    if !canonical.starts_with(base) {
        return Err(DxError::Validation(
            "Path traversal attempt detected".into()
        ));
    }
    
    Ok(canonical)
}

// Input sanitization
pub fn sanitize_command(input: &str) -> String {
    input.chars()
        .filter(|c| c.is_alphanumeric() || " -_./".contains(*c))
        .collect()
}

// Secure token storage
use keyring::Entry;

pub fn store_token(service: &str, token: &str) -> Result<()> {
    let entry = Entry::new(service, "dx-cli")?;
    entry.set_password(token)?;
    Ok(())
}
```

---

## 📝 Documentation Standards

### Documentation Requirements

1. **Module Docs**: Every module must have `//!` documentation
2. **Public APIs**: Every public function/struct must be documented
3. **Examples**: Include examples in doc comments
4. **Error Cases**: Document possible errors

### Documentation Template

```rust
//! # Dx Generator
//! 
//! Template-based code generation engine for Dx-CLI.
//! 
//! ## Overview
//! 
//! The generator module provides functionality for generating:
//! - UI components
//! - Project structures
//! - Documentation
//! - Test files
//! 
//! ## Example
//! 
//! ```rust
//! use dx_generator::Generator;
//! 
//! let generator = Generator::new();
//! let output = generator.generate("button", &context)?;
//! ```

/// Generates code from templates.
/// 
/// # Arguments
/// 
/// * `template_name` - Name of the template to use
/// * `context` - Variables for template rendering
/// 
/// # Returns
/// 
/// Returns `Ok(GeneratorOutput)` on success, or `Err(DxError)` if:
/// - Template is not found
/// - Template rendering fails
/// - Output directory is not writable
/// 
/// # Example
/// 
/// ```rust
/// let output = generator.generate("accordion", &context)?;
/// for file in output.files {
///     println!("Generated: {}", file.path.display());
/// }
/// ```
pub fn generate(&self, template_name: &str, context: &Context) -> DxResult<GeneratorOutput> {
    // Implementation
}
```

---

## 🎨 Code Style Guide

### Formatting (rustfmt.toml)

```toml
edition = "2021"
max_width = 100
tab_spaces = 4
use_small_heuristics = "Default"
imports_granularity = "Module"
group_imports = "StdExternalCrate"
reorder_imports = true
reorder_modules = true
format_code_in_doc_comments = true
format_strings = true
```

### Linting (clippy.toml)

```toml
cognitive-complexity-threshold = 25
too-many-arguments-threshold = 7
type-complexity-threshold = 250
```

### Clippy Lints

```rust
// In lib.rs or main.rs
#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate)]
```

### Naming Conventions

| Item | Convention | Example |
|------|------------|---------|
| Crates | snake_case | `dx_generator` |
| Modules | snake_case | `template_engine` |
| Types | PascalCase | `GeneratorConfig` |
| Traits | PascalCase | `Generator` |
| Functions | snake_case | `generate_component` |
| Constants | SCREAMING_SNAKE | `MAX_RETRIES` |
| Enum Variants | PascalCase | `Success`, `Error` |

---

## 📦 Release and Distribution

### Release Checklist

1. Update version in all Cargo.toml files
2. Update CHANGELOG.md
3. Run full test suite
4. Run clippy and fix warnings
5. Build release binaries for all platforms
6. Create GitHub release with binaries
7. Publish to crates.io

### Cross-Platform Build

```yaml
# Build targets
targets:
  - x86_64-unknown-linux-gnu
  - x86_64-unknown-linux-musl
  - aarch64-unknown-linux-gnu
  - x86_64-apple-darwin
  - aarch64-apple-darwin
  - x86_64-pc-windows-msvc
```

### Binary Size Optimization

```toml
[profile.release]
lto = "fat"
codegen-units = 1
strip = true
panic = "abort"
opt-level = "z"  # Optimize for size

[profile.release.build-override]
opt-level = 3
```

---

## 📚 Additional Resources

### Reference Projects

- [ripgrep](https://github.com/BurntSushi/ripgrep) - CLI best practices
- [starship](https://github.com/starship/starship) - Shell integration
- [zellij](https://github.com/zellij-org/zellij) - TUI architecture
- [helix](https://github.com/helix-editor/helix) - LSP integration
- [nushell](https://github.com/nushell/nushell) - Shell design

### Learning Resources

- [Rust CLI Book](https://rust-cli.github.io/book/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Ratatui Documentation](https://ratatui.rs/)

---

## ⚠️ Important Notes for Copilot

1. **Always use the latest stable crate versions** as specified in this document
2. **Follow the layered architecture** strictly
3. **Use async/await for all I/O operations**
4. **Implement proper error handling** with context
5. **Write tests for all public functions**
6. **Document all public APIs**
7. **Use the type system to prevent errors** (newtype pattern, Option, Result)
8. **Prefer composition over inheritance**
9. **Keep functions small and focused**
10. **Use meaningful variable and function names**

---

## 🎯 MVP Features Priority

### Phase 1: Core CLI (Week 1-2)
- [ ] Project structure setup
- [ ] CLI argument parsing with clap
- [ ] Banner display
- [ ] Basic command routing
- [ ] Configuration loading

### Phase 2: Shell Integration (Week 3-4)
- [ ] Autocomplete engine
- [ ] Shell hooks (bash, zsh, fish)
- [ ] History management
- [ ] Fuzzy search

### Phase 3: AI Integration (Week 5-6)
- [ ] Friday AI client
- [ ] Chat interface (TUI)
- [ ] Context management
- [ ] Memory persistence

### Phase 4: Generator (Week 7-8)
- [ ] Template engine
- [ ] Component generator
- [ ] Project generator
- [ ] Documentation generator

### Phase 5: DXP & Agents (Week 9-10)
- [ ] DXP protocol
- [ ] Agent definitions
- [ ] Orchestrator
- [ ] Tool integration

---

*This document should be updated as the project evolves. Last updated: 2025*
