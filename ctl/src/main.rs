use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};

use axum::handler::Handler;
use clap::Parser;
use proto::well_known::{CTL_BALANCER_PORT, CTL_HTTP_PORT};
use tokio::task::JoinSet;
use tracing::info;
use utils::server::mk_listener;

use crate::{
    args::CtlArgs, balancer::BalancerState, discovery::Discovery, http::HttpState,
    worker_mgr::WorkerMgr,
};

mod args;
mod balancer;
mod discovery;
mod http;
mod worker_mgr;

const ANY_IP: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));

#[tokio::main]
async fn main() -> eyre::Result<()> {
    utils::setup::tracing();

    let args = Arc::new(CtlArgs::parse());
    info!(?args, "started ctl");

    let balancer_listener = mk_listener(ANY_IP, CTL_BALANCER_PORT).await?;
    let http_listener = mk_listener(ANY_IP, CTL_HTTP_PORT).await?;

    let mut bag = JoinSet::new();

    let (discovery, discovery_handle) = Discovery::new();
    bag.spawn(async move {
        discovery.run().await;
    });

    let (worker_mgr, worker_mgr_handle) = WorkerMgr::new(args.worker_liveness_timeout);
    bag.spawn(async move {
        worker_mgr.run().await;
    });

    let balancer_state = BalancerState::new();
    bag.spawn(async move {
        let app = balancer::proxy.with_state(balancer_state);
        info!("balancer http listening at {ANY_IP}:{CTL_BALANCER_PORT}");
        axum::serve(balancer_listener, app).await.unwrap();
    });

    bag.spawn(async move {
        let state = HttpState {
            discovery: discovery_handle,
            worker_mgr: worker_mgr_handle,
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
