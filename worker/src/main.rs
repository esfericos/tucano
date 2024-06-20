use std::{
    net::{IpAddr, Ipv4Addr},
    sync::Arc,
};

use axum::handler::Handler;
use bollard::Docker;
use clap::Parser;
use eyre::Result;
use http::HttpState;
use proto::clients::CtlClient;
use runner::Runner;
use tokio::task::JoinSet;
use tracing::info;
use utils::server::mk_listener;

use crate::{args::WorkerArgs, monitor::pusher, proxy::ProxyState};

mod args;
mod http;
mod monitor;
mod proxy;
mod runner;

const ANY_IP: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));

#[tokio::main]
async fn main() -> Result<()> {
    utils::setup::tracing();

    let args = Arc::new(WorkerArgs::parse());
    info!(?args, "started worker");

    let ctl_client = CtlClient::new(args.controller_addr);

    let (proxy_listener, proxy_port) = mk_listener(ANY_IP, args.proxy_port).await?;
    let (http_listener, http_port) = mk_listener(ANY_IP, args.http_port).await?;

    let mut bag = JoinSet::new();

    let (proxy_state, proxy_handle) = ProxyState::new();
    bag.spawn(async move {
        let app = proxy::proxy.with_state(proxy_state);
        info!("worker proxy listening at {ANY_IP}:{proxy_port}");
        axum::serve(proxy_listener, app).await.unwrap();
    });

    let docker = Arc::new(Docker::connect_with_defaults().unwrap());
    let (runner, runner_handle) = Runner::new(docker, ctl_client.clone(), proxy_handle);
    bag.spawn(async move {
        runner.run().await;
    });

    bag.spawn(async move {
        let state = HttpState {
            runner: runner_handle.clone(),
        };
        let app = http::mk_app(state);
        info!("worker http listening at {ANY_IP}:{http_port}");
        axum::serve(http_listener, app).await.unwrap();
    });

    bag.spawn({
        let args = Arc::clone(&args);
        let ctl_client = ctl_client.clone();
        async move {
            pusher::start_pusher(args, ctl_client).await;
        }
    });

    while let Some(res) = bag.join_next().await {
        res?;
    }

    Ok(())
}

#[derive(Debug)]
pub struct PortMap {
    pub proxy: u16,
    pub host: u16,
}
