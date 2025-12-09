use anyhow::{Context, Result};
use std::fs;

use crate::ShellType;

const BASH_INIT: &str = include_str!("dx-shell-init.sh");
const BASH_COMMANDS: &str = include_str!("dx-shell-commands.sh");
const BASH_MEMORY: &str = include_str!("dx-shell-memory.sh");
const INSTALL_SCRIPT: &str = include_str!("install.sh");
const DEMO_SCRIPT: &str = include_str!("demo.sh");

pub fn install_hooks(shell_type: ShellType) -> Result<()> {
    let home = dirs::home_dir().context("Could not find home directory")?;
    let dx_dir = home.join(".dx");
    let shell_dir = dx_dir.join("shell");
    
    // Create directories
    fs::create_dir_all(&shell_dir)?;
    fs::create_dir_all(dx_dir.join("history"))?;
    
    // Write shell scripts
    fs::write(shell_dir.join("dx-shell-init.sh"), BASH_INIT)?;
    fs::write(shell_dir.join("dx-shell-commands.sh"), BASH_COMMANDS)?;
    fs::write(shell_dir.join("dx-shell-memory.sh"), BASH_MEMORY)?;
    fs::write(shell_dir.join("install.sh"), INSTALL_SCRIPT)?;
    fs::write(shell_dir.join("demo.sh"), DEMO_SCRIPT)?;
    
    // Make scripts executable on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        for script in &["install.sh", "demo.sh"] {
            let path = shell_dir.join(script);
            let mut perms = fs::metadata(&path)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&path, perms)?;
        }
    }
    
    println!("✓ Shell enhancement scripts installed to: {}", shell_dir.display());
    println!();
    println!("To complete installation, run:");
    println!("  bash ~/.dx/shell/install.sh");
    println!();
    println!("Or manually add to your shell rc file:");
    
    match shell_type {
        ShellType::Bash => {
            println!("  echo 'source ~/.dx/shell/dx-shell-init.sh' >> ~/.bashrc");
        }
        ShellType::Zsh => {
            println!("  echo 'source ~/.dx/shell/dx-shell-init.sh' >> ~/.zshrc");
        }
        ShellType::Fish => {
            println!("  echo 'source ~/.dx/shell/dx-shell-init.sh' >> ~/.config/fish/config.fish");
        }
        ShellType::PowerShell => {
            println!("  Add-Content $PROFILE 'source ~/.dx/shell/dx-shell-init.sh'");
        }
    }
    
    Ok(())
}

pub fn bash_hook() -> &'static str {
    r#"dx_hook() {
    local cmd="$1"
    eval "$cmd"
}
"#
}
