use clap::{Parser, Subcommand, ValueEnum};

/// Simple program to greet a person
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: CliCommands,
}

#[derive(Subcommand)]

pub enum CliCommands {
    /// Login to your Plexo instance.
    Login,
    /// List one or more resources.
    Get { resource: Resource },
    /// Create one or more resources from a file or stdin.
    Create,
    /// Set resources either from a file, stdin, or specifying label selectors, names, resource selectors, or resources.
    Set,
    /// Delete resources either from a file, stdin, or specifying label selectors, names, resource selectors, or resources.
    Delete,
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq)]
pub enum Resource {
    Tasks,
    Projects,
    Teams,
    Members,
    Labels,
}
