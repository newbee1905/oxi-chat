use serde::Serialize;
use thiserror::Error;

pub mod handlers;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum AuthError {
	#[error("invalid credentials")]
	InvalidCredentials,
	#[error("existed credentials")]
	ExistedCredentials,
	#[error("could not hash password")]
	ArgonError,
}

#[derive(Error, Debug, Serialize)]
pub enum AppError {
	#[error("wrong credentials")]
	WrongCredentialsError,
	#[error("jwt token not valid")]
	JWTTokenError,
	#[error("jwt token creation error")]
	JWTTokenCreationError,
	#[error("no auth header")]
	NoAuthHeaderError,
	#[error("invalid auth header")]
	InvalidAuthHeaderError,
	#[error("no permission")]
	NoPermissionError,
}

#[derive(Serialize, Debug)]
struct ErrorResponse {
	message: String,
	status: String,
}

impl warp::reject::Reject for AuthError {}

impl warp::reject::Reject for AppError {}

impl From<sqlx::error::Error> for AppError {
	fn from(_err: sqlx::error::Error) -> Self {
		AppError::WrongCredentialsError
	}
}

impl From<anyhow::Error> for AppError {
	fn from(_err: anyhow::Error) -> Self {
		AppError::from(_err)
	}
}

impl From<AuthError> for AppError {
	fn from(_err: AuthError) -> Self {
		AppError::from(_err)
	}
}
