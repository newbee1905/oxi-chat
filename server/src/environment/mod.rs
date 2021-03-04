use argon::Argon;
use chrono::prelude::Utc;
use config::Config;
use log::info;
use sqlx::postgres::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::convert::Infallible;
use warp::Filter;

mod argon;
mod config;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
													abcdefghijklmnopqrstuvwxyz\
													0123456789)(*&^%$#@!~";

lazy_static! {
	static ref LEN: usize = 30;
	static ref SECRET_KEY: String = rand_key();
}

#[derive(Clone, Debug)]
pub struct Env {
	db_pool: PgPool,
	pub config: Config,
	pub argon: Argon,
}

impl Env {
	pub async fn new() -> anyhow::Result<Self> {
		let config = Config::new();
		let argon = Argon::new(SECRET_KEY.to_string());
		let db_pool = PgPoolOptions::new()
			.max_connections(5)
			.connect(&config.db_url)
			.await?;

		if config.secret_key.is_empty() {
			info!("Reseting the database...");
			info!("Current secret key: {}", SECRET_KEY.to_string());
			info!("Save it and put it into .env file as SECRET_KEY=`the key` if you want to keep this database for others time");
			sqlx::query("DROP TABLE users;").execute(&db_pool).await?;
			sqlx::query(
				"
CREATE TABLE users
(
  id uuid NOT NULL,
  username varchar(100) NOT NULL,
  password varchar(150) NOT NULL,
  created_at timestamp WITHOUT TIME ZONE DEFAULT (NOW() AT TIME ZONE 'UTC') NOT NULL,
  updated_at timestamp WITHOUT TIME ZONE NULL,
  role varchar(20) NOT NULL,
  PRIMARY KEY (id),
  UNIQUE (username)
);",
			)
			.execute(&db_pool)
			.await?;
		}

		Ok(Self {
			db_pool,
			config,
			argon,
		})
	}

	pub fn argon(&self) -> &Argon {
		&self.argon
	}

	pub fn db(&self) -> &PgPool {
		&self.db_pool
	}
}

pub fn with_env(env: Env) -> impl Filter<Extract = (Env,), Error = Infallible> + Clone {
	warp::any().map(move || env.clone())
}

fn rand_key() -> String {
	use rand::Rng;
	let mut rng = rand::thread_rng();

	let key: String = (0..*LEN)
		.map(|_| {
			let idx = rng.gen_range(0..CHARSET.len());
			CHARSET[idx] as char
		})
		.collect();
	format!("{}{}", key, Utc::now().timestamp())
}
