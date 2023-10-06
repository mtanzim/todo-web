use serde::{Deserialize, Serialize};
// DB
#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub id: i64,
    pub name: String,
    pub completed: i64,
}

#[derive(Debug, Deserialize)]
pub struct NewTask {
    pub name: String,
}