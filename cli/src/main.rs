use clap::{Args, Parser, Subcommand};

mod ctl;
mod worker;

#[derive(Debug, Parser)]
struct Cli {
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
    cmd: ctl::Cmd,
}

#[derive(Args, Debug)]
struct Worker {
    cmd: worker::Cmd,
}

fn main() {}
