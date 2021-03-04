use log::debug;
use warp::Filter;

use crate::auth::{Claims, Role, JWT_SECRET};
use crate::error::AppError;
use crate::models::user::UserInfo;
use crate::WebResult;

pub fn with_auth() -> impl Filter<Extract = (UserInfo,), Error = warp::reject::Rejection> + Clone {
	// warp::header::headers_cloned()
	// 	.map(move |headers: HeaderMap<HeaderValue>| (role.clone(), headers))
	// 	.and_then(authorize)
	warp::filters::cookie::cookie("role")
		.and(warp::filters::cookie::cookie("jwt"))
		.map(move |role: String, jwt: String| (Role::from_str(&role), jwt))
		.and_then(authorize)
}

async fn authorize((role, jwt): (Role, String)) -> WebResult<UserInfo> {
	let decoded = jsonwebtoken::decode::<Claims>(
		&jwt,
		&jsonwebtoken::DecodingKey::from_secret(JWT_SECRET),
		&jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS512),
	)
	.map_err(|_| warp::reject::custom(AppError::JWTTokenError))?;

	if role == Role::Admin && Role::from_str(&decoded.claims.role) != Role::Admin {
		return Err(warp::reject::custom(AppError::NoPermissionError));
	}

	debug!(
		"id: {}, username: {}, role: {}",
		decoded.claims.id, decoded.claims.username, decoded.claims.role
	);

	Ok(UserInfo {
		id: decoded.claims.id,
		username: decoded.claims.username,
		role: decoded.claims.role,
	})
}
