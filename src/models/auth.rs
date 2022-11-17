use serde::{Deserialize,Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {
    pub password: String,
    pub email: String,
}