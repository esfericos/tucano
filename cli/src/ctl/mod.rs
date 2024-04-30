use std::net::SocketAddr;

use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Cmd {
    #[command(arg_required_else_help = true)]
    Node(NodeArgs),
    #[command(arg_required_else_help = true)]
    Service(ServiceArgs),
    #[command(arg_required_else_help = true)]
    Deploy(DeployArgs),
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
pub struct NodeArgs {
    #[arg(short, long)]
    list: bool,
    #[arg(short, long)]
    show: SocketAddr,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
pub struct ServiceArgs {
    #[arg(short, long)]
    list: bool,
    #[arg(short, long)]
    show: String,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
pub struct DeployArgs {
    #[arg(short, long)]
    list: bool,
    #[arg(short, long)]
    show: String,
}
