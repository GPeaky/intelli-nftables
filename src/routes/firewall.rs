use axum::{
    error_handling::HandleErrorLayer,
    routing::{get, IntoMakeService},
    Router,
};
use hyper::StatusCode;
use std::time::Duration;
use tower::{load_shed::LoadShedLayer, ServiceBuilder};

use crate::handlers::{close_all_ports, close_port, open_port, open_port_partially};

async fn handle_error(e: Box<dyn std::error::Error + Send + Sync>) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Unhandled internal error: {}", e),
    )
}

#[inline(always)]
pub(crate) async fn service_routes() -> IntoMakeService<Router> {
    Router::new()
        .route("/open", get(open_port))
        .route("/open_partially", get(open_port_partially))
        .route("/close", get(close_port))
        .route("/close_all", get(close_all_ports))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(handle_error))
                .layer(LoadShedLayer::new())
                .timeout(Duration::from_secs(3)),
        )
        .into_make_service()
}
