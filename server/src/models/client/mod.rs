use serde::Deserialize;
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use uuid::Uuid;
use warp::{ws::Message, Filter};

pub mod handlers;

/// Contain uuid and sender
pub type Clients = Arc<RwLock<HashMap<Uuid, mpsc::UnboundedSender<Result<Message, warp::Error>>>>>;

// TODO: Add create_add and secret mode
// secret_mode: -> Delete after a certain amount of time
// create_at: -> to check time
#[derive(Deserialize, Debug)]
pub struct Event {
	pub message: String,
}

pub fn with_clients(
	clients: Clients,
) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
	warp::any().map(move || clients.clone())
}
