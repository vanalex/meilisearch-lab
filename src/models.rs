use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Movie {
    pub id: usize,
    pub title: String,
    pub genres: Vec<String>,
}