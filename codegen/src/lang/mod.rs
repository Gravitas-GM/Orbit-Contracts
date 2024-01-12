use crate::contract::Contract;
use clap::ValueEnum;
use std::path::Path;

mod php;
mod rust;
mod typescript;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, ValueEnum)]
pub enum Language {
    Php,
    Rust,
    Typescript,
}

impl Language {
    pub fn get_template_path(&self, contract: &Contract) -> Option<&'static Path> {
        match self {
            Self::Php => php::get_template_path(contract),
            Self::Rust => rust::get_template_path(contract),
            Self::Typescript => typescript::get_template_path(contract),
        }
    }
}
