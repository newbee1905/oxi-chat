use chrono::prelude::*;
use log::info;
use serde_json::json;
use sqlx::{postgres::PgPool, postgres::PgRow, query_as_unchecked, query_unchecked};
use uuid::Uuid;
use warp::{Rejection, Reply};

use crate::error::AuthError;
use crate::models::user::{User, UserInfo};
use crate::WebResult;

pub async fn create_user(
	connection: &PgPool,
	id: Uuid,
	username: &str,
	password: &str,
	role: &str,
) -> anyhow::Result<Vec<PgRow>> {
	query_unchecked!(
		r#"INSERT INTO users (id, username, password, role, created_at) VALUES ($1, $2, $3, $4, $5)"#,
		id,
		username,
		password,
		role,
		Utc::now(),
	)
	.fetch_all(connection)
	.await
	.map_err(|err| err.into())
}

pub async fn get_user(connection: &PgPool, username: &str) -> Result<Option<User>, Rejection> {
	let user = query_as_unchecked!(
		User,
		r#"SELECT id, username, password, role, created_at, updated_at FROM users WHERE username = $1"#,
		username,
	)
	.fetch_one(connection)
	.await
	.map_err(|_| AuthError::InvalidCredentials)
	.ok();
	Ok(user)
}

pub async fn user_handler(user_info: UserInfo) -> WebResult<impl Reply> {
	info!("id: {}, username: {}", user_info.id, user_info.username);
	Ok(warp::reply::json(
		&json!({"id": user_info.id, "username": user_info.username}),
	))
}
