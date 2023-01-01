use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct KeyValueResponseData {
    pub data: HashMap<String, String>
}

#[derive(Debug, Deserialize)]
pub struct KeyValueV2Response {
    pub data: KeyValueResponseData
}

#[derive(Debug, Deserialize)]
pub struct KeyValueV1Response {
    pub data: HashMap<String, String>
}

#[derive(Debug)]
pub struct KeyValueResponse {
    pub data: HashMap<String, String>
}

impl KeyValueResponse {
    pub fn new(data: HashMap<String, String>) -> Self {
        Self { data }
    }
}