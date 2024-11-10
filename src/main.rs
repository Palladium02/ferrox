use std::{env, fs};

mod api;
mod balancer;
mod config;
mod pool;
mod proxy;
mod round_robin;

use balancer::LoadBalancer;
use config::{Config, PartialConfig};

#[tokio::main]
async fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("Usage: {} <config_file>", args[0]);
        return;
    }

    let read_config = fs::read_to_string(&args[1]).expect("Failed to read config file");
    let user_config =
        serde_json::from_str::<PartialConfig>(&read_config).expect("Failed to parse config file");

    LoadBalancer::new(Config::default().merge(user_config))
        .expose_api()
        .await
        .run()
        .await;
}
