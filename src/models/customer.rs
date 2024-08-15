use serde::{Deserialize , Serialize};

#[derive(Clone, Debug , Default , Deserialize, Serialize)]
pub struct Customer {
    pub id: String,
    pub name: String,
}