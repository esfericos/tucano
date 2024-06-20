use std::{
    collections::HashMap,
    net::IpAddr,
    str::FromStr as _,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};

use axum::{
    body::Body,
    extract::{Request, State},
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
    well_known::PROXY_INSTANCE_HEADER_NAME,
};
use utils::http::{self, OptionExt as _, ResultExt as _};

pub struct InstanceBag {
    pub instances: Vec<(InstanceId, IpAddr)>,
    pub count: AtomicUsize,
}

#[derive(Clone)]
pub struct Balancer {
    pub addrs: Arc<Mutex<HashMap<ServiceId, InstanceBag>>>,
}

impl Balancer {
    pub fn new() -> Self {
        Balancer {
            addrs: Arc::new(Mutex::new(HashMap::default())),
        }
    }

    pub fn next(&self, service: &ServiceId) -> (InstanceId, IpAddr) {
        let map = self.addrs.lock().unwrap();
        let bag = map.get(service).unwrap();
        let count = bag.count.fetch_add(1, Ordering::Relaxed);
        bag.instances[count % bag.instances.len()]
    }
}

#[derive(Clone)]
pub struct BalancerState {
    pub balancer: Balancer,
    pub client: Client<HttpConnector, Body>,
}

impl BalancerState {
    #[must_use]
    pub fn new() -> Self {
        BalancerState {
            balancer: Balancer::new(),
            client: {
                let mut connector = HttpConnector::new();
                connector.set_keepalive(Some(Duration::from_secs(60)));
                connector.set_nodelay(true);
                Client::builder(TokioExecutor::new()).build::<_, Body>(connector)
            },
        }
    }
}

#[axum::debug_handler]
pub async fn proxy(
    State(state): State<BalancerState>,
    mut req: Request,
) -> http::Result<impl IntoResponse> {
    let service = extract_service_id(&mut req)?;

    let (instance, server) = state.balancer.next(&service);

    *req.uri_mut() = {
        let uri = req.uri();
        let mut parts = uri.clone().into_parts();
        parts.authority = Authority::from_str(&format!("{server}")).ok();
        parts.scheme = Some(Scheme::HTTP);
        Uri::from_parts(parts).unwrap()
    };

    req.headers_mut().insert(
        PROXY_INSTANCE_HEADER_NAME,
        HeaderValue::from_str(&format!("{instance}")).unwrap(),
    );

    state
        .client
        .request(req)
        .await
        .http_error(StatusCode::BAD_GATEWAY, "bad gateway")
}

fn extract_service_id(req: &mut Request) -> http::Result<ServiceId> {
    let inner = req
        .headers()
        .get("host")
        .unwrap()
        .to_str()
        .ok()
        .and_then(|s| s.parse().ok())
        .or_http_error(StatusCode::BAD_REQUEST, "invalid service name")?;
    Ok(ServiceId(inner))
}
