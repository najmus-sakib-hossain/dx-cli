use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "dx",
    version,
    about = "Dx - Enhanced Development Experience CLI",
    author = "Dx Team"
)]
pub struct Cli {
    /// Increase output verbosity
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Silence most output
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// Path to configuration file
    #[arg(short, long, global = true)]
    pub config: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Ui(ToolCommand),
    Style(ToolCommand),
    Icon(ToolCommand),
    Font(ToolCommand),
    Auth(ToolCommand),
    Media(ToolCommand),
    I18n(ToolCommand),
    Forge(ToolCommand),
    Generate(GenerateCommand),
    Chat(ChatCommand),
    Agent(AgentCommand),
    Shell(ShellCommand),
    Completions { shell: clap_complete::Shell },
}

#[derive(Args, Debug, Default)]
pub struct ToolCommand {
    /// List available assets for the tool
    #[arg(long)]
    pub list: bool,

    /// Target item name
    #[arg(long)]
    pub name: Option<String>,
}

#[derive(Args, Debug, Default)]
pub struct GenerateCommand {
    /// Generator to use (component, project, readme, docs, test)
    #[arg(long, default_value = "project")]
    pub kind: String,

    /// Name of the template or generator
    #[arg(long)]
    pub template: Option<String>,

    /// Dry-run without writing files
    #[arg(long)]
    pub dry_run: bool,
}

#[derive(Args, Debug, Default)]
pub struct ChatCommand {
    /// Inline user message
    #[arg(long)]
    pub message: Option<String>,
}

#[derive(Args, Debug, Default)]
pub struct AgentCommand {
    /// Agent id to invoke
    #[arg(long)]
    pub id: Option<String>,

    /// Task description
    #[arg(long)]
    pub task: Option<String>,
}

#[derive(Args, Debug, Default)]
pub struct ShellCommand {
    /// Enable shell enhancements
    #[arg(long)]
    pub enable: bool,

    /// Disable shell enhancements
    #[arg(long)]
    pub disable: bool,
}
