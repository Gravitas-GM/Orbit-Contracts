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
    HubDepartment,
    HubFeedEntry,
    HubFeedEntryPayload,
    HubFeedEntryRecipientPayload,
    WeekDay,
    JwtClaim,
    FeedRecipientKind,
}

impl Contract {
    pub fn build_context(&self, root: &Path) -> Result<TemplateContext, Error> {
        use Contract::*;

        match self {
            Permission => for_enum(root.join("permissions.json")),
            Role => for_enum(root.join("roles.json")),
            HubUser => for_struct(root.join("clients/hub/models/hub_user.json")),
            HubAccount => for_struct(root.join("clients/hub/models/hub_account.json")),
            HubDepartment => for_struct(root.join("clients/hub/models/hub_department.json")),
            HubFeedEntry => for_struct(root.join("clients/hub/models/hub_feed_entry.json")),
            HubFeedEntryPayload => {
                for_struct(root.join("clients/hub/models/hub_feed_entry_payload.json"))
            }
            HubFeedEntryRecipientPayload => {
                for_struct(root.join("clients/hub/models/hub_feed_entry_recipient_payload.json"))
            }
            WeekDay => for_enum(root.join("week_day.json")),
            JwtClaim => for_enum(root.join("jwt_claims.json")),
            FeedRecipientKind => for_enum(root.join("feed_recipient_kind.json")),
        }
    }
}
