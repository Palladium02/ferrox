use std::sync::Arc;

use tokio::{net::TcpListener, sync::Mutex};

use crate::{config::Config, pool::Pool, proxy::proxy, round_robin::RoundRobin};

pub struct LoadBalancer {
    config: Config,
    pool: Arc<Pool>,
    round_robin: Arc<Mutex<RoundRobin>>,
}

impl LoadBalancer {
    pub fn new(config: Config) -> Self {
        let pool = Arc::new(Pool::new(config.remote_addrs.clone()));
        let round_robin = Arc::new(Mutex::new(RoundRobin::new(Arc::clone(&pool))));

        LoadBalancer {
            config,
            pool,
            round_robin,
        }
    }

    pub async fn run(&self) {
        let listener = TcpListener::bind(&self.config.addr)
            .await
            .expect("Failed to bind to address");

        self.health_check_loop(Arc::clone(&self.pool), self.config.health_check_interval);

        loop {
            let (client, _) = listener
                .accept()
                .await
                .expect("Failed to accept connection");
            let remote_addr = self.round_robin.lock().await.get_next().await.clone();

            if let Some(remote_addr) = remote_addr {
                tokio::spawn(async move {
                    proxy(client, &remote_addr).await;
                });
            }
        }
    }

    fn health_check_loop(&self, pool: Arc<Pool>, interval: u64) {
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(interval)).await;
                pool.health_check().await;
            }
        });
    }
}
