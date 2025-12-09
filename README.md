# Dx-CLI

Dx-CLI is the Rust-first command-line interface for the Dx platform. This repository now ships a full workspace scaffold with dedicated crates for CLI, TUI, AI (Friday), generators, shell intelligence, and supporting toolkits. The goal: **"every command should feel magical - fast, intelligent, and transparent."**

## Quick start

- Install Rust 1.80+ (pinned via `rust-toolchain.toml`).
- Build the workspace:
	- `cargo build`
- Run the CLI:
	- `cargo run -p dx-cli -- --help`
- Generate completions:
	- `j`

## Workspace layout

- `crates/dx-cli` — main CLI entrypoint (clap-based) + banner + command routing.
- `crates/dx-tui` — ratatui/crossterm TUI shell components.
- `crates/dx-core` — shared types, config loading, error model.
- `crates/dx-ai` — Friday client traits, agents, and DXP protocol shapes.
- `crates/dx-generator` — template/generator interfaces.
- `crates/dx-shell` — shell hooks, autocomplete config, memory.
- `crates/dx-forge` — VCS/package orchestration stubs.
- `crates/dx-style`, `dx-ui`, `dx-icons`, `dx-fonts`, `dx-auth`, `dx-media`, `dx-i18n`, `dx-lsp` — placeholders for tool-specific logic.
- `xtask` — build/automation helpers.

Supporting files:
- `assets/banner.txt` — startup banner displayed by the CLI.
- `.github/copilot-instructions.md` — detailed build guidance for agents.
- `clippy.toml`, `rustfmt.toml`, `deny.toml`, `.cargo/config.toml` — lint/format/audit defaults.

## Layered architecture

- Presentation: `dx-cli`, `dx-tui`
- Application: command dispatch and workflows (CLI modules)
- Domain: `dx-core`, `dx-ai`, `dx-generator`, `dx-shell`, `dx-forge`
- Infrastructure: transports, filesystem, HTTP clients (stubs ready for expansion)

## Development notes

- Use async/await for I/O; tokio is enabled workspace-wide.
- Favor `thiserror` in libraries and `anyhow` in binaries.
- Keep functions small, type-safe (newtypes/aliases), and avoid unnecessary clones.
- Tests are not yet implemented; add unit/integration coverage as features land.

## Next steps

- Flesh out command handlers for each Dx tool crate.
- Implement Friday AI client backed by real transport.
- Wire DXP protocol to transports and agent orchestrator.
- Expand TUI layouts/themes and streaming chat.
- Add CI (fmt, clippy, tests) and release pipelines.
