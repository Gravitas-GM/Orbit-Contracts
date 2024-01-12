use serde::{Deserialize, Serialize};
use std::borrow::Borrow;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum Permission {
}

impl Permission {
	pub fn is_allowed<P: Borrow<Self>>(permission: P, permissions: &[Self]) -> bool {
		permissions.contains(&Permission::Admin) || permissions.contains(permission.borrow())
	}

	pub fn check(&self, permissions: &[Self]) -> bool {
		Self::is_allowed(self, permissions)
	}
}
