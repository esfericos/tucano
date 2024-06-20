use std::{convert::Infallible, future::IntoFuture, io};

use axum::{
    extract::Request,
    response::Response,
    serve::{IncomingStream, Serve},
};
use tokio::net::{TcpListener, ToSocketAddrs};
use tower::Service;
use tracing::info;

pub async fn listen<A, M, S>(name: &'static str, mk_svc: M, addr: A)
where
    A: ToSocketAddrs,
    M: for<'a> Service<IncomingStream<'a>, Error = Infallible, Response = S>,
    S: Service<Request, Response = Response, Error = Infallible> + Clone + Send + 'static,
    S::Future: Send,
    Serve<M, S>: IntoFuture<Output = io::Result<()>>,
{
    let listener = TcpListener::bind(addr).await.unwrap();
    let addr = listener.local_addr().unwrap();
    info!("{name} listening at {addr}");
    axum::serve(listener, mk_svc).await.unwrap();
}
