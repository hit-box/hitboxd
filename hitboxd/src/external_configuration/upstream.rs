use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Scheme {
    Http,
    Https,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    pub host: String,
    pub port: u16,
    pub scheme: Scheme,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Upstream {
    pub name: String,
    pub addresses: Vec<Address>,
}

impl Default for Scheme {
    fn default() -> Self {
        Scheme::Http
    }
}

impl Default for Address {
    fn default() -> Self {
        Self {
            host: String::from("127.0.0.1"),
            port: 8080,
            scheme: Default::default(),
        }
    }
}

impl Default for Upstream {
    fn default() -> Self {
        Upstream {
            name: String::from("default-upstream"),
            addresses: vec![Address::default()],
        }
    }
}
