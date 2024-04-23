use tracing::info;

use crate::discovery::Discovery;

mod discovery;
mod http;

#[tokio::main]
async fn main() {
    setup::tracing();

    info!("started controller");

    let (discovery, _discovery_handle) = Discovery::new();
    let discovery_actor_handle = tokio::spawn(async move {
        discovery.run().await;
    });

    let http_handle = tokio::spawn(async {
        http::run_server().await;
    });

    discovery_actor_handle.await.unwrap();
    http_handle.await.unwrap();
}
