use std::env;

#[derive(Clone, Debug)]
pub struct Config {
	pub db_url: String,
	pub secret_key: String,
	pub admin_username: String,
	pub admin_password: String,
}

impl Config {
	pub fn new() -> Self {
		let db_url = env::var("DATABASE_URL").unwrap();
		let secret_key = env::var("SECRET_KEY").unwrap_or(String::new());
		let admin_username = env::var("ADMIN_USERNAME").unwrap_or(String::from("admin"));
		let admin_password = env::var("ADMIN_PASSWORD").unwrap_or(String::from("1234"));
		Self {
			db_url,
			secret_key,
			admin_username,
			admin_password,
		}
	}
}
