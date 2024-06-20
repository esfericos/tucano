use std::{
    net::{IpAddr, Ipv4Addr},
    sync::Arc,
};

use clap::Parser;
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

    let (_balancer_listener, _balancer_port) = mk_listener(ANY_IP, args.balancer_port).await?;
    let (http_listener, http_port) = mk_listener(ANY_IP, args.http_port).await?;

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
        info!("ctl http listening at {ANY_IP}:{http_port}");
        axum::serve(http_listener, app).await.unwrap();
    });

    while let Some(res) = bag.join_next().await {
        res?;
    }

    Ok(())
}
