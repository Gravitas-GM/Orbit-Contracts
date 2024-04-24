use crate::contract::Contract;
use std::path::Path;

pub fn get_template_path(contract: &Contract) -> Option<&'static Path> {
    use Contract::*;

    match contract {
        Permission => Some(Path::new("php/Permission.php.hbs")),
        Role => Some(Path::new("php/Role.php.hbs")),
        HubUser => Some(Path::new("php/Clients/Hub/Models/HubUser.php.hbs")),
        HubAccount => Some(Path::new("php/Clients/Hub/Models/HubAccount.php.hbs")),
        HubDepartment => Some(Path::new("php/Clients/Hub/Models/HubDepartment.php.hbs")),
        WeekDay => Some(Path::new("php/WeekDay.php.hbs")),
        JwtClaim => Some(Path::new("php/JwtClaim.php.hbs")),
    }
}
