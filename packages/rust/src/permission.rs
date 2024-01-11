use serde::{Deserialize, Serialize};
use std::borrow::Borrow;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum Permission {
	#[serde(rename = "admin")]
	Admin,
	#[serde(rename = "manage quiz")]
	ManageQuiz,
	#[serde(rename = "manage user")]
	ManageUser,
	#[serde(rename = "manage account")]
	ManageAccount,
	#[serde(rename = "manage survey")]
	ManageSurvey,
}

impl Permission {
	pub fn is_allowed<P: Borrow<Self>>(permission: P, permissions: &[Self]) -> bool {
		permissions.contains(&Permission::Admin) || permissions.contains(permission.borrow())
	}

	pub fn check(&self, permissions: &[Self]) -> bool {
		Self::is_allowed(self, permissions)
	}
}
