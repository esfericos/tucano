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
    well_known::{PROXY_FORWARDED_HEADER_NAME, PROXY_INSTANCE_HEADER_NAME},
};
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

    pub fn next(&self, service: &ServiceId) -> (InstanceId, IpAddr) {
        let map = self.addrs.lock().unwrap();
        let bag = map.get(service).unwrap();
        let count = bag.count.fetch_add(1, Ordering::Relaxed);
        bag.instances[count % bag.instances.len()]
    }
}

pub struct BalancerHandle {
    pub addrs: Arc<Mutex<HashMap<ServiceId, InstanceBag>>>,
}

impl BalancerHandle {
    #[allow(dead_code)]
    pub fn add_instance(&mut self, id: ServiceId, at: (InstanceId, IpAddr)) {
        let mut map = self.addrs.lock().unwrap();
        let bag = map.entry(id).or_default();
        bag.instances.push(at);
    }

    #[allow(dead_code)]
    pub fn drop_instance(&mut self, id: &ServiceId, at: (InstanceId, IpAddr)) {
        let mut map = self.addrs.lock().unwrap();
        let Some(bag) = map.get_mut(id) else {
            return;
        };
        bag.instances
            .retain(|(inst, addr)| inst == &at.0 && addr == &at.1);
    }
}

pub async fn proxy(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(balancer): State<BalancerState>,
    mut req: Request,
) -> http::Result<impl IntoResponse> {
    let service = extract_service_id(&mut req)?;

    let (instance, server_addr) = balancer.next(&service);

    *req.uri_mut() = {
        let uri = req.uri();
        let mut parts = uri.clone().into_parts();
        parts.authority = Authority::from_str(&server_addr.to_string()).ok();
        parts.scheme = Some(Scheme::HTTP);
        Uri::from_parts(parts).unwrap()
    };

    req.headers_mut().insert(
        PROXY_INSTANCE_HEADER_NAME,
        HeaderValue::from_str(&instance.to_string()).unwrap(),
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
