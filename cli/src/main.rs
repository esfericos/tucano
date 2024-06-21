use std::net::IpAddr;

use clap::{Parser, Subcommand};
use proto::{
    clients::CtlClient,
    common::service::{ResourceConfig, ServiceId, ServiceImage, ServiceSpec},
    ctl::deployer::RedeploymentPolicy,
};
use tabled::{self, Table, Tabled};

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
    #[arg(short, long)]
    ctl_addr: IpAddr,
}

#[derive(Debug, Subcommand)]
pub enum Cmd {
    #[clap(subcommand)]
    Node(NodeCmd),
    #[clap(subcommand)]
    Service(ServiceCmd),
}

#[derive(Debug, Subcommand)]
pub enum NodeCmd {
    List,
    Show {
        address: IpAddr,
    },
    #[clap(subcommand)]
    Worker(WorkerCmd),
}

#[derive(Debug, Subcommand)]
pub enum WorkerCmd {
    Remove { address: IpAddr },
}

#[derive(Debug, Subcommand)]
pub enum ServiceCmd {
    List,
    Show {
        id: String,
    },
    Deploy {
        #[arg(long)]
        id: String,
        #[arg(long)]
        image: String,
        #[arg(long)]
        public: bool,
        #[arg(long)]
        concurrency: u32,
        // #[arg(long)]
        // cpu_shares: i64,
        // #[arg(long)]
        // memory_limit: i64,
    },
    Terminate {
        id: String,
    },
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let cli = Cli::parse();
    let ctl_client = CtlClient::new(cli.ctl_addr);
    match cli.cmd {
        Cmd::Node(cmd) => handle_node(cmd, ctl_client).await?,
        Cmd::Service(cmd) => handle_service(cmd, ctl_client).await?,
    }
    Ok(())
}

async fn handle_node(cmd: NodeCmd, ctl_client: CtlClient) -> eyre::Result<()> {
    match cmd {
        NodeCmd::List => {
            let workers = ctl_client.query_workers().await.unwrap().workers;
            print_table(workers);
            Ok(())
        }
        NodeCmd::Show { .. } => todo!(),
        NodeCmd::Worker(_) => todo!(),
    }
}

async fn handle_service(cmd: ServiceCmd, ctl_client: CtlClient) -> eyre::Result<()> {
    match cmd {
        ServiceCmd::List => todo!(),
        ServiceCmd::Show { .. } => todo!(),
        ServiceCmd::Deploy {
            id,
            image,
            public,
            concurrency,
        } => {
            let spec = ServiceSpec {
                service_id: ServiceId(id),
                image: ServiceImage(image),
                public,
                concurrency,
                resource_config: ResourceConfig {
                    // These are being ignored by the server, hence we may mock
                    // them here.
                    cpu_shares: 0,
                    memory_limit: 0,
                },
            };
            let rd = RedeploymentPolicy::None;
            let res = ctl_client.deploy_service(spec, rd).await?;
            println!("Successfully deployed service #{}", res.deployment_id);
            Ok(())
        }
        ServiceCmd::Terminate { .. } => todo!(),
    }
}

fn print_table(addrs: Vec<IpAddr>) {
    #[derive(Tabled)]
    pub struct WorkerTable {
        name: String,
        addr: IpAddr,
    }

    let mut workers: Vec<WorkerTable> = Vec::new();
    for (i, addr) in addrs.into_iter().enumerate() {
        workers.push(WorkerTable {
            name: format!("worker-{}", i + 1),
            addr,
        });
    }
    let table = Table::new(workers).to_string();
    print!("{table}");
}
