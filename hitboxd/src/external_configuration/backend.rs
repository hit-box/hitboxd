use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InMemory {
    pub name: String,
    pub capacity: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum Backend {
    InMemory(InMemory),
}

impl Default for InMemory {
    fn default() -> Self {
        Self {
            name: String::from("StrettoBackend"),
            capacity: 10_000_000,
        }
    }
}

impl Default for Backend {
    fn default() -> Self {
        Self::InMemory(InMemory::default())
    }
}
