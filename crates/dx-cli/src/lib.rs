#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::module_name_repetitions)]

pub mod banner;
pub mod cli;

use clap::Parser;
use tracing_subscriber::EnvFilter;

use crate::cli::args::Cli;
use crate::cli::commands::dispatch_command;

pub async fn run() -> anyhow::Result<()> {
    setup_tracing();

    let cli = Cli::try_parse();
    
    match cli {
        Ok(cli) => {
            crate::banner::print();
            dispatch_command(cli).await?;
        }
        Err(e) => {
            // Print help/error and exit with appropriate code
            e.exit();
        }
    }

    Ok(())
}

fn setup_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();
}
