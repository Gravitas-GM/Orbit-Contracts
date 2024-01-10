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
    pub fn get_template_path(&self, contract: &Contract) -> &'static Path {
        match self {
            Self::Php => php::get_template_path(contract),
            Self::Rust => rust::get_template_path(contract),
            Self::Typescript => typescript::get_template_path(contract),
        }
    }

    pub fn all() -> Vec<Self> {
        use Language::*;
        const ITEMS: [Language; 3] = [Php, Rust, Typescript];

        ITEMS.to_vec()
    }
}
