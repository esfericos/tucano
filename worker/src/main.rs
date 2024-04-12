use eyre::Result;
use tokio::task::spawn;
use tracing::info;

use crate::{args::WorkerArgs, monitor::pusher};

mod args;
mod monitor;

#[tokio::main]
async fn main() -> Result<()> {
    setup::tracing();

    let args = WorkerArgs::parse();
    info!(?args, "started worker");

    spawn(async { pusher::push(args).await });

    Ok(())
}
