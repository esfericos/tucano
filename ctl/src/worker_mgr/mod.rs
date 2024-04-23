use std::net::SocketAddr;

use proto::common::node::Metrics;
use tokio::sync::mpsc;

struct WorkerMgr {
    rx: mpsc::Receiver<Msg>,
}

impl WorkerMgr {
    #[must_use]
    pub fn new() -> (WorkerMgr, WorkerMgrHandle) {
        let (tx, rx) = mpsc::channel(10);
        let actor = WorkerMgr { rx };
        let handle = WorkerMgrHandle(tx);
        (actor, handle)
    }

    pub async fn run(mut self) {
        while let Some(msg) = self.rx.recv().await {
            self.handle_msg(msg).await;
        }
    }

    async fn handle_msg(&mut self, msg: Msg) {
        match msg {
            Msg::MetricReport(_addr, _metrics) => {
                unimplemented!()
            }
        }
    }
}

#[derive(Clone)]
struct WorkerMgrHandle(mpsc::Sender<Msg>);

impl WorkerMgrHandle {
    async fn send(&self, msg: Msg) {
        _ = self.0.send(msg).await;
    }
}

enum Msg {}
