pub fn health_request(base_url: String) -> String {
    let uri = "sys/health";
    format!("{}/{}", base_url, uri)
}