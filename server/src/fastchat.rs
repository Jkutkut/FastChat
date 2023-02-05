use std::convert::Infallible;
// use tokio::sync::{mpsc, RwLock};
use warp::Filter;

use crate::client::{Clients};
use crate::routes::{add_handler, print_client, ws_handler};

pub async fn launch_server(port: u16) {
    let client_db: Clients = Clients::new();

    client_db.add_usr("test".to_string()).await;
    print_client(&client_db, "test".to_string()).await;

    // ******* Routes *******
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(warp::path::param()) // uuid
        .and(with_clients(client_db.clone()))
        .and_then(ws_handler);

    // add, remove
    // {name: "test"}
    let add_usr = warp::path!("add_usr")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_clients(client_db.clone()))
        .and_then(add_handler);

    // TODO implement other routes


    let routes = ws_route
        .or(add_usr)
        .with(warp::cors().allow_any_origin());

    println!("Running on port {port}");
    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    // warp::any() // Create a new filter that matches any request.
    //     .map( // Map the request to a new value.
    // Move: converts any variables captured by reference or mutable
    // reference to variables captured by value.
    // TODO check if this code can be called just once
    warp::any().map(move || clients.clone())
}
