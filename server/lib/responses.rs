use serde::{Deserialize, Serialize};

use crate::ClientID;

#[derive(Serialize, Deserialize)]
pub struct Connect {
    pub client_id: ClientID,
}

#[derive(Serialize, Deserialize)]
pub struct ListClients {
    pub clients: Vec<ClientID>,
}
