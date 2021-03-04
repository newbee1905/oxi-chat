use serde_json::json;
use warp::http::{
	header::{HeaderMap, HeaderValue},
	StatusCode,
};
use warp::Reply;

use crate::auth::{create_jwt, Role};
use crate::environment::Env;
use crate::error::AuthError;
use crate::models::login::LoginRequest;
use crate::models::user::handlers::{create_user, get_user};
use crate::WebResult;

pub async fn register_handler(req: LoginRequest, env: Env) -> WebResult<impl Reply> {
	let hash = env
		.argon()
		.hasher()
		.with_password(&req.password)
		.hash()
		.unwrap();
	let role = Role::User.to_string();
	match create_user(env.db(), uuid::Uuid::new_v4(), &req.username, &hash, &role).await {
		Ok(_) => Ok(warp::reply::json(
			&json!({"status": "success", "username": &req.username}),
		)),
		Err(_) => Err(warp::reject::custom(AuthError::ExistedCredentials)),
	}
}

pub async fn login_handler(req: LoginRequest, env: Env) -> WebResult<impl Reply> {
	let result = get_user(env.db(), &req.username).await?;
	let user = match result {
		Some(_) => result.unwrap(),
		None => return Err(warp::reject::custom(AuthError::ArgonError)),
	};

	let is_valid = env
		.argon()
		.verifier()
		.with_hash(&user.password)
		.with_password(&req.password)
		.verify()
		.or(Err(warp::reject::custom(AuthError::ArgonError)))?;

	if !is_valid {
		return Err(warp::reject::custom(AuthError::InvalidCredentials));
	}

	let token = create_jwt(
		&user.id.to_string(),
		&user.username,
		&Role::from_str(&user.role),
	)
	.unwrap();

	let reply = warp::reply::json(
		&json!({ "id": &user.id, "username": &user.username, "role": &Role::from_str(&user.role)}),
	);
	let reply = warp::reply::with_status(reply, StatusCode::OK);

	let mut cookies = HeaderMap::new();

	cookies.append(
		"set-cookie",
		HeaderValue::from_str(&format!(
			"jwt={}; max-age={}; path=/; SameSite=Lax; HttpOnly",
			token,
			48 * 60 * 60,
		))
		.unwrap(),
	);

	cookies.append(
		"set-cookie",
		HeaderValue::from_str(&format!(
			"role={}; max-age={}; path=/; SameSite=Lax; HttpOnly",
			user.role,
			48 * 60 * 60,
		))
		.unwrap(),
	);

	cookies.append("credentials", HeaderValue::from_str("include").unwrap());

	let mut response = reply.into_response();
	let headers = response.headers_mut();
	headers.extend(cookies);

	Ok(response)
}

pub async fn logout_handler() -> WebResult<impl Reply> {
	let reply = warp::reply::json(&json!({}));
	let reply = warp::reply::with_status(reply, StatusCode::OK);

	let mut cookies = HeaderMap::new();

	cookies.append(
		"set-cookie",
		HeaderValue::from_str(&format!(
			"jwt={}; expires={}; path=/; SameSite=Lax; HttpOnly",
			" ", "Thu, Jan 01 1970 00:00:00 UTC",
		))
		.unwrap(),
	);

	cookies.append(
		"set-cookie",
		HeaderValue::from_str(&format!(
			"role={}; expires={}; path=/; SameSite=Lax; HttpOnly",
			" ", "Thu, Jan 01 1970 00:00:00 UTC",
		))
		.unwrap(),
	);

	cookies.append("credentials", HeaderValue::from_str("include").unwrap());

	let mut response = reply.into_response();
	let headers = response.headers_mut();
	headers.extend(cookies);

	Ok(response)
}
