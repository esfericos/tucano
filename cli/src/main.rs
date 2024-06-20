use std::net::IpAddr;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
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

fn main() {
    let cli = Cli::parse();

    match cli.cmd {
        Cmd::Node(cmd) => handle_node(&cmd),
        Cmd::Service(cmd) => handle_service(&cmd),
    }
}

fn handle_node(cmd: &NodeCmd) {
    match cmd {
        NodeCmd::List => todo!(),
        NodeCmd::Show { .. } => todo!(),
        NodeCmd::Worker(_) => todo!(),
    }
}

fn handle_service(cmd: &ServiceCmd) {
    match cmd {
        ServiceCmd::List => todo!(),
        ServiceCmd::Show { .. } => todo!(),
        ServiceCmd::Deploy { .. } => todo!(),
        ServiceCmd::Terminate { .. } => todo!(),
    }
}
