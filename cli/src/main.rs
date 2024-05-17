use clap::{Args, Parser, Subcommand};
use ctl::handle_deploy;

mod ctl;
mod worker;

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, Subcommand)]
enum Cmd {
    Ctl(Ctl),
    Worker(Worker),
}

#[derive(Args, Debug)]
struct Ctl {
    #[command(subcommand)]
    cmd: ctl::Cmd,
}

#[derive(Args, Debug)]
struct Worker {
    #[command(subcommand)]
    cmd: worker::Cmd,
}

fn main() {
    let cli = Cli::parse();

    match cli.cmd {
        Cmd::Ctl(ctl) => handle_ctl(ctl),
        Cmd::Worker(_) => todo!(),
    }
}

fn handle_ctl(ctl: Ctl) {
    match ctl.cmd {
        ctl::Cmd::Node(_) => todo!(),
        ctl::Cmd::Service(_) => todo!(),
        ctl::Cmd::Deploy(deploy) => handle_deploy(deploy),
    }
}
