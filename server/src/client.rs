use std::collections::HashMap;
use std::sync::Arc; // Arc to share ownership
use tokio::sync::RwLock;

use uuid::Uuid;


#[derive(Debug, Clone)] // To be able to print it and to clone it
pub struct Client {
    pub uuid: String,
    pub name: String,
    // pub topics: Vec<String>,
    // pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

type ClientsHM = Arc<RwLock<HashMap<String, Client>>>;

pub struct Clients {
    clients: ClientsHM,
}

impl Clients {
    pub fn new() -> Self {
        Clients {
            clients: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn get_client(&self, id: String) -> Option<Client> {
        self.clients.read().await.get(&id).cloned()
    }

    pub async fn add_usr(&self, name: String) {
        let uuid = Uuid::new_v4().as_simple().to_string();
        let c: Client = Client {
            uuid: uuid,
            name: name.clone(),
        };
        self.clients.write().await.insert(name, c);
    }

    pub async fn remove_usr(&self, name: String) {
        self.clients.write().await.remove(&name);
    }
}