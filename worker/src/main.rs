use std::sync::Arc;

use axum::handler::Handler;
use bollard::Docker;
use eyre::Result;
use http::HttpState;
use proto::{clients::CtlClient, well_known::WORKER_PROXY_PORT};
use runner::Runner;
use tracing::info;
use utils::server;

use crate::{args::WorkerArgs, monitor::pusher, proxy::ProxyState};

mod args;
mod http;
mod monitor;
mod proxy;
mod runner;

#[tokio::main]
async fn main() -> Result<()> {
    utils::setup::tracing();

    let args = Arc::new(WorkerArgs::parse());
    info!(?args, "started worker");

    let ctl_client = CtlClient::new(args.controller_addr);

    let pusher_handle = tokio::spawn({
        let args = Arc::clone(&args);
        let ctl_client = ctl_client.clone();
        async move {
            pusher::start_pusher(args, ctl_client).await;
        }
    });

    let (proxy_state, proxy_handle) = ProxyState::new();

    let proxy_server = tokio::spawn(async {
        let app = proxy::proxy.with_state(proxy_state);
        server::listen("worker proxy", app, ("0.0.0.0", WORKER_PROXY_PORT)).await;
    });

    let docker = Arc::new(Docker::connect_with_defaults().unwrap());
    let (runner, runner_handle) = Runner::new(docker, ctl_client, proxy_handle);
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
