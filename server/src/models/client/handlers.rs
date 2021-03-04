use crate::models::client::{Clients, Event};
use crate::models::user::UserInfo;
use crate::WebResult;

use futures::{FutureExt, StreamExt};
use log::{debug, error, info};
use serde_json::json;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
use warp::ws::{Message, WebSocket};
use warp::{http::StatusCode, Reply};

pub async fn connect_client(ws: WebSocket, clients: Clients, user_info: UserInfo) {
	let (client_ws_sender, mut client_ws_rcv) = ws.split();
	let (client_sender, client_rcv) = mpsc::unbounded_channel();

	let client_rcv = UnboundedReceiverStream::new(client_rcv);

	let id = user_info.id;
	let username = user_info.username;

	tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
		if let Err(e) = result {
			error!("error sending websocket msg: {}", e);
		}
	}));

	clients
		.write()
		.await
		.insert(Uuid::parse_str(&id).unwrap(), client_sender);

	// Make an extra clone to give to diconect handler
	let clients_clone = clients.clone();

	info!("User {} connected", id);

	while let Some(result) = client_ws_rcv.next().await {
		let msg = match result {
			Ok(msg) => msg,
			Err(e) => {
				error!("error receiving ws message for id: {}): {}", id.clone(), e);
				break;
			}
		};
		client_msg(&id, &username, msg, &clients).await;
	}

	disconnect_client(&id, &clients_clone).await;
}

async fn disconnect_client(id: &str, clients: &Clients) {
	info!("User {} disconnected", id);

	clients.write().await.remove(&Uuid::parse_str(id).unwrap());
}

async fn client_msg(my_id: &str, my_username: &str, msg: Message, clients: &Clients) {
	info!("received message from {}: {:?}", my_id, msg);
	let message = match msg.to_str() {
		Ok(v) => v,
		Err(_) => return,
	};

	if message == "ping" || message == "ping\n" {
		return;
	}

	clients.read().await.iter().for_each(|(id, sender)| {
		if my_id != &id.to_string() {
			debug!("message: {}, id: {}", message, id);
			if let Err(_) = sender.send(Ok(Message::text(
				json!({"message": message.clone(), "id": my_id, "username": my_username}).to_string(),
			))) {};
		}
	});
}

pub async fn chat_handler(_user_info: UserInfo) -> WebResult<impl Reply> {
	Ok(StatusCode::OK)
}

pub async fn ws_handler(
	ws: warp::ws::Ws,
	clients: Clients,
	user_info: UserInfo,
) -> WebResult<impl Reply> {
	info!(
		"WS: username: {}, id: {}",
		&user_info.username, &user_info.id
	);
	Ok(ws.on_upgrade(move |socket| connect_client(socket, clients, user_info.clone())))
}
