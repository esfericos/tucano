use std::{
    net::{IpAddr, Ipv4Addr},
    sync::Arc,
};

use clap::Parser;
use proto::well_known::{CTL_BALANCER_PORT, CTL_HTTP_PORT};
use tokio::task::JoinSet;
use tracing::info;
use utils::server::mk_listener;

use crate::{args::CtlArgs, discovery::Discovery, http::HttpState};

mod args;
mod discovery;
mod http;

const ANY_IP: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));

#[tokio::main]
async fn main() -> eyre::Result<()> {
    utils::setup::tracing();

    let args = Arc::new(CtlArgs::parse());
    info!(?args, "started ctl");

    let _balancer_listener = mk_listener(ANY_IP, CTL_BALANCER_PORT).await?;
    let http_listener = mk_listener(ANY_IP, CTL_HTTP_PORT).await?;

    let mut bag = JoinSet::new();

    let (discovery, discovery_handle) = Discovery::new();
    bag.spawn(async move {
        discovery.run().await;
    });

    bag.spawn(async move {
        let state = HttpState {
            discovery: discovery_handle.clone(),
        };
        let app = http::mk_app(state);
        info!("ctl http listening at {ANY_IP}:{CTL_HTTP_PORT}");
        axum::serve(http_listener, app).await.unwrap();
    });

    while let Some(res) = bag.join_next().await {
        res?;
    }

    Ok(())
}
