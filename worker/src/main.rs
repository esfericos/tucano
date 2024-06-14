use std::sync::Arc;

use eyre::Result;
use http::HttpState;
use runner::Runner;
use tracing::info;

use crate::{args::WorkerArgs, monitor::pusher};

mod args;
mod http;
mod monitor;
mod runner;

#[tokio::main]
async fn main() -> Result<()> {
    setup::tracing();

    let args = WorkerArgs::parse();
    info!(?args, "started worker");

    let pusher_handle = tokio::spawn({
        async move {
            pusher::start_pusher(Arc::new(args)).await;
        }
    });

    let (runner, runner_handle) = Runner::new();
    let runner_actor_handle = tokio::spawn(async move {
        runner.run().await;
    });

    let http_handle = tokio::spawn({
        let state = HttpState {
            runner: runner_handle.clone(),
        };
        async {
            http::run_server(state).await;
        }
    });

    pusher_handle.await.unwrap();
    runner_actor_handle.await.unwrap();
    http_handle.await.unwrap();

    Ok(())
}
