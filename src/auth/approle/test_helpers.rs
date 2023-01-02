use std::collections::HashMap;
use serde_json::json;
use super::responses::{ApproleLoginResponse, Auth};

#[allow(dead_code)]
pub fn successful_login_json_response() -> serde_json::Value {
    json!({
      "auth": {
        "accessor": "LsKyvlMAvGcQhktNAjZ9it8q",
        "client_token": "hvs.CAESINqsFsEB_CyrfrTzkOLf_eZbOpcvAlAr5Kh8ulWb6_HmGh4KHGh2cy5DclY5bm9aakJrMDBheTExYU1TSVQxanQ",
        "entity_id": "4dc20fd9-71e4-b61e-5907-f5926ccbf964",
        "lease_duration": 2764800,
        "metadata": {
          "role_name": "client"
        },
        "mfa_requirement": null,
        "num_uses": 0,
        "orphan": true,
        "policies": [
          "default",
          "example"
        ],
        "renewable": true,
        "token_policies": [
          "default",
          "example"
        ],
        "token_type": "service"
      },
      "data": null,
      "lease_duration": 0,
      "lease_id": "",
      "renewable": false,
      "request_id": "8ff6e17e-61b0-7a65-dcbc-6870b1fd8d1e",
      "warnings": null,
      "wrap_info": null
    })
}

#[allow(dead_code)]
pub fn successful_login_response() -> ApproleLoginResponse {
  ApproleLoginResponse { 
    auth: Auth { 
        accessor: "LsKyvlMAvGcQhktNAjZ9it8q".into(),
        client_token: "hvs.CAESINqsFsEB_CyrfrTzkOLf_eZbOpcvAlAr5Kh8ulWb6_HmGh4KHGh2cy5DclY5bm9aakJrMDBheTExYU1TSVQxanQ".into(),
        entity_id: "4dc20fd9-71e4-b61e-5907-f5926ccbf964".into(),
        lease_duration: 2764800,
        metadata: HashMap::from([("role_name".into(), "client".into())]),
        mfa_requirement: None,
        num_uses: 0,
        orphan: true,
        policies: vec!["default".into(), "example".into()],
        renewable: true,
        token_policies: vec!["default".into(), "example".into()],
        token_type: "service".into() 
    },
    data: None,
    lease_duration: 0,
    lease_id: "".into(),
    renewable: false,
    request_id: "8ff6e17e-61b0-7a65-dcbc-6870b1fd8d1e".into(),
    warnings: None,
    wrap_info: None
  }
}