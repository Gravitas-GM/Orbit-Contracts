use crate::contract::Contract;
use std::path::Path;

pub fn get_template_path(contract: &Contract) -> &'static Path {
    use Contract::*;

    match contract {
        Permission => Path::new("rust/permission.rs.handlebars"),
        Role => Path::new("rust/role.rs.handlebars"),
    }
}
