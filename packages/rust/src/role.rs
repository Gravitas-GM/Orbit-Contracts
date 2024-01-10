use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum Role {
	#[serde(rename = "ROLE_ADMIN")]
	Admin,
	#[serde(rename = "ROLE_USER")]
	User,
	#[serde(rename = "ROLE_SERVICE")]
	Service,
}
