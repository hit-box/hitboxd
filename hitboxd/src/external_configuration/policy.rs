use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Enabled {
    ttl: u16,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Policy {
    Enabled(Enabled),
    Disabled,
}
