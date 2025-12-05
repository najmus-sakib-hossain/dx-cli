# Dx-CLI Architecture

This document summarizes the layered approach for Dx-CLI.

- Presentation: CLI/TUI crates (`dx-cli`, `dx-tui`)
- Application: Command handlers, workflows
- Domain: Core models (`dx-core`, `dx-ai`, `dx-generator`, `dx-shell`, `dx-forge`)
- Infrastructure: File system, network, AI transports

Further details can be expanded as features land.
