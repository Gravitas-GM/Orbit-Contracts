use crate::contract::Contract;
use std::path::Path;

pub fn get_template_path(contract: &Contract) -> &'static Path {
    use Contract::*;

    match contract {
        Permission => Path::new("php/Permission.php.handlebars"),
        Role => Path::new("php/Role.php.handlebars"),
    }
}
