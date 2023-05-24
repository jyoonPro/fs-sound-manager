use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use axum_server::Handle;
use serde::{Deserialize, Serialize};
use std::net::{Ipv4Addr, SocketAddr};

pub struct AudioServer {
    tokio_handle: Option<tokio::task::JoinHandle<()>>,
    axum_handle: Option<Handle>,
}

impl AudioServer {
    pub fn new() -> Self {
        Self {
            tokio_handle: None,
            axum_handle: None,
        }
    }

    pub fn start(&mut self, port: u16) {
        println!("Server started on port {}", port);
        let app = Router::new()
            .route("/", get(root));

        let handle = Handle::new();
        self.axum_handle = Some(handle.clone());

        let addr = SocketAddr::from(([127, 0, 0, 1], port));
        self.tokio_handle = Some(tokio::spawn(async move {
            axum_server::bind(addr)
                .handle(handle)
                .serve(app.into_make_service())
                .await
                .unwrap();
        }));
    }

    pub fn stop(&mut self) {
        if let Some(axum_handle) = self.axum_handle.take() {
            axum_handle.shutdown();

            if let Some(tokio_handle) = self.tokio_handle.take() {
                tokio_handle.abort();
            }
        }
    }
}

async fn root() -> &'static str {
    "Hello, World!"
}
