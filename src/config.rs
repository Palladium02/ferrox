use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct PartialConfig {
    addr: Option<String>,
    remote_addrs: Option<Vec<String>>,
    health_check_interval: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub addr: String,
    pub remote_addrs: Vec<String>,
    pub health_check_interval: u64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            addr: "127.0.0.1:8080".into(),
            remote_addrs: Vec::new(),
            health_check_interval: 5,
        }
    }
}

impl Config {
    pub fn merge(self, partial: PartialConfig) -> Self {
        Config {
            addr: partial.addr.unwrap_or(self.addr),
            remote_addrs: partial.remote_addrs.unwrap_or(self.remote_addrs),
            health_check_interval: partial
                .health_check_interval
                .unwrap_or(self.health_check_interval),
        }
    }
}
