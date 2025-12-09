use anyhow::Result;
use tracing::{info, warn};

use crate::cli::args::{AgentCommand, ChatCommand, Cli, Commands, GenerateCommand, ShellCommand, ToolCommand, ConfigCommand, ConfigAction};
use crate::cli::completions;
use dx_tui::ChatApp;
use dx_core::config;

pub async fn dispatch_command(cli: Cli) -> Result<()> {
    // If no subcommand provided, launch interactive chat
    let command = match cli.command {
        Some(cmd) => cmd,
        None => {
            // Launch interactive AI chat
            let config_data = config::load().ok();
            let mut chat_app = ChatApp::new(config_data);
            return chat_app.run().await;
        }
    };

    match command {
        Commands::Ui(cmd) => handle_tool("ui", cmd).await?,
        Commands::Style(cmd) => handle_tool("style", cmd).await?,
        Commands::Icon(cmd) => handle_tool("icon", cmd).await?,
        Commands::Font(cmd) => handle_tool("font", cmd).await?,
        Commands::Auth(cmd) => handle_tool("auth", cmd).await?,
        Commands::Media(cmd) => handle_tool("media", cmd).await?,
        Commands::I18n(cmd) => handle_tool("i18n", cmd).await?,
        Commands::Forge(cmd) => handle_tool("forge", cmd).await?,
        Commands::Generate(cmd) => handle_generate(cmd).await?,
        Commands::Chat(cmd) => handle_chat(cmd).await?,
        Commands::Agent(cmd) => handle_agent(cmd).await?,
        Commands::Shell(cmd) => handle_shell(cmd).await?,
        Commands::Completions { shell } => completions::generate_completions(shell)?,
        Commands::Config(cmd) => handle_config(cmd).await?,
    }

    Ok(())
}

async fn handle_tool(tool: &str, cmd: ToolCommand) -> Result<()> {
    if cmd.list {
        println!("🔍 Listing available {tool} entries (preview)");
        println!(" - use `dx {tool} --name <item>` to target a specific entry");
    }

    if let Some(name) = cmd.name {
        println!("🚀 Invoking {tool} with target '{name}'");
    } else if !cmd.list {
        warn!("No target provided for {tool}; try --list or --name <item>");
    }

    Ok(())
}

async fn handle_generate(cmd: GenerateCommand) -> Result<()> {
    info!("generator" = %cmd.kind, template = ?cmd.template, dry_run = cmd.dry_run, "Dispatching generator command");
    if cmd.dry_run {
        println!("[dry-run] Would run generator '{}'", cmd.kind);
    } else {
        println!("✨ Running generator '{}'", cmd.kind);
    }
    Ok(())
}

async fn handle_chat(cmd: ChatCommand) -> Result<()> {
    if let Some(message) = cmd.message {
        println!("[chat] user: {message}");
        println!("[chat] assistant: Friday is thinking... (placeholder response)");
    } else {
        println!("Opening interactive chat shell (placeholder)");
    }
    Ok(())
}

async fn handle_agent(cmd: AgentCommand) -> Result<()> {
    println!("🛰️ Executing agent: {:?} with task {:?}", cmd.id, cmd.task);
    Ok(())
}

async fn handle_shell(cmd: ShellCommand) -> Result<()> {
    if cmd.enable {
        dx_shell::enable_shell_enhancements()?;
        
        // Auto-install if not already done
        if let Some(shell_type) = dx_shell::ShellType::detect() {
            println!("\nDetected shell: {}", shell_type.name());
            println!("\nTo complete setup, run:");
            println!("  bash ~/.dx/shell/install.sh");
        }
    } else if cmd.disable {
        dx_shell::disable_shell_enhancements()?;
    } else {
        // Show status and install if needed - shell detection always succeeds now
        let shell_type = dx_shell::ShellType::detect().unwrap_or(dx_shell::ShellType::Bash);
        
        println!("✓ Detected shell: {}", shell_type.name());
        println!();
        println!("Shell enhancements available:");
        println!("  • Enhanced ls with file type icons");
        println!("  • Command help hints (Ctrl+H)");
        println!("  • Smart autocomplete and history");
        println!("  • Command suggestions for typos");
        println!("  • Persistent command memory");
        println!();
        println!("Compatible with:");
        println!("  • Bash (Linux, macOS, Git Bash, WSL, Termux)");
        println!("  • Zsh (macOS, Linux)");
        println!("  • Fish");
        println!("  • PowerShell (Windows)");
        println!();
        println!("Install with: dx shell --enable");
        
        // Check if already installed
        let home = dirs::home_dir().unwrap_or_default();
        let installed = home.join(".dx/shell/dx-shell-init.sh").exists();
        
        if installed {
            println!();
            println!("✓ Enhancement scripts installed");
            println!("  Restart your shell to activate");
        }
    }
    Ok(())
}

async fn handle_config(cmd: ConfigCommand) -> Result<()> {
    let config_dir = dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?
        .join(".dx");
    
    std::fs::create_dir_all(&config_dir)?;
    let config_file = config_dir.join("config.toml");

    match cmd.action {
        ConfigAction::SetApiKey { key } => {
            // Load existing config or create new
            let mut config = config::load().unwrap_or_default();
            config.ai.gemini_api_key = Some(key.clone());
            
            // Save config
            let toml_string = toml::to_string_pretty(&config)?;
            std::fs::write(&config_file, toml_string)?;
            
            println!("✓ Gemini API key saved to: {}", config_file.display());
            println!("  Key: {}...{}", &key[..8.min(key.len())], &key[key.len().saturating_sub(4)..]);
        }
        ConfigAction::GetApiKey => {
            if let Ok(config) = config::load() {
                if let Some(key) = config.ai.gemini_api_key {
                    println!("Gemini API Key: {}...{}", &key[..8.min(key.len())], &key[key.len().saturating_sub(4)..]);
                } else {
                    println!("No API key set. Using default key.");
                    println!("Set your key with: dx config set-api-key <YOUR_KEY>");
                }
            } else {
                println!("No configuration found. Using default API key.");
                println!("Set your key with: dx config set-api-key <YOUR_KEY>");
            }
        }
    }
    
    Ok(())
}

