use std::sync::Arc;
use tokio::{net::TcpStream, sync::Mutex};

pub struct Pool {
    alive_addrs: Arc<Mutex<Vec<String>>>,
    all_addrs: Vec<String>,
}

impl Pool {
    pub fn new(addrs: Vec<String>) -> Self {
        Pool {
            alive_addrs: Arc::new(Mutex::new(addrs.clone())),
            all_addrs: addrs,
        }
    }

    pub async fn at(&self, index: usize) -> Option<String> {
        self.alive_addrs.lock().await.get(index).cloned()
    }

    pub async fn len(&self) -> usize {
        self.alive_addrs.lock().await.len()
    }

    pub async fn health_check(&self) {
        let mut addrs = self.alive_addrs.lock().await;
        let mut to_retain = self.all_addrs.clone();
        for addr in &self.all_addrs {
            if !self.is_alive(addr).await {
                to_retain.retain(|x| x != addr);
            }
        }

        *addrs = to_retain;
    }

    async fn is_alive(&self, addr: &str) -> bool {
        match TcpStream::connect(addr).await {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}