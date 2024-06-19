use std::sync::Arc;

use axum::handler::Handler;
use bollard::Docker;
use eyre::Result;
use http::HttpState;
use proto::well_known;
use runner::Runner;
use tracing::info;

use crate::{args::WorkerArgs, monitor::pusher, proxy::ProxyState};

mod args;
mod http;
mod monitor;
mod proxy;
mod runner;
mod sender;

#[tokio::main]
async fn main() -> Result<()> {
    setup::tracing();

    let args = Arc::new(WorkerArgs::parse());
    info!(?args, "started worker");

    let sender = Arc::new(sender::Sender::new(args.controller_addr));

    let pusher_handle = tokio::spawn({
        let args = Arc::clone(&args);
        let sender = Arc::clone(&sender);
        async move {
            pusher::start_pusher(args, sender).await;
        }
    });

    let (proxy_state, proxy_handle) = ProxyState::new();

    let proxy_server = tokio::spawn(async {
        let app = proxy::proxy.with_state(proxy_state);
        let listener = tokio::net::TcpListener::bind(("127.0.0.1", well_known::WORKER_PROXY_PORT))
            .await
            .unwrap();
        axum::serve(listener, app).await.unwrap();
    });

    let docker = Arc::new(Docker::connect_with_defaults().unwrap());
    let (runner, runner_handle) = Runner::new(docker, sender, proxy_handle);
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

    proxy_server.await.unwrap();
    pusher_handle.await.unwrap();
    runner_actor_handle.await.unwrap();
    http_handle.await.unwrap();

    Ok(())
}
