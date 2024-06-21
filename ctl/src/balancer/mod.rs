use std::{
    collections::HashMap,
    net::{IpAddr, SocketAddr},
    str::FromStr as _,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};

use axum::{
    body::Body,
    extract::{ConnectInfo, Request, State},
    http::{
        uri::{Authority, Scheme},
        HeaderValue, StatusCode, Uri,
    },
    response::IntoResponse,
};
use hyper_util::{
    client::legacy::{connect::HttpConnector, Client},
    rt::TokioExecutor,
};
use proto::{
    common::{instance::InstanceId, service::ServiceId},
    well_known::{PROXY_FORWARDED_HEADER_NAME, PROXY_INSTANCE_HEADER_NAME, WORKER_PROXY_PORT},
};
use tracing::{instrument, trace, warn};
use utils::http::{self, OptionExt as _, ResultExt as _};

#[derive(Default)]
pub struct InstanceBag {
    pub instances: Vec<(InstanceId, IpAddr)>,
    pub count: AtomicUsize,
}

#[derive(Clone)]
pub struct BalancerState {
    pub addrs: Arc<Mutex<HashMap<ServiceId, InstanceBag>>>,
    pub client: Client<HttpConnector, Body>,
}

impl BalancerState {
    #[must_use]
    pub fn new() -> (Self, BalancerHandle) {
        let map = Arc::new(Mutex::new(HashMap::default()));
        (
            BalancerState {
                addrs: map.clone(),
                client: {
                    let mut connector = HttpConnector::new();
                    connector.set_keepalive(Some(Duration::from_secs(60)));
                    connector.set_nodelay(true);
                    Client::builder(TokioExecutor::new()).build::<_, Body>(connector)
                },
            },
            BalancerHandle { addrs: map },
        )
    }

    pub fn next(&self, service: &ServiceId) -> Option<(InstanceId, IpAddr)> {
        let map = self.addrs.lock().unwrap();
        let bag = map.get(service)?;
        let count = bag.count.fetch_add(1, Ordering::Relaxed);
        Some(bag.instances[count % bag.instances.len()])
    }
}

pub struct BalancerHandle {
    pub addrs: Arc<Mutex<HashMap<ServiceId, InstanceBag>>>,
}

impl BalancerHandle {
    #[allow(dead_code)]
    pub fn add_instance(&self, id: ServiceId, instance_id: InstanceId, addr: IpAddr) {
        let mut map = self.addrs.lock().unwrap();
        let bag = map.entry(id).or_default();
        bag.instances.push((instance_id, addr));
    }

    #[allow(dead_code)]
    pub fn drop_instance(&self, id: &ServiceId, instance_id: InstanceId) {
        let mut map = self.addrs.lock().unwrap();
        let Some(bag) = map.get_mut(id) else {
            warn!("attempted to drop instance from unknown service id");
            return;
        };
        // Remove the instance (keep all except this one)
        bag.instances.retain(|(inst, _)| inst != &instance_id);
    }
}

#[instrument(skip_all)]
pub async fn proxy(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(balancer): State<BalancerState>,
    mut req: Request,
) -> http::Result<impl IntoResponse> {
    let service_id = extract_service_id(&mut req)?;

    let (instance_id, server_addr) = balancer
        .next(&service_id)
        .or_http_error(StatusCode::NOT_FOUND, "service not found")?;
    trace!(%service_id, %instance_id, %server_addr, "received and balanced user request");

    *req.uri_mut() = {
        let uri = req.uri();
        let mut parts = uri.clone().into_parts();
        parts.authority = Authority::from_str(&format!("{server_addr}:{WORKER_PROXY_PORT}")).ok();
        parts.scheme = Some(Scheme::HTTP);
        Uri::from_parts(parts).unwrap()
    };

    req.headers_mut().insert(
        PROXY_INSTANCE_HEADER_NAME,
        HeaderValue::from_str(&instance_id.to_string()).unwrap(),
    );
    req.headers_mut().insert(
        PROXY_FORWARDED_HEADER_NAME,
        HeaderValue::from_str(&addr.ip().to_string()).unwrap(),
    );

    balancer
        .client
        .request(req)
        .await
        .http_error(StatusCode::BAD_GATEWAY, "bad gateway")
}

fn extract_service_id(req: &mut Request) -> http::Result<ServiceId> {
    let inner = req
        .headers()
        .get("Host")
        .unwrap()
        .to_str()
        .ok()
        .and_then(|s| s.parse().ok())
        .or_http_error(StatusCode::BAD_REQUEST, "invalid service name")?;
    Ok(ServiceId(inner))
}
