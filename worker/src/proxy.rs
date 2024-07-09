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
use tracing::{instrument, trace};
use utils::http::{self, OptionExt as _, ResultExt as _};

use crate::args::WorkerArgs;

#[instrument(skip_all)]
pub async fn proxy(
    State(proxy): State<ProxyState>,
    mut req: Request,
) -> http::Result<impl IntoResponse> {
    let instance_id = extract_instance_id(&mut req)?;
    trace!(%instance_id, "received user request");

    let maybe_port = {
        let read_map = proxy.ports.read().unwrap();
        read_map.get(&instance_id).copied()
    };
    let port = maybe_port
        .ok_or_else(|| eyre::eyre!("requested instance doesn't exist at requested worker"))
        .http_error(StatusCode::BAD_GATEWAY, "bad gateway")?;

    let host_name = match proxy.mode {
        ProxyMode::Normal => format!("127.0.0.1:{port}"),
        ProxyMode::DockerNetwork => format!("instance-{instance_id}:{port}"),
    };

    *req.uri_mut() = {
        let uri = req.uri();
        let mut parts = uri.clone().into_parts();
        parts.authority = Authority::from_str(&host_name).ok();
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
    pub mode: ProxyMode,
}

#[derive(Copy, Clone)]
pub enum ProxyMode {
    Normal,
    DockerNetwork,
}

impl ProxyState {
    #[must_use]
    pub fn new(worker_args: &WorkerArgs) -> (Self, ProxyHandle) {
        let mode = match &worker_args.use_docker_network {
            Some(_) => ProxyMode::DockerNetwork,
            None => ProxyMode::Normal,
        };
        let ports = Arc::new(RwLock::new(HashMap::default()));
        let state = ProxyState {
            ports: ports.clone(),
            client: {
                let mut connector = HttpConnector::new();
                connector.set_keepalive(Some(Duration::from_secs(60)));
                connector.set_nodelay(true);
                Client::builder(TokioExecutor::new()).build::<_, Body>(connector)
            },
            mode,
        };
        let handle = ProxyHandle { ports };
        (state, handle)
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
    let inner = req
        .headers_mut()
        .get(PROXY_INSTANCE_HEADER_NAME)
        .or_http_error(StatusCode::BAD_REQUEST, "missing instance id from gw")?
        .to_str()
        .ok()
        .and_then(|s| s.parse().ok())
        .or_http_error(StatusCode::BAD_REQUEST, "invalid instance id")?;
    Ok(InstanceId(inner))
}
