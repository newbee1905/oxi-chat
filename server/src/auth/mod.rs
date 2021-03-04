use serde::{Deserialize, Serialize};
use std::fmt;

use crate::error::AppError;
use crate::Result;

const JWT_SECRET: &[u8] = b"secret";

pub mod handlers;
pub mod middleware;

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
	id: String,
	username: String,
	role: String,
	exp: usize,
}

#[derive(Clone, PartialEq, Serialize)]
pub enum Role {
	User,
	Admin,
}

impl Role {
	pub fn from_str(role: &str) -> Role {
		match role {
			"Admin" => Role::Admin,
			_ => Role::User,
		}
	}
}

impl fmt::Display for Role {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Role::User => write!(f, "User"),
			Role::Admin => write!(f, "Admin"),
		}
	}
}

pub fn create_jwt(id: &str, username: &str, role: &Role) -> Result<String> {
	let expiration = chrono::Utc::now()
		.checked_add_signed(chrono::Duration::hours(48))
		.expect("valid timestamp")
		.timestamp();

	let claims = Claims {
		id: id.to_owned(),
		username: username.to_owned(),
		role: role.to_string(),
		exp: expiration as usize,
	};
	let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS512);
	jsonwebtoken::encode(
		&header,
		&claims,
		&jsonwebtoken::EncodingKey::from_secret(JWT_SECRET),
	)
	.map_err(|_| AppError::JWTTokenError)
}
