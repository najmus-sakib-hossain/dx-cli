use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "xtask", about = "Build automation tasks for dx-cli")]
struct Xtask {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    /// Example task placeholder
    Hello,
}

fn main() -> anyhow::Result<()> {
    let cli = Xtask::parse();
    match cli.command.unwrap_or(Commands::Hello) {
        Commands::Hello => println!("xtask says hello"),
    }
    Ok(())
}
