use std::collections::HashMap;

use serde_json::json;
use super::responses::KeyValueResponse;

#[allow(dead_code)]
pub fn successful_v1_json_response() -> serde_json::Value {
    json!({
        "auth": null,
        "data": {
        "foo": "bar"
        },
        "lease_duration": 2764800,
        "lease_id": "",
        "renewable": false,
        "request_id": "782a5b1b-8ec3-3e47-b2e5-e0a43fb6ad5f",
        "warnings": null,
        "wrap_info": null
    })
}

#[allow(dead_code)]
pub fn successful_v2_json_response() -> serde_json::Value {
    json!({
        "auth": null,
        "data": {
          "data": {
            "foo": "bar"
          },
          "metadata": {
            "created_time": "2023-01-01T20:36:17.398127031Z",
            "custom_metadata": null,
            "deletion_time": "",
            "destroyed": false,
            "version": 1
          }
        },
        "lease_duration": 0,
        "lease_id": "",
        "renewable": false,
        "request_id": "630bd562-0379-18c5-d3a1-dddccf8bf3ec",
        "warnings": null,
        "wrap_info": null
    })
}

#[allow(dead_code)]
pub fn successful_key_value_response() -> KeyValueResponse {
    KeyValueResponse {
        data: HashMap::from([("foo".into(), "bar".into())]),
    }
}
