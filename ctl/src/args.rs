use clap::Parser;

#[derive(Debug, Parser)]
pub struct CtlArgs {
    /// This controller node's HTTP server port.
    ///
    /// If not provided, a random port will be chosen.
    #[arg(long)]
    pub http_port: Option<u16>,

    /// This controller node's Balancer server port.
    ///
    /// If not provided, a random port will be chosen.
    #[arg(long)]
    pub balancer_port: Option<u16>,
}
