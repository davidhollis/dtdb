use std::sync::Arc;

use axum::{Router, routing::get};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use crate::{handlers::toaster::toaster, application::Application};

pub fn build_routes(app: Arc<Application>) -> Router {

    Router::new()
        .route("/toaster", get(toaster))
        .with_state(app)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
}
