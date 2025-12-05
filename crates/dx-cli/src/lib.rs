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

    let cli = Cli::parse();
    crate::banner::print();

    dispatch_command(cli).await?;

    Ok(())
}

fn setup_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();
}
