use std::{net::SocketAddr, path::PathBuf};

use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Cmd {
    #[command(arg_required_else_help = true)]
    Node(NodeArgs),
    #[command(arg_required_else_help = true)]
    Service(ServiceArgs),
    #[command(arg_required_else_help = true)]
    Deploy(Deploy),
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
pub struct Deploy {
    #[command(subcommand)]
    cmd: DeployCmd,
}

#[derive(Debug, Subcommand)]
pub enum DeployCmd {
    List,
    Show { node: String },
    New { path: PathBuf, tag: String },
}

pub fn handle_deploy(deploy: Deploy) {
    match deploy.cmd {
        DeployCmd::List => todo!(),
        DeployCmd::Show { node } => todo!(),

        /// The `new` subcommand requires the following @params:
        /// path: PathBuf to the directory containing the Dockerfile for the image
        /// tag: String for image's name
        DeployCmd::New { path, tag } => todo!(),
    }
}
