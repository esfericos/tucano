use std::sync::{Arc, Mutex};
use tracing::info;

use crate::discovery::Discovery;

mod discovery;
mod http;

#[tokio::main]
async fn main() {
    setup::tracing();

    info!("started controller");

    let (discovery, discovery_handle) = Discovery::new();

    let discovery_actor_handle = tokio::spawn(async move {
        discovery.run().await;
    });

    let http_handle = tokio::spawn({
        let discovery_handle = discovery_handle.clone();
  
        async move {
            http::run_server(discovery_handle).await;
        }
    });

    discovery_actor_handle.await.unwrap();
    http_handle.await.unwrap();
}
