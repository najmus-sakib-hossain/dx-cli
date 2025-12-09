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
    // Detect shell type and install hooks
    if let Some(shell_type) = ShellType::detect() {
        hooks::install_hooks(shell_type)?;
    } else {
        // Fallback: install for bash
        hooks::install_hooks(ShellType::Bash)?;
    }
    
    println!();
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
        // Try multiple detection methods for cross-platform support
        
        // Method 1: Check SHELL environment variable (Unix-like systems)
        if let Ok(shell) = std::env::var("SHELL") {
            let shell_name = PathBuf::from(&shell)
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_lowercase();
            
            match shell_name.as_str() {
                "bash" => return Some(Self::Bash),
                "zsh" => return Some(Self::Zsh),
                "fish" => return Some(Self::Fish),
                "pwsh" | "powershell" => return Some(Self::PowerShell),
                _ => {}
            }
        }
        
        // Method 2: Check for PowerShell on Windows
        if cfg!(windows) {
            if std::env::var("PSModulePath").is_ok() || 
               std::env::var("POWERSHELL_DISTRIBUTION_CHANNEL").is_ok() {
                return Some(Self::PowerShell);
            }
        }
        
        // Method 3: Check common shell indicators
        if std::env::var("BASH_VERSION").is_ok() || 
           std::env::var("BASH").is_ok() ||
           std::env::var("BASH_SOURCE").is_ok() {
            return Some(Self::Bash);
        }
        
        if std::env::var("ZSH_VERSION").is_ok() || 
           std::env::var("ZSH_NAME").is_ok() {
            return Some(Self::Zsh);
        }
        
        if std::env::var("FISH_VERSION").is_ok() || 
           std::env::var("fish_pid").is_ok() {
            return Some(Self::Fish);
        }
        
        // Method 4: Platform-specific defaults
        if cfg!(windows) {
            // Default to PowerShell on Windows
            return Some(Self::PowerShell);
        }
        
        // Method 5: Check parent process (fallback)
        #[cfg(unix)]
        {
            if let Ok(output) = std::process::Command::new("ps")
                .args(["-p", &std::os::unix::process::parent_id().to_string(), "-o", "comm="])
                .output()
            {
                if let Ok(parent) = String::from_utf8(output.stdout) {
                    let parent = parent.trim().to_lowercase();
                    if parent.contains("bash") {
                        return Some(Self::Bash);
                    } else if parent.contains("zsh") {
                        return Some(Self::Zsh);
                    } else if parent.contains("fish") {
                        return Some(Self::Fish);
                    }
                }
            }
        }
        
        // Default to Bash as most compatible fallback
        Some(Self::Bash)
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
