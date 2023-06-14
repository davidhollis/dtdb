use std::sync::Arc;

use axum::extract::State;

use crate::application::Application;

use super::HandlerResult;

pub async fn toaster(
    State(app): State<Arc<Application>>
) -> HandlerResult {
    Ok(app.views.render_page("toaster")?)
}