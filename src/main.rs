use std::{
    env, fs,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
};

use serde::Deserialize;
use tokio::{
    io::copy_bidirectional,
    net::{TcpListener, TcpStream},
};

#[derive(Debug, Clone, Deserialize)]
struct Config {
    pub addr: String,
    pub remote_addrs: Vec<String>,
    pub health_check_interval: u64,
}

struct RoundRobin {
    current: AtomicUsize,
    alive_addrs: Vec<String>,
    all_addrs: Vec<String>,
}

impl RoundRobin {
    pub fn new(addrs: Vec<String>) -> Self {
        RoundRobin {
            current: AtomicUsize::new(0),
            alive_addrs: addrs.clone(),
            all_addrs: addrs,
        }
    }

    pub fn get_next(&self) -> &String {
        let idx = self.current.fetch_add(1, Ordering::SeqCst) % self.alive_addrs.len();
        &self.alive_addrs[idx]
    }
}

async fn run(config: Arc<Config>) {
    let listener = TcpListener::bind(&config.addr)
        .await
        .expect("Failed to bind to address");
    let round_robin = Arc::new(Mutex::new(RoundRobin::new(config.remote_addrs.clone())));

    let round_robin_for_health_check = Arc::clone(&round_robin);
    tokio::spawn(async move {
        health_check(round_robin_for_health_check, config.health_check_interval).await;
    });

    loop {
        let (client, _) = listener
            .accept()
            .await
            .expect("Failed to accept connection");
        let remote_addr = round_robin
            .lock()
            .expect("Poisoned lock")
            .get_next()
            .clone();

        tokio::spawn(async move {
            proxy(client, &remote_addr).await;
        });
    }
}

async fn proxy(mut client: TcpStream, remote_addr: &str) {
    match TcpStream::connect(remote_addr).await {
        Ok(mut server) => {
            if copy_bidirectional(&mut client, &mut server).await.is_err() {
                eprintln!("Failed to copy data between client and server");
            }
        }
        Err(e) => {
            eprintln!("Failed to connect to remote server: {}", e);
        }
    }
}

async fn health_check(round_robin: Arc<Mutex<RoundRobin>>, interval: u64) {
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(interval)).await;
        // Move all_addrs access into own scope to release lock as soon as possible
        let all_addrs = { round_robin.lock().expect("Poisoned lock").all_addrs.clone() };

        let mut alive_addrs = Vec::new();
        for addr in all_addrs {
            match TcpStream::connect(&addr).await {
                Ok(_) => {
                    alive_addrs.push(addr);
                }
                Err(e) => {
                    eprintln!("Failed to connect to remote server: {}", e);
                }
            }
        }

        round_robin.lock().expect("Poisoned lock").alive_addrs = alive_addrs;
    }
}

#[tokio::main]
async fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("Usage: {} <config_file>", args[0]);
        return;
    }

    let read_config = fs::read_to_string(&args[1]).expect("Failed to read config file");
    let config = serde_json::from_str(&read_config).expect("Failed to parse config file");

    run(Arc::new(config)).await;
}
