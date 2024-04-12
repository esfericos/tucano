use std::{collections::HashMap, net::SocketAddr};

use proto::common::node::Metrics;
use tokio::sync::{mpsc, oneshot};

pub struct Discovery {
    rx: mpsc::Receiver<Msg>,
    // TODO: Add more information on workers
    workers: HashMap<SocketAddr, Metrics>,
}

impl Discovery {
    #[must_use]
    pub fn new() -> (Discovery, DiscoveryHandle) {
        let (tx, rx) = mpsc::channel(10);
        let actor = Discovery {
            rx,
            workers: HashMap::default(),
        };
        let handle = DiscoveryHandle(tx);
        (actor, handle)
    }

    pub async fn run(mut self) {
        while let Some(msg) = self.rx.recv().await {
            // Attention to back pressure.
            self.handle_msg(msg).await;
        }
    }

    async fn handle_msg(&mut self, msg: Msg) {
        match msg {
            Msg::WorkerAdd(addr, metrics) => {
                self.workers.insert(addr, metrics);
            }
            Msg::WorkerDrop(addr) => {
                self.workers.remove(&addr);
            }
            Msg::WorkerQuery(reply) => {
                let entries = self
                    .workers
                    .iter()
                    .map(|(&addr, metrics)| WorkerDetails {
                        addr,
                        metrics: metrics.clone(),
                    })
                    .collect();
                _ = reply.send(entries);
            }
        }
    }
}

#[derive(Clone)]
pub struct DiscoveryHandle(mpsc::Sender<Msg>);

impl DiscoveryHandle {
    async fn send(&self, msg: Msg) {
        _ = self.0.send(msg).await;
    }

    #[allow(dead_code)] // TODO: Remove
    pub async fn worker_add(&self, addr: SocketAddr, metrics: Metrics) {
        self.send(Msg::WorkerAdd(addr, metrics)).await;
    }

    #[allow(dead_code)] // TODO: Remove
    pub async fn worker_drop(&self, addr: SocketAddr) {
        self.send(Msg::WorkerDrop(addr)).await;
    }

    #[allow(dead_code)] // TODO: Remove
    pub async fn worker_query(&self) -> Vec<WorkerDetails> {
        let (tx, rx) = oneshot::channel();
        self.send(Msg::WorkerQuery(tx)).await;
        rx.await.expect("actor must be alive")
    }
}

#[derive(Debug)]
enum Msg {
    WorkerAdd(SocketAddr, Metrics),
    WorkerDrop(SocketAddr),
    WorkerQuery(oneshot::Sender<Vec<WorkerDetails>>),
}

#[derive(Debug)]
pub struct WorkerDetails {
    pub addr: SocketAddr,
    pub metrics: Metrics,
}
