use std::sync::Arc;

use axum::{extract::State, Router, routing::get};

use crate::application::Application;

use super::HandlerResult;

pub fn routes() -> Router<Arc<Application>> {
    Router::new()
        .route("/", get(toaster))
}

async fn toaster(
    State(app): State<Arc<Application>>
) -> HandlerResult {
    Ok(app.views.render_page("toaster")?)
}