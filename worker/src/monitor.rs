use eyre::Result;
use proto::common::node::Metrics;
use reqwest::{
    Client,
    Url
};

// A connection pool that sends metrics to the http component
// from time to time.
pub struct Monitor {
    client: Client,
    base_url: String,
}

impl Monitor {
    // Instantiates a new Monitor.
    //  *Creates a default header used by the http component
    //   to manage data routing.
    pub fn new(url: String) -> Result<Self> {
        
        let client = Client::builder()
            .build()?;

        let base_url = Url::parse(&url)?.to_string();

        Ok(Self { client, base_url})
    }

    // Sends a POST request containing metrics data
    // to the http component.
    pub async fn send_request(&mut self, metrics: &Metrics) -> Result<()> {
        self.client.post(&self.base_url)
            .json(metrics)
            .send()
            .await?;

        Ok(())
    }
}