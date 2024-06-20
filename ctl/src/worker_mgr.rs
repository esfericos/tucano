use std::{
    collections::{hash_map::Entry, HashMap},
    net::IpAddr,
    time::{Duration, Instant},
};

use proto::{
    common::node::Metrics,
    ctl::worker::{HelloStatus, PushMetricsStatus},
};
use tokio::{
    select,
    sync::{mpsc, oneshot},
    time,
};
use tracing::{info, instrument, trace, warn};

pub struct WorkerMgr {
    rx: mpsc::Receiver<Msg>,
    handle: WorkerMgrHandle,
    workers: HashMap<IpAddr, WorkerDetails>,
    liveness_timeout: Duration,
}

#[derive(Debug, Clone)]
pub struct WorkerDetails {
    pub addr: IpAddr,
    pub metrics: Metrics,
    pub collected_at: Instant,
}

impl WorkerMgr {
    #[must_use]
    pub fn new(liveness_timeout: Duration) -> (WorkerMgr, WorkerMgrHandle) {
        let (tx, rx) = mpsc::channel(16);
        let handle = WorkerMgrHandle(tx);
        let actor = WorkerMgr {
            rx,
            handle: handle.clone(),
            workers: HashMap::default(),
            liveness_timeout,
        };
        (actor, handle)
    }

    pub async fn run(mut self) {
        let mut interval = time::interval(self.liveness_timeout);
        loop {
            select! {
                Some(msg) = self.rx.recv() => {
                    // Attention to back pressure.
                    self.handle_msg(msg).await;
                }
                inst = interval.tick() => {
                    self.handle_msg(Msg::Tick(inst.into_std())).await;
                }
            }
        }
    }

    async fn handle_msg(&mut self, msg: Msg) {
        trace!(?msg, "got msg");
        match msg {
            Msg::Hello(addr, reply) => {
                _ = reply.send(self.handle_hello(addr));
            }
            Msg::Bye(addr) => {
                self.handle_bye(addr);
            }
            Msg::PushMetrics(a, m, reply) => {
                _ = reply.send(self.handle_push_metrics(a, m));
            }
            Msg::QueryWorkers(reply) => {
                let workers = self.workers.values().cloned().collect();
                _ = reply.send(workers);
            }
            Msg::Tick(instant) => {
                self.handle_tick(instant).await;
            }
        }
    }

    #[instrument(skip(self))]
    fn handle_hello(&mut self, addr: IpAddr) -> HelloStatus {
        match self.workers.entry(addr) {
            Entry::Occupied(_) => {
                warn!("unnecessary hello operation");
                HelloStatus::AlreadyRegistered
            }
            Entry::Vacant(entry) => {
                info!("worker joined");
                entry.insert(WorkerDetails {
                    addr,
                    metrics: Metrics::default(),
                    collected_at: Instant::now(),
                });
                HelloStatus::Ok
            }
        }
        // TODO: Notify interested parties
    }

    #[instrument(skip(self))]
    fn handle_bye(&mut self, addr: IpAddr) {
        let opt = self.workers.remove(&addr);
        info!("removed worker from ctl pool");
        if opt.is_none() {
            warn!("worker wasn't registered");
        }
    }

    #[instrument(skip(self, metrics))]
    fn handle_push_metrics(&mut self, addr: IpAddr, metrics: Metrics) -> PushMetricsStatus {
        let Some(details) = self.workers.get_mut(&addr) else {
            warn!("received metrics from removed worker");
            return PushMetricsStatus::Removed;
        };
        details.metrics = metrics;
        details.collected_at = Instant::now();
        PushMetricsStatus::Ack
    }

    async fn handle_tick(&mut self, instant: Instant) {
        // For the purposes of this routine, we assume that `instant` occurs
        // AFTER every `worker`'s `collected_at` instant.
        for worker in self.workers.values() {
            let maybe_elapsed = instant.checked_duration_since(worker.collected_at);
            let Some(elapsed) = maybe_elapsed else {
                // collected_at occurred after instant, so the worker is alive
                continue;
            };
            if elapsed < self.liveness_timeout {
                // elapsed time is within the timeout bounds, so worker is alive
                continue;
            }
            // worker is most possibly dead, send a bye
            self.handle.send(Msg::Bye(worker.addr)).await;
        }
    }
}

#[derive(Clone)]
pub struct WorkerMgrHandle(mpsc::Sender<Msg>);

impl WorkerMgrHandle {
    async fn send(&self, msg: Msg) {
        _ = self.0.send(msg).await;
    }

    /// Sends a message and waits for a reply.
    async fn send_wait<F, R>(&self, f: F) -> R
    where
        F: FnOnce(oneshot::Sender<R>) -> Msg,
    {
        let (tx, rx) = oneshot::channel();
        self.send(f(tx)).await;
        rx.await.expect("actor must be alive")
    }

    pub async fn hello(&self, addr: IpAddr) -> HelloStatus {
        self.send_wait(|r| Msg::Hello(addr, r)).await
    }

    pub async fn bye(&self, addr: IpAddr) {
        self.send(Msg::Bye(addr)).await;
    }

    pub async fn push_metrics(&self, addr: IpAddr, metrics: Metrics) -> PushMetricsStatus {
        self.send_wait(|r| Msg::PushMetrics(addr, metrics, r)).await
    }

    #[allow(dead_code)]
    pub async fn query_workers(&self) -> Vec<WorkerDetails> {
        self.send_wait(Msg::QueryWorkers).await
    }
}

#[derive(Debug)]
enum Msg {
    Hello(IpAddr, oneshot::Sender<HelloStatus>),
    Bye(IpAddr),
    PushMetrics(IpAddr, Metrics, oneshot::Sender<PushMetricsStatus>),
    QueryWorkers(oneshot::Sender<Vec<WorkerDetails>>),
    Tick(Instant),
}
