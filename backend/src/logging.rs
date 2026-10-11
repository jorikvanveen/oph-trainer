use std::{
    sync::atomic::{AtomicU64, Ordering},
    time::Instant,
};

use axum::{
    extract::{MatchedPath, Request},
    middleware::Next,
    response::Response,
};
use tracing::Instrument;

static NEXT_REQUEST_ID: AtomicU64 = AtomicU64::new(1);

pub(crate) async fn log_request(request: Request, next: Next) -> Response {
    // Only record registered routes; arbitrary URLs and query strings can contain secrets.
    let route = request
        .extensions()
        .get::<MatchedPath>()
        .map(MatchedPath::as_str)
        .unwrap_or("unmatched");
    // Keep context available when the filter enables only warnings or errors.
    let span = tracing::error_span!(
        "request",
        request_id = NEXT_REQUEST_ID.fetch_add(1, Ordering::Relaxed),
        method = %request.method(),
        route,
    );
    async move {
        let started = Instant::now();
        tracing::debug!("Request started");
        let response = next.run(request).await;
        let status = response.status();
        let elapsed_ms = started.elapsed().as_secs_f64() * 1000.0;
        if status.is_server_error() {
            tracing::error!(status = status.as_u16(), elapsed_ms, "Request completed");
        } else if status.is_client_error() {
            tracing::warn!(status = status.as_u16(), elapsed_ms, "Request completed");
        } else {
            tracing::info!(status = status.as_u16(), elapsed_ms, "Request completed");
        }
        response
    }
    .instrument(span)
    .await
}
