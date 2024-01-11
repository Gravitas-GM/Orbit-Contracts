use crate::cli::CliArgs;
use crate::contract::Contract;
use crate::error::Error;
use crate::lang::Language;
use clap::Parser;

mod cli;
mod contract;
mod error;
mod lang;
mod template;

fn main() -> Result<(), Error> {
    env_logger::init();

    let mut cli = CliArgs::parse();

    let contracts = cli.contract.take().unwrap_or_else(Contract::all);
    let languages = cli.lang.take().unwrap_or_else(Language::all);

    template::build(&contracts, &languages, cli.into())
}
