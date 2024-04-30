use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Cmd {
    Node,
    Service,
    Deploy,
}
