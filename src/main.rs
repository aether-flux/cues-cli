use clap::Parser;
use cli::{Cli, Commands, NewProject};
use commands::handle;
use keyring::Entry;
use utils::auth::AuthStore;

mod cli;
mod commands;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse arguments from command
    let args = Cli::try_parse().unwrap_or_else(|e| e.exit());

    // Initialize store for storing access and refresh tokens
    let auth_store = AuthStore {
        access: Entry::new("cues", "access_token")?,
        refresh: Entry::new("cues", "refresh_token")?,
    };

    // Handle CLI commands through the function
    handle::handle_cli(args, auth_store).await?;

    Ok(())
}
