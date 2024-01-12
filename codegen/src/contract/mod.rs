use crate::error::Error;
use crate::template::TemplateContext;
use clap::ValueEnum;
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
    pub fn build_context(&self, data_dir: &Path) -> Result<TemplateContext, Error> {
        match self {
            Self::Permission => context::for_enum(data_dir.join("permissions.json")),
            Self::Role => context::for_enum(data_dir.join("roles.json")),
            Self::HubUser => context::for_struct("clients/hub/models/hub_user.json"),
            Self::HubAccount => context::for_struct("clients/hub/models/hub_account.json"),
            Self::WeekDay => context::for_enum(data_dir.join("week_day.json")),
        }
    }
}
