use std::sync::Arc;

use eyre::Result;
use tracing::info;

use crate::{args::WorkerArgs, monitor::pusher};

mod args;
mod monitor;

#[tokio::main]
async fn main() -> Result<()> {
    setup::tracing();

    let args = Arc::new(WorkerArgs::parse());
    info!(?args, "started worker");

    let pusher_handle = tokio::spawn({
        let args = Arc::clone(&args);
        async {
            pusher::start_pusher(args).await;
        }
    });
    pusher_handle.await.unwrap();

    Ok(())
}
