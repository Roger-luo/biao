mod cli;
mod client;
mod models;
mod error;
mod git;
mod config;
mod templates;

use anyhow::Result;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    let args = cli::Args::parse();
    cli::execute(args).await.map_err(|e| anyhow::anyhow!(e))
}
