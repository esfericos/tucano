use clap::{Args, Parser, Subcommand};

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

fn main() {}
