use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{handlers::toaster::toaster, application::Application};

pub fn build_routes(app: Arc<Application>) -> Router {
    Router::new()
        .route("/toaster", get(toaster))
        .with_state(app)
}