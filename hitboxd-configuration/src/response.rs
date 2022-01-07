use serde::{Deserialize, Serialize};

use crate::field::Field;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Response {
    pub(crate) status_codes: Option<Vec<u16>>,
    pub(crate) headers: Option<HashMap<String, Field>>,
    #[serde(rename = "if")]
    pub(crate) body: Option<String>,
}
