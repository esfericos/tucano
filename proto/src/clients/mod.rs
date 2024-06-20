mod ctl;
use std::time::Duration;

pub use ctl::CtlClient;

mod worker;
use eyre::Context as _;
use serde::{de::DeserializeOwned, Serialize};
pub use worker::WorkerClient;

#[derive(Clone)]
pub struct BaseClient {
    client: reqwest::Client,
}

impl BaseClient {
    #[must_use]
    fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(20))
            .build()
            .unwrap();
        Self { client }
    }

    /// Sends a request to the given path, on the given worker.
    ///
    /// Paths must start with a `/`.
    async fn send<Req, Res>(&self, url: impl AsRef<str>, body: &Req) -> eyre::Result<Res>
    where
        Req: Serialize,
        Res: DeserializeOwned,
    {
        let res = self
            .client
            .post(url.as_ref())
            .json(body)
            .send()
            .await
            .wrap_err("failed to send request to worker")?
            .json::<Res>()
            .await
            .wrap_err("failed to parse response from worker")?;
        Ok(res)
    }
}
