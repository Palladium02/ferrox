use std::sync::Arc;

use tokio::{net::TcpListener, sync::Mutex};

use crate::{config::Config, pool::Pool, proxy::proxy, round_robin::RoundRobin};

pub struct LoadBalancer {
    config: Config,
}

impl LoadBalancer {
    pub fn new(config: Config) -> Self {
        LoadBalancer { config }
    }

    pub async fn run(&self) {
        let listener = TcpListener::bind(&self.config.addr)
            .await
            .expect("Failed to bind to address");
        let pool = Arc::new(Pool::new(self.config.remote_addrs.clone()));
        let round_robin = Arc::new(Mutex::new(RoundRobin::new(Arc::clone(&pool))));

        let pool_for_health_check = Arc::clone(&pool);
        let interval = self.config.health_check_interval;
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(interval)).await;

                pool_for_health_check.health_check().await;
            }
        });

        loop {
            let (client, _) = listener
                .accept()
                .await
                .expect("Failed to accept connection");
            let remote_addr = round_robin.lock().await.get_next().await.clone();

            if let Some(remote_addr) = remote_addr {
                tokio::spawn(async move {
                    proxy(client, &remote_addr).await;
                });
            }
        }
    }
}
