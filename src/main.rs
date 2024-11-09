use std::{env, fs};

mod balancer;
mod config;
mod pool;
mod proxy;
mod round_robin;

use balancer::LoadBalancer;

#[tokio::main]
async fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("Usage: {} <config_file>", args[0]);
        return;
    }

    let read_config = fs::read_to_string(&args[1]).expect("Failed to read config file");
    let config = serde_json::from_str(&read_config).expect("Failed to parse config file");

    LoadBalancer::new(config).run().await;
}
