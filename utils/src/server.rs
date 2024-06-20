use std::net::IpAddr;

use eyre::Context;
use tokio::net::TcpListener;

/// Creates a new TCP listener.
///
/// Tries to use the provided port, if any. If the provided port is already in
/// use, this method will return an error.
///
/// If no port is provided, a random one will be chosen by the OS.
pub async fn mk_listener(
    addr: impl Into<IpAddr>,
    port: Option<u16>,
) -> eyre::Result<(TcpListener, u16)> {
    let addr = addr.into();
    let port = port.unwrap_or(0);

    let listener = TcpListener::bind((addr, port))
        .await
        .wrap_err("failed to start tcp listener")?;

    let local_addr = listener.local_addr().expect("local addr must exist");
    let port = local_addr.port();

    Ok((listener, port))
}
