use serde_json::json;

use super::responses::{HealthResponse, LeaderResponse};
#[allow(dead_code)]
pub fn health_success_json() -> serde_json::Value {
    json!({
        "cluster_id": "36f318fa-b31d-dcea-e660-162ae39118ab",
        "cluster_name": "vault-cluster-980caf19",
        "initialized": true,
        "performance_standby": false,
        "replication_dr_mode": "disabled",
        "replication_performance_mode": "disabled",
        "sealed": false,
        "server_time_utc": 1672718284,
        "standby": false,
        "version": "1.12.2"
    })
}

#[allow(dead_code)]
pub fn health_success() -> HealthResponse {
    HealthResponse {
        cluster_id: "36f318fa-b31d-dcea-e660-162ae39118ab".into(),
        cluster_name: "vault-cluster-980caf19".into(),
        initialized: true,
        performance_standby: false,
        replication_dr_mode: "disabled".into(),
        replication_performance_mode: "disabled".into(),
        sealed: false,
        server_time_utc: 1672718284,
        standby: false,
        version: "1.12.2".into(),
    }
}

#[allow(dead_code)]
pub fn leader_success_json() -> serde_json::Value {
    json!({
        "active_time": "0001-01-01T00:00:00Z",
        "ha_enabled": false,
        "is_self": false,
        "leader_address": "",
        "leader_cluster_address": "",
        "performance_standby": false,
        "performance_standby_last_remote_wal": 0
    })
}

#[allow(dead_code)]
pub fn leader_success() -> LeaderResponse {
    LeaderResponse {
        active_time: "0001-01-01T00:00:00Z".into(),
        ha_enabled: false,
        is_self: false,
        leader_address: "".into(),
        leader_cluster_address: "".into(),
        performance_standby: false,
        performance_standby_last_remote_wal: 0,
    }
}