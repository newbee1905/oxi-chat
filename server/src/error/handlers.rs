use std::convert::Infallible;
use warp::{http::StatusCode, Rejection, Reply};

use crate::error::{AppError, ErrorResponse};

pub async fn handle_app_error(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
	let (code, message) = if err.is_not_found() {
		(StatusCode::NOT_FOUND, "Not Found".to_string())
	} else if let Some(e) = err.find::<AppError>() {
		match e {
			AppError::WrongCredentialsError => (StatusCode::FORBIDDEN, e.to_string()),
			AppError::NoPermissionError => (StatusCode::UNAUTHORIZED, e.to_string()),
			AppError::JWTTokenError => (StatusCode::UNAUTHORIZED, e.to_string()),
			AppError::JWTTokenCreationError => (
				StatusCode::INTERNAL_SERVER_ERROR,
				"Internal Server Error".to_string(),
			),
			_ => (StatusCode::BAD_REQUEST, e.to_string()),
		}
	} else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
		(
			StatusCode::METHOD_NOT_ALLOWED,
			"Method Not Allowed".to_string(),
		)
	} else {
		eprintln!("Unhandled error: {:?}", err);
		(
			StatusCode::INTERNAL_SERVER_ERROR,
			"Internal Server Error".to_string(),
		)
	};

	let json = warp::reply::json(&ErrorResponse {
		status: code.to_string(),
		message,
	});

	Ok(warp::reply::with_status(json, code))
}
