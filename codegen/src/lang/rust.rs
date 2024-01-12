use crate::contract::Contract;
use std::path::Path;

pub fn get_template_path(contract: &Contract) -> Option<&'static Path> {
    use Contract::*;

    match contract {
        Permission => Some(Path::new("rust/permission.rs.hbs")),
        Role => Some(Path::new("rust/role.rs.hbs")),
        HubUser | HubAccount | WeekDay => None,
    }
}
