use eyre::Result;
use proto::common::node::Metrics;
use reqwest::{Client, Url};

// A connection pool that sends the collected `Metrics` to the `http` component.
pub struct Monitor {
    client: Client,
    base_url: String,
}

impl Monitor {
    // Instantiates a new `Monitor`.
    //
    //  Sends requests containing a JSON with the collected metrics
    //   to the `http` component from `ctl` node.
    //
    // **⚠ Instantiate as mutable**
    //
    // | Keep in mind that you only need to instantiate `Monitor` ONCE.
    pub fn new(url: &str) -> Result<Self> {
        let client = Client::builder().build()?;

        let base_url = Url::parse(url)?.to_string();

        Ok(Self { client, base_url })
    }

    // Sends a POST request containing `Metrics` to the `http` component.
    pub async fn send_request(&mut self, metrics: &Metrics) -> Result<()> {
        self.client
            .post(&self.base_url)
            .json(metrics)
            .send()
            .await?;

        Ok(())
    }
}
