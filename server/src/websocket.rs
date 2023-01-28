use futures::{FutureExt, StreamExt};
use serde::{/*Deserialize,*/ Serialize};
use serde_json::{json/*, from_str*/};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};

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
    client = clients.update_usr(client.name.clone(), client).await.unwrap();

    println!("{} connected", &client.name);

    while let Some(result) = client_ws_rcv.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("error receiving ws message for id: {}): {}", uuid.clone(), e);
                break;
            }
        };
        usr_msg(msg, &client, &clients).await;
    }

    println!("{} disconnected", &client.name);
    clients.remove_usr(client.name.clone()).await;
}


#[derive(Serialize)]
struct Msg {
    user: String,
    msg: String,
}

async fn usr_msg(message: Message, client: &Client, clients: &Clients) {
    let msg = match message.to_str() {
        Ok(v) => v,
        Err(_) => return,
    };
    println!("- Message from \"{}\": {:?}\n\tuuid: {}", client.name, msg, client.uuid);

    // Respond to the client
    // client.send_msg(
    //     json!(&Msg {
    //         user: client.name.clone(),
    //         msg: msg.to_string(),
    //     }).to_string()
    // ).await;

    // For all clients, send the message
    clients.broadcast_msg(
        json!(&Msg {
            user: client.name.clone(),
            msg: msg.to_string(),
        }).to_string(),
        client
    ).await;
}