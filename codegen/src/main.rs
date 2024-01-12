use crate::cli::CliArgs;
use crate::contract::Contract;
use crate::error::Error;
use crate::lang::Language;
use clap::{Parser, ValueEnum};

mod cli;
mod contract;
mod error;
mod lang;
mod template;

fn main() -> Result<(), Error> {
    env_logger::init();

    let mut cli = CliArgs::parse();

    let contracts = cli.contract.take();
    let contracts = contracts
        .as_deref()
        .unwrap_or_else(Contract::value_variants);

    let languages = cli.lang.take();
    let languages = languages
        .as_deref()
        .unwrap_or_else(Language::value_variants);

    template::build(contracts, languages, cli.into())
}
