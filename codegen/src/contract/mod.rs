use crate::error::Error;
use crate::template::TemplateContext;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::path::Path;

mod permission;
mod role;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, ValueEnum)]
pub enum Contract {
    Permission,
    Role,
}

impl Contract {
    pub fn build_context(&self, data_dir: &Path) -> Result<TemplateContext, Error> {
        match self {
            Self::Permission => permission::build_context(data_dir),
            Self::Role => role::build_context(data_dir),
        }
    }

    pub fn all() -> Vec<Self> {
        use Contract::*;
        const ITEMS: [Contract; 2] = [Permission, Role];

        ITEMS.to_vec()
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct EnumDescriptor {
    key: String,
    value: String,
    description: Option<String>,
}
