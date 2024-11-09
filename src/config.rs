use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub addr: String,
    pub remote_addrs: Vec<String>,
    pub health_check_interval: u64,
}
