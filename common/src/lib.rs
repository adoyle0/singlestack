use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Todo {
    pub id: Option<String>,
    pub title: String,
}
