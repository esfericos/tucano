use std::net::IpAddr;

use clap::{Parser, Subcommand};
use eyre::Ok;
use proto::clients::CtlClient;
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
    Show { id: String },
    Deploy { id: String, image: String },
    Terminate { id: String },
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let cli = Cli::parse();
    let ctl_client = CtlClient::new(cli.ctl_addr);

    match cli.cmd {
        Cmd::Node(cmd) => handle_node(&cmd, ctl_client).await?,
        Cmd::Service(cmd) => handle_service(&cmd)?,
    }

    Ok(())
}

async fn handle_node(cmd: &NodeCmd, ctl_client: CtlClient) -> eyre::Result<()> {
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

fn handle_service(cmd: &ServiceCmd) -> eyre::Result<()> {
    match cmd {
        ServiceCmd::List => todo!(),
        ServiceCmd::Show { .. } => todo!(),
        ServiceCmd::Deploy { .. } => todo!(),
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
