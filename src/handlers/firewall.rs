use crate::{
    dtos::{ClosePortForm, OpenPortForm, OpenPortPartiallyForm},
    error::AppResult,
    states::AppState,
};
use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Form,
};
use hyper::StatusCode;

#[inline(always)]
pub async fn open_port(
    State(state): State<AppState>,
    Form(form): Form<OpenPortForm>,
) -> AppResult<Response> {
    state
        .firewall
        .open_port(form.port, form.championship_id)
        .await?;

    Ok(StatusCode::OK.into_response())
}

#[inline(always)]
pub async fn open_port_partially(
    State(state): State<AppState>,
    Form(form): Form<OpenPortPartiallyForm>,
) -> AppResult<Response> {
    state
        .firewall
        .open_port_partially(form.port, form.address, form.championship_id)
        .await?;

    Ok(StatusCode::OK.into_response())
}

#[inline(always)]
pub async fn close_port(
    State(state): State<AppState>,
    Form(form): Form<ClosePortForm>,
) -> AppResult<Response> {
    state.firewall.close_port(&form.championship_id).await?;

    Ok(StatusCode::OK.into_response())
}

#[inline(always)]
pub async fn close_all_ports(State(state): State<AppState>) -> AppResult<Response> {
    state.firewall.close_all_ports().await?;

    Ok(StatusCode::OK.into_response())
}
