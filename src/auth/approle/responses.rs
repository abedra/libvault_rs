use std::collections::HashMap;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize, PartialEq)]
pub struct MfaRequirement {
    pub mfa_request_id: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, PartialEq)]
pub struct Auth {
    pub accessor: String,
    pub client_token: String,
    pub entity_id: String,
    pub lease_duration: i64,
    pub metadata: HashMap<String, String>,
    pub mfa_requirement: Option<MfaRequirement>,
    pub num_uses: i64,
    pub orphan: bool,
    pub policies: Vec<String>,
    pub renewable: bool,
    pub token_policies: Vec<String>,
    pub token_type: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, PartialEq)]
pub struct ApproleLoginResponse {
    pub auth: Auth,
    pub data: Option<HashMap<String, String>>,
    pub lease_duration: i64,
    pub lease_id: String,
    pub renewable: bool,
    pub request_id: String,
    pub warnings: Option<String>,
    pub wrap_info: Option<HashMap<String, String>>
}