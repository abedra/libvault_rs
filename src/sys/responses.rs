use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct HealthResponse {
    pub cluster_id: String,
    pub cluster_name: String,
    pub initialized: bool,
    pub performance_standby: bool,
    pub replication_dr_mode: String,
    pub replication_performance_mode: String,
    pub sealed: bool,
    pub server_time_utc: u64,
    pub standby: bool,
    pub version: String
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct LeaderResponse {
    pub active_time: String,
    pub ha_enabled: bool,
    pub is_self: bool,
    pub leader_address: String,
    pub leader_cluster_address: String,
    pub performance_standby: bool,
    pub performance_standby_last_remote_wal: u64
}