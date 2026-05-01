mod background;
mod cue;
mod db;
mod logger;
mod notifier;
mod resources;
mod service;
mod settings;
mod types;

use clap::{Parser, Subcommand};
use db::{ManifestDb, OsvDb};
use tracing::info;

use crate::{cue::CueCtx, db::CoreDb, notifier::Notifier, resources::ResourceRegistry};

#[derive(Parser)]
#[command(version, about = "OPPSY backend")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Print the `OpenAPI` JSON schema to stdout
    Docs,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Docs) => {
            print!("{}", service::spec());
            Ok(())
        },
        None => run_server().await,
    }
}

async fn run_server() -> anyhow::Result<()> {
    ResourceRegistry::register::<settings::Settings>().await?;
    logger::init()?;

    info!("OSV service - starting");

    let mut tasks = tokio::task::JoinSet::new();

    tasks.spawn(async { ResourceRegistry::register::<CueCtx>().await });
    tasks.spawn(async { ResourceRegistry::register::<ManifestDb>().await });
    tasks.spawn(async { ResourceRegistry::register::<OsvDb>().await });
    tasks.spawn(async { ResourceRegistry::register::<CoreDb>().await });
    tasks.spawn(async { ResourceRegistry::register::<Notifier>().await });
    tasks.spawn(async { service::run().await });
    tasks.spawn(background::osv_sync_task());

    while let Some(res) = tasks.join_next().await {
        res??;
    }

    info!("OSV service - shut down");

    Ok(())
}
