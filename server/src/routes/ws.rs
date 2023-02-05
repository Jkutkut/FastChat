use warp::Reply;

use crate::client::{Clients};
use crate::websocket::client_connection;

pub async fn ws_handler(ws: warp::ws::Ws, uuid: String, clients: Clients) -> Result<impl Reply, warp::Rejection> {
    let client = clients.get_usr_by_uuid(uuid.clone()).await;
    match client {
        Some(c) => Ok(ws.on_upgrade(move |socket| client_connection(socket, uuid, clients, c))),
        None => Err(warp::reject::not_found()),
    }
}
