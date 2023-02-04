use std::collections::HashMap;
use std::sync::Arc; // Arc to share ownership
use tokio::sync::{mpsc, RwLock};
use warp::{ws::Message};

use uuid::Uuid;


#[derive(Debug, Clone)] // To be able to print it and to clone it
pub struct Client {
    pub uuid: String,
    pub name: String,
    // pub channels: Vec<String>,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

impl Client {
    pub fn send_msg(&self, msg: String) {
        if let Some(sender) = &self.sender {
            let _ = sender.send(Ok(Message::text(msg)));
        }
    }
}

// type ClientsHM = Arc<RwLock<HashMap<String, Client>>>; // TODO implement with DB.

#[derive(Debug, Clone)]
pub struct Clients {
    clients: Arc<RwLock<HashMap<String, Client>>>,
}

impl Clients {
    pub fn new() -> Self {
        Clients {
            clients: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn get_usr(&self, id: String) -> Option<Client> {
        self.clients.read().await.get(&id).cloned()
    }

    pub async fn get_usr_by_uuid(&self, uuid: String) -> Option<Client> {
        let clients = self.clients.read().await;
        for (_, client) in clients.iter() {
            if client.uuid == uuid {
                return Some(client.clone());
            }
        }
        None
    }

    pub async fn size(&self) -> usize {
        self.clients.read().await.len()
    }

    pub async fn add_usr(&self, name: String) {
        let uuid = Uuid::new_v4().as_simple().to_string();
        let c: Client = Client {
            uuid: uuid,
            name: name.clone(),
            sender: None::<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
        };
        self.clients.write().await.insert(name, c);
    }

    pub async fn update_usr(&self, name: String, client: Client) -> Option<Client> {
        self.clients.write().await.insert(name.clone(), client);
        self.get_usr(name).await
    }

    pub async fn remove_usr(&self, name: String) {
        self.clients.write().await.remove(&name);
    }

    pub async fn broadcast_msg(&self, msg: String, client: &Client) {
        println!("Broadcasting message to all clients...");
        println!("  - msg: {}", &msg);
        println!("  - sender: {}", &client.name);

        self.clients.read().await
            .iter()
            // .filter(|(_, c)| match c.sender {
            //     Some(_) => c.uuid != client.uuid,
            //     None => false,
            // })
            .for_each(|(_, c)| {
                c.send_msg(msg.clone());
            });
    }
}
