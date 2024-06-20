use std::net::IpAddr;

use clap::{Parser, Subcommand};
use eyre::Ok;
use prettytable::{cell::Cell, row::Row, Table};
use proto::clients::CtlClient;

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

fn print_table(workers: Vec<IpAddr>) {
    let mut table = Table::new();
    for (i, addr) in workers.into_iter().enumerate() {
        table.add_row(Row::new(vec![
            Cell::new(format!("worker_{i}").as_str()),
            Cell::new(format!("{addr}").as_str()),
        ]));
    }
    table.printstd();
}
