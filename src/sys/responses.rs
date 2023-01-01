use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct HealthResponse {
    cluster_id: String,
    cluster_name: String,
    initialized: bool,
    performance_standby: bool,
    replication_dr_mode: String,
    replication_performance_mode: String,
    sealed: bool,
    standby: bool,
    version: String
}
