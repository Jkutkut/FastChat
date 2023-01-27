mod client;
mod fastchat;
// mod websocket;

#[tokio::main]
async fn main() {
    fastchat::launch_server(4242).await;
}