use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UploadMetadata {
    pub timestamp: u128,
    pub files: HashMap<String, String>,
    pub root_hash: String,
}
