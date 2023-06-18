use std::sync::Arc;

use axum::Router;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use crate::{handlers::{toaster, seasons}, application::Application, util, data::{models::season::Season, identifiers::IdentifierPrefix}};

pub fn build_routes(app: Arc<Application>) -> Router {
    Router::new()
        .nest(
            util::route_for_id_prefix(Season::identifier_prefix()),
            seasons::routes()
        )
        .nest("/toaster", toaster::routes())
        .with_state(app)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
}
