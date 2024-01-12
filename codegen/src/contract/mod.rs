use crate::error::Error;
use crate::template::TemplateContext;
use clap::ValueEnum;
use context::{for_enum, for_struct};
use std::path::Path;

pub mod context;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, ValueEnum)]
pub enum Contract {
    Permission,
    Role,
    HubUser,
    HubAccount,
    WeekDay,
}

impl Contract {
    pub fn build_context(&self, root: &Path) -> Result<TemplateContext, Error> {
        match self {
            Self::Permission => for_enum(root.join("permissions.json")),
            Self::Role => for_enum(root.join("roles.json")),
            Self::HubUser => for_struct(root.join("clients/hub/models/hub_user.json")),
            Self::HubAccount => for_struct(root.join("clients/hub/models/hub_account.json")),
            Self::WeekDay => for_enum(root.join("week_day.json")),
        }
    }
}
