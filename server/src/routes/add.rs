use serde::{Deserialize, Serialize};
use warp::{Reply, reply::json};

use crate::client::{Clients, Client};

#[derive(Deserialize)]
pub struct AddRequest {
    name: String,
    // password: String // TODO
}

#[derive(Serialize)]
pub struct AddResponse {
    uuid: String,
}

pub async fn print_client(clients: &Clients, name: String) { // TODO remove
    let c: Option<Client> = clients.get_usr(name).await;
    match c {
        Some(c) => {
            println!("{:?}", c);
            println!("Total clients: {}", clients.size().await)
        },
        None => {
            println!("No client found");
            println!("Total clients: {}", clients.size().await)
        }
    }
}

pub async fn add_handler(body: AddRequest, clients: Clients) -> Result<impl Reply, warp::Rejection> {
    let name: String = body.name;

    clients.add_usr(name.clone()).await; // TODO: check if user already exists
    print_client(&clients, name.clone()).await;
    match clients.get_usr(name.clone()).await {
        Some(c) => Ok(json(&AddResponse {
            uuid: format!("ws://127.0.0.1:4242/ws/{}", c.uuid),
        })),
        None => Err(warp::reject::not_found()),
    }
}
