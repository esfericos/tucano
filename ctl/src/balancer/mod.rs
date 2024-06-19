#![allow(dead_code)]

use std::{
    collections::HashMap,
    net::SocketAddr
};

use axum::extract::Request;
use proto::common::instance::InstanceId;
use proto::common::service::ServiceId;

struct Balancer<S> {
    strategy: S,
    addrs: HashMap<ServiceId, Vec<(InstanceId, SocketAddr)>>
}

trait Strategy {
    async fn get_server(&self, _req: &Request) -> (InstanceId, SocketAddr);
}

impl<S> Balancer<S>
where
    S: Strategy
{
    pub async fn run() {
        todo!();
    }

    async fn next_server(&self, _req: &Request) -> (InstanceId, SocketAddr) {
        todo!();
    }

    pub async fn drop_instance(&self, _id: InstanceId) {
        todo!();
    }
    
    pub async fn add_instance(&self, _id: InstanceId, _at: SocketAddr) {
        todo!();
    }

    pub async fn swap_instance(_old_id: InstanceId, _new_id: InstanceId, _new_at: SocketAddr) {
        todo!();
    }
}
