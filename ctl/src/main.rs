use tracing::info;

mod http;

#[tokio::main]
async fn main() {
    setup::tracing();

    info!("started controller");

    let http_handle = tokio::spawn(async { http::run_server().await });
    http_handle.await.unwrap();
}
