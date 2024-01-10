use crate::contract::Contract;
use crate::lang::Language;
use crate::template::Config;
use clap::Parser;
use std::path::PathBuf;

/// Builds shared contract packages for the Happy Orbit project.
#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct CliArgs {
    /// The path to the template root; this should be where directories such as "php" and "rust"
    /// are located
    #[arg(long)]
    pub templates: Option<PathBuf>,

    /// The path to the data directory
    #[arg(long)]
    pub contexts: Option<PathBuf>,

    /// The path to the package output directory
    #[arg(long)]
    pub output: Option<PathBuf>,

    /// Which contracts to build; if omitted, all contracts will be built
    #[arg(short, long)]
    pub contract: Option<Vec<Contract>>,

    /// Which languages to build contracts for; if omitted, all languages will be built
    #[arg(short, long)]
    pub lang: Option<Vec<Language>>,
}

impl From<CliArgs> for Config {
    fn from(value: CliArgs) -> Self {
        Self {
            template_dir: value.templates,
            data_dir: value.contexts,
            output_dir: value.output,
        }
    }
}
