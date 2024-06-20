use std::net::IpAddr;

use eyre::Context;
use tokio::net::TcpListener;

/// Creates a new TCP listener.
///
/// Tries to use the provided port, if any. If the provided port is already in
/// use, this method will return an error.
///
/// If no port is provided, a random one will be chosen by the OS.
pub async fn mk_listener(addr: impl Into<IpAddr>, port: u16) -> eyre::Result<TcpListener> {
    let addr = addr.into();

    let listener = TcpListener::bind((addr, port))
        .await
        .wrap_err("failed to start tcp listener")?;

    Ok(listener)
}
