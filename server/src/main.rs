mod client;
mod fastchat;
mod websocket;
mod routes;

#[tokio::main]
async fn main() {
    fastchat::launch_server(4242).await;
}
