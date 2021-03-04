use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod handlers;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct User {
	pub id: Uuid,
	pub username: String,
	pub password: String,
	pub role: String,
	pub created_at: DateTime<Utc>,
	pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UserInfo {
	pub id: String,
	pub username: String,
	pub role: String,
}

impl UserInfo {
	pub fn new(id: String, username: String, role: String) -> Self {
		Self { id, username, role }
	}
}
