use crate::contract::Contract;
use std::path::Path;

pub fn get_template_path(contract: &Contract) -> Option<&'static Path> {
    use Contract::*;

    match contract {
        Permission => Some(Path::new("php/Permission.php.handlebars")),
        Role => Some(Path::new("php/Role.php.handlebars")),
        HubUser => Some(Path::new("php/Clients/Hub/Models/HubUser.php.handlebars")),
        HubAccount => Some(Path::new(
            "php/Clients/Hub/Models/HubAccount.php.handlebars",
        )),
    }
}
