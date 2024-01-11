use crate::error::Error;
use crate::template::TemplateContext;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::path::Path;

mod hub_account;
mod hub_user;
mod permission;
mod role;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, ValueEnum)]
pub enum Contract {
    Permission,
    Role,
    HubUser,
    HubAccount,
}

impl Contract {
    pub fn build_context(&self, data_dir: &Path) -> Result<TemplateContext, Error> {
        match self {
            Self::Permission => permission::build_context(data_dir),
            Self::Role => role::build_context(data_dir),
            Self::HubUser => hub_user::build_context(data_dir),
            Self::HubAccount => hub_account::build_context(data_dir),
        }
    }

    pub fn all() -> Vec<Self> {
        use Contract::*;
        const ITEMS: [Contract; 4] = [Permission, Role, HubUser, HubAccount];

        ITEMS.to_vec()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EnumDescriptor {
    key: String,
    value: String,
    description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FieldDescriptor {
    pub field: String,
    pub kind: String,

    #[serde(rename = "innerKind")]
    pub inner_kind: Option<String>,
}

#[derive(Debug)]
pub enum FieldKind<'a> {
    String,
    U32,
    Array(Option<&'a str>),
    Class(&'a str),
}

impl<'a> FieldKind<'a> {
    pub fn from_descriptor(descriptor: &'a FieldDescriptor) -> Self {
        let kind = descriptor.kind.to_ascii_lowercase();

        match kind.as_str() {
            "string" => Self::String,
            "u32" => Self::U32,
            "array" => Self::Array(descriptor.inner_kind.as_deref()),
            _ => Self::Class(descriptor.kind.as_str()),
        }
    }
}
