use std::sync::{atomic::AtomicUsize, Arc};

use crate::pool::Pool;

pub struct RoundRobin {
    pool: Arc<Pool>,
    current: AtomicUsize,
}

impl RoundRobin {
    pub fn new(pool: Arc<Pool>) -> Self {
        RoundRobin {
            pool,
            current: AtomicUsize::new(0),
        }
    }

    pub async fn get_next(&self) -> Option<String> {
        let idx = self
            .current
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
            % self.pool.len().await;
        self.pool.at(idx).await
    }
}
