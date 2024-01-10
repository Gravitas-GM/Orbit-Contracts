use crate::contract::Contract;
use std::path::Path;

pub fn get_template_path(contract: &Contract) -> &'static Path {
    use Contract::*;

    match contract {
        Permission => Path::new("typescript/permission.ts.handlebars"),
        Role => Path::new("typescript/role.ts.handlebars"),
    }
}
