use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};

use axum::handler::Handler;
use clap::Parser;
use proto::{
    clients::WorkerClient,
    well_known::{CTL_BALANCER_PORT, CTL_HTTP_PORT},
};
use tokio::task::JoinSet;
use tracing::info;
use utils::server::mk_listener;

use crate::{
    args::CtlArgs, balancer::BalancerState, deployer::Deployer, http::HttpState,
    worker_mgr::WorkerMgr,
};

mod args;
mod balancer;
mod deployer;
mod http;
mod worker_mgr;

const ANY_IP: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));

#[tokio::main]
async fn main() -> eyre::Result<()> {
    utils::setup::tracing();

    let args = Arc::new(CtlArgs::parse());
    info!(?args, "started ctl");

    let worker_client = WorkerClient::new();

    let worker_client = WorkerClient::new();

    let balancer_listener = mk_listener(ANY_IP, CTL_BALANCER_PORT).await?;
    let http_listener = mk_listener(ANY_IP, CTL_HTTP_PORT).await?;

    let mut bag = JoinSet::new();

    let (worker_mgr, worker_mgr_handle) = WorkerMgr::new(args.worker_liveness_timeout);
    bag.spawn(async move {
        worker_mgr.run().await;
    });

    let (balancer, _balancer_handle) = BalancerState::new();
    bag.spawn(async move {
        let app = balancer::proxy
            .with_state(balancer)
            .into_make_service_with_connect_info::<SocketAddr>();
        info!("balancer http listening at {ANY_IP}:{CTL_BALANCER_PORT}");
        axum::serve(balancer_listener, app).await.unwrap();
    });

    let (deployer, deployer_handle) = Deployer::new(worker_mgr_handle.clone(), worker_client);
    bag.spawn(async move {
        deployer.run().await;
    });

    bag.spawn(async move {
        let state = HttpState {
            worker_mgr: worker_mgr_handle,
            deployer: deployer_handle,
        };
        let app = http::mk_app(state).into_make_service_with_connect_info::<SocketAddr>();
        info!("ctl http listening at {ANY_IP}:{CTL_HTTP_PORT}");
        axum::serve(http_listener, app).await.unwrap();
    });

    while let Some(res) = bag.join_next().await {
        res?;
    }

    Ok(())
}
