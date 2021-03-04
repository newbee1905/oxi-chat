#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::Filter;

use crate::environment::Env;
use crate::models::client;
use crate::models::user;

mod auth;
mod environment;
mod error;
mod models;

type Result<T> = std::result::Result<T, error::AppError>;
type WebResult<T> = std::result::Result<T, warp::reject::Rejection>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	pretty_env_logger::init();
	dotenv::from_filename("../.env").ok();

	let clients: client::Clients = Arc::new(RwLock::new(HashMap::new()));

	let env = Env::new().await?;

	if env.config.secret_key.is_empty() {
		let hash = env
			.argon()
			.hasher()
			.with_password(&env.config.admin_password)
			.hash()
			.unwrap();
		user::handlers::create_user(
			env.db(),
			uuid::Uuid::new_v4(),
			&env.config.admin_username,
			&hash,
			"Admin",
		)
		.await?;
	}

	// Check jwt token return user or admin
	let login_get = warp::path!("api" / "auth" / "login")
		.and(auth::middleware::with_auth())
		.and_then(user::handlers::user_handler);
	// Send json to login
	let login_post = warp::path!("api" / "auth" / "login")
		.and(warp::post())
		.and(warp::body::json())
		.and(environment::with_env(env.clone()))
		.and_then(auth::handlers::login_handler);
	let login_routes = login_get.or(login_post);

	let logout_route =
		warp::path!("api" / "auth" / "logout").and_then(auth::handlers::logout_handler);

	let register_route = warp::path!("api" / "auth" / "register")
		.and(warp::post())
		.and(warp::body::json())
		.and(environment::with_env(env.clone()))
		.and_then(auth::handlers::register_handler);

	let auth_routes = login_routes.or(logout_route).or(register_route);

	let client = warp::fs::dir("../client/public");
	let login_client_route = warp::path("login").and(warp::fs::file("../client/public/index.html"));
	let register_client_route =
		warp::path("register").and(warp::fs::file("../client/public/index.html"));
	let chat_client_route = warp::path("chat").and(warp::fs::file("../client/public/index.html"));

	let client_routes = client
		.or(login_client_route)
		.or(register_client_route)
		.or(chat_client_route);

	let ws_route = warp::path("ws")
		.and(warp::ws())
		.and(client::with_clients(clients.clone()))
		.and(auth::middleware::with_auth())
		.and_then(client::handlers::ws_handler);

	let chat_route = warp::path!("chat")
		.and(auth::middleware::with_auth())
		.and_then(client::handlers::chat_handler);

	let routes = auth_routes
		.or(client_routes)
		.or(ws_route)
		.or(chat_route)
		.recover(error::handlers::handle_app_error);

	warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
	Ok(())
}
