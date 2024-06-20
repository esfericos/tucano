use std::{
    collections::HashMap,
    str::FromStr as _,
    sync::{Arc, RwLock},
    time::Duration,
};

use axum::{
    self,
    body::Body,
    extract::{Request, State},
    http::{
        uri::{Authority, Scheme},
        Uri,
    },
    response::IntoResponse,
};
use hyper_util::{
    client::legacy::{connect::HttpConnector, Client},
    rt::TokioExecutor,
};
use proto::{common::instance::InstanceId, well_known::PROXY_INSTANCE_HEADER_NAME};
use reqwest::StatusCode;
use utils::http::{self, OptionExt as _, ResultExt as _};

pub async fn proxy(
    State(proxy): State<ProxyState>,
    mut req: Request,
) -> http::Result<impl IntoResponse> {
    let id = extract_instance_id(&mut req)?;

    let port = {
        let read_map = proxy.ports.read().unwrap();
        *read_map.get(&id).unwrap()
    };

    *req.uri_mut() = {
        let uri = req.uri();
        let mut parts = uri.clone().into_parts();
        parts.authority = Authority::from_str(&format!("127.0.0.1:{port}")).ok();
        parts.scheme = Some(Scheme::HTTP);
        Uri::from_parts(parts).unwrap()
    };

    proxy
        .client
        .request(req)
        .await
        .http_error(StatusCode::BAD_GATEWAY, "bad gateway")
}

#[derive(Clone)]
pub struct ProxyState {
    pub ports: Arc<RwLock<HashMap<InstanceId, u16>>>,
    pub client: Client<HttpConnector, Body>,
}

impl ProxyState {
    #[must_use]
    pub fn new() -> (Self, ProxyHandle) {
        let map = Arc::new(RwLock::new(HashMap::default()));
        (
            ProxyState {
                ports: map.clone(),
                client: {
                    let mut connector = HttpConnector::new();
                    connector.set_keepalive(Some(Duration::from_secs(60)));
                    connector.set_nodelay(true);
                    Client::builder(TokioExecutor::new()).build::<_, Body>(connector)
                },
            },
            ProxyHandle { ports: map },
        )
    }
}

pub struct ProxyHandle {
    pub ports: Arc<RwLock<HashMap<InstanceId, u16>>>,
}

impl ProxyHandle {
    pub fn add_instance(&mut self, id: InstanceId, port: u16) {
        let mut map = self.ports.write().unwrap();
        map.insert(id, port);
    }

    pub fn remove_instance(&mut self, id: InstanceId) {
        let mut map = self.ports.write().unwrap();
        map.remove(&id);
    }
}

fn extract_instance_id(req: &mut Request) -> http::Result<InstanceId> {
    // i'm so sorry
    let inner = req
        .headers_mut()
        .remove(PROXY_INSTANCE_HEADER_NAME)
        .or_http_error(StatusCode::BAD_REQUEST, "missing instance id from gw")?
        .to_str()
        .ok()
        .and_then(|s| s.parse().ok())
        .or_http_error(StatusCode::BAD_REQUEST, "invalid instance id")?;
    Ok(InstanceId(inner))
}
