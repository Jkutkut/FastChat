use futures::{FutureExt, StreamExt};
// use serde::Deserialize;
// use serde_json::from_str;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{/*Message,*/ WebSocket};

use crate::client::{Clients, Client};

pub async fn client_connection(ws: WebSocket, uuid: String, clients: Clients, mut client: Client) {
    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel();

    let client_rcv = UnboundedReceiverStream::new(client_rcv);
    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            eprintln!("error sending websocket msg: {}", e);
        }
    }));

    client.sender = Some(client_sender);
    // clients.write().await.insert(uuid.clone(), client);
    

    println!("{} connected", client.name);

    while let Some(result) = client_ws_rcv.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("error receiving ws message for id: {}): {}", uuid.clone(), e);
                break;
            }
        };
        // client_msg(&id, msg, &clients).await;
        println!("received message from {}: {:?}", client.name, msg);
    }

    println!("{} disconnected", client.name);
    clients.remove_usr(&client.name);
}