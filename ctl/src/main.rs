use tracing::info;

use crate::{discovery::Discovery, http::HttpState};

mod discovery;
mod http;

#[tokio::main]
async fn main() {
    utils::setup::tracing();
    info!("started controller");

    let (discovery, discovery_handle) = Discovery::new();

    let discovery_actor_handle = tokio::spawn(async move {
        discovery.run().await;
    });

    let http_handle = tokio::spawn({
        let state = HttpState {
            discovery: discovery_handle.clone(),
        };
        async move {
            http::run_server(state).await;
        }
    });

    discovery_actor_handle.await.unwrap();
    http_handle.await.unwrap();
}
