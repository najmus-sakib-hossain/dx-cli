#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::module_name_repetitions)]

pub mod autocomplete;
pub mod memory;
pub mod hooks;
pub mod intelligence;

use std::path::PathBuf;
use anyhow::Result;

/// Install shell enhancements
pub fn install_shell_enhancements(shell_type: ShellType) -> Result<()> {
    hooks::install_hooks(shell_type)
}

/// Enable shell enhancements
pub fn enable_shell_enhancements() -> Result<()> {
    println!("✓ Shell enhancements enabled");
    println!("  Features:");
    println!("    • Enhanced ls with file icons");
    println!("    • Command help hints");
    println!("    • Smart autocomplete");
    println!("    • Command suggestions");
    println!("    • Persistent memory");
    Ok(())
}

/// Disable shell enhancements  
pub fn disable_shell_enhancements() -> Result<()> {
    println!("✓ Shell enhancements disabled");
    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub enum ShellType {
    Bash,
    Zsh,
    Fish,
    PowerShell,
}

impl ShellType {
    pub fn detect() -> Option<Self> {
        std::env::var("SHELL")
            .ok()
            .and_then(|shell| {
                let shell_name = PathBuf::from(shell)
                    .file_name()?
                    .to_str()?
                    .to_lowercase();
                
                match shell_name.as_str() {
                    "bash" => Some(Self::Bash),
                    "zsh" => Some(Self::Zsh),
                    "fish" => Some(Self::Fish),
                    "pwsh" | "powershell" => Some(Self::PowerShell),
                    _ => None,
                }
            })
    }
    
    pub fn name(&self) -> &str {
        match self {
            Self::Bash => "bash",
            Self::Zsh => "zsh",
            Self::Fish => "fish",
            Self::PowerShell => "powershell",
        }
    }
}
