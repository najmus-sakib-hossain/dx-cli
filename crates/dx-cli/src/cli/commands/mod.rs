use anyhow::Result;
use tracing::{info, warn};

use crate::cli::args::{AgentCommand, ChatCommand, Cli, Commands, GenerateCommand, ShellCommand, ToolCommand};
use crate::cli::completions;
use dx_tui::TuiApp;

pub async fn dispatch_command(cli: Cli) -> Result<()> {
    match cli.command {
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
        Commands::Tui => handle_tui().await?,
        Commands::Completions { shell } => completions::generate_completions(shell)?,
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
        println!("🔧 Enabling shell enhancements");
    }
    if cmd.disable {
        println!("🛑 Disabling shell enhancements");
    }
    if !cmd.enable && !cmd.disable {
        println!("Shell enhancements status: active (preview)");
    }
    Ok(())
}

async fn handle_tui() -> Result<()> {
    println!("Launching dx-tui (press q to quit)...");
    let mut app = TuiApp::new()?;
    app.run()?;
    Ok(())
}
