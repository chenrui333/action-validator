use clap::Parser;
use std::path::PathBuf;

/// Minimal struct for Clap usage, without Serde
#[derive(Parser, Debug)]
#[command(
    name = "action-validator",
    about = "A validator for GitHub Action and Workflow YAML files",
    version
)]
pub struct CliConfig {
    /// Be more verbose
    #[arg(short, long)]
    pub verbose: bool,

    /// Input file(s)
    #[arg(name = "path_to_action_yaml")]
    pub src: Vec<PathBuf>,
}
