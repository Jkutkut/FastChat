use std::convert::Infallible;
// use tokio::sync::{mpsc, RwLock};
use serde::{Deserialize, Serialize};
use warp::{Filter, /*Rejection,*/ Reply};
use warp::{/*http::StatusCode,*/ reply::json};


use crate::client::{Clients, Client};
// use crate::websocket::client_connection;

pub async fn launch_server(port: u16) {
    let client_db: Clients = Clients::new();

    client_db.add_usr("test".to_string()).await;
    print_client(&client_db, "test".to_string()).await;

    // Routes
    // let ws_route = warp::path("ws")
    //     .and(warp::ws())
    //     .and(warp::path::param()) // uuid
    //     .and(with_clients(client_db.clone()))
    //     .and_then(ws_handler);
    // TODO implement other routes
    // add
    // {name: "test"}
    let add_usr = warp::path!("add")
        .and(warp::body::json())
        .and(with_clients(client_db.clone()))
        .and_then(add_handler);

    let routes = 
        add_usr
        // ws_route
        .with(warp::cors().allow_any_origin());

    println!("127.0.0.1:{port}");
    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    // warp::any() // Create a new filter that matches any request.
    //     .map( // Map the request to a new value.
    // Move: converts any variables captured by reference or mutable
    // reference to variables captured by value.
    // TODO check if this code can be called just once
    warp::any().map(move || clients.clone())
}

// pub async fn ws_handler(ws: warp::ws::Ws, uuid: String, clients: Clients) -> Result<impl Reply> {
//     // TODO not working
//     print!("ws_handler: {}", uuid);
//     let client = clients.get_usr_by_uuid(uuid).await;
//     match client {
//         Some(c) => Ok(ws.on_upgrade(move |socket| client_connection(socket, uuid, clients, c))),
//         None => Err(warp::reject::not_found()),
//     }
// }

async fn print_client(clients: &Clients, name: String) {
    let c: Option<Client> = clients.get_usr(name).await;
    match c {
        Some(c) => println!("{:?}", c),
        None => println!("No client found"),
    }
}

#[derive(Deserialize)]
struct AddRequest {
    name: String,
}

#[derive(Serialize)]
struct AddResponse {
    uuid: String,
}

async fn add_handler(body: AddRequest, clients: Clients) -> Result<impl Reply, warp::Rejection> {
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
