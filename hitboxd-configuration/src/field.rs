use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Field {
    All,
    Variants(Vec<String>),
    Exclude(Vec<String>),
}
