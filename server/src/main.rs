mod client;
use client::{Client, Clients};

#[tokio::main]
async fn main() {
    let client_db: Clients = Clients::new();
    client_db.add_usr("John".to_string()).await;
    client_db.add_usr("Jane".to_string()).await;

    print_client(&client_db, "John".to_string()).await;
    print_client(&client_db, "Jane".to_string()).await;

    client_db.remove_usr("John".to_string()).await;
    print_client(&client_db, "John".to_string()).await;
}


async fn print_client(clients: &Clients, name: String) {
    let c: Option<Client> = clients.get_client(name).await;
    match c {
        Some(c) => println!("{:?}", c),
        None => println!("No client found"),
    }
}