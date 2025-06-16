use crate::contract::Contract;
use std::path::Path;

pub fn get_template_path(contract: &Contract) -> Option<&'static Path> {
    use Contract::*;

    match contract {
        Permission => Some(Path::new("typescript/permission.ts.hbs")),
        Role => Some(Path::new("typescript/role.ts.hbs")),
        WeekDay
        | HubUser
        | HubAccount
        | HubDepartment
        | HubFeedEntry
        | HubFeedEntryPayload
        | HubFeedEntryRecipientPayload
        | JwtClaim
        | FeedRecipientKind => None,
    }
}
