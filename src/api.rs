use std::sync::Arc;

use axum::{extract::Path, routing::post, Router};
use tokio::net::TcpListener;

use crate::pool::Pool;

pub async fn create_api(pool: Arc<Pool>) {
    let api = Router::new().route(
        "/register/:addr",
        post(|Path(addr): Path<String>| async move {
            pool.push(addr).await;
            "ok"
        }),
    );
    let listener = TcpListener::bind("127.0.0.1:8081")
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, api)
        .await
        .expect("Failed to start server");
}
