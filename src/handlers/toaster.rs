use std::sync::Arc;

use axum::{response::IntoResponse, extract::State};

use crate::{view::EmptyContext, application::Application};

pub async fn toaster(
    State(app): State<Arc<Application>>
) -> impl IntoResponse {
    app.views.render_page("toaster", EmptyContext {})
}