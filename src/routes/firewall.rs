use crate::{
    handlers::{close_all_ports, close_port, open_port, open_port_partially},
    states::AppStateInner,
};
use axum::{
    error_handling::HandleErrorLayer,
    routing::{get, post, IntoMakeService},
    Router,
};
use hyper::StatusCode;
use std::{sync::Arc, time::Duration};
use tower::{load_shed::LoadShedLayer, ServiceBuilder};

async fn handle_error(e: Box<dyn std::error::Error + Send + Sync>) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Unhandled internal error: {}", e),
    )
}

#[inline(always)]
pub fn service_routes() -> IntoMakeService<Router> {
    let firewall_service = Arc::new(AppStateInner::new());

    Router::new()
        .route("/open", post(open_port))
        .route("/open_partially", post(open_port_partially))
        .route("/close", post(close_port))
        .route("/close_all", get(close_all_ports))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(handle_error))
                .layer(LoadShedLayer::new())
                .timeout(Duration::from_secs(3)),
        )
        .with_state(firewall_service)
        .into_make_service()
}
