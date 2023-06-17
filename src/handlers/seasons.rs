use std::sync::Arc;

use axum::Router;
use axum::extract::{State, Path};
use axum::routing::get;
use diesel::dsl::today;
use diesel::prelude::*;
use tera::Context;

use crate::data::models::season::Season;
use crate::data::schema::seasons::dsl::*;
use crate::application::Application;

use super::HandlerResult;

pub fn routes() -> Router<Arc<Application>> {
    Router::new()
        .route("/", get(list))
        .route("/:season_id", get(single))
}

async fn list(
    State(app): State<Arc<Application>>
) -> HandlerResult {
    let all_seasons = app.with_db_connection(|db| {
        Ok(seasons.filter(start_year.le(today)).load::<Season>(db)?)
    })?;
    let mut context = Context::new();
    context.insert("seasons", &all_seasons);
    Ok(app.views.render_page_with("seasons/list", &context)?)
}

async fn single(
    Path(season_id): Path<String>,
    State(app): State<Arc<Application>>
) -> HandlerResult {
    let season = app.with_db_connection(|db| {
        Ok(seasons.find(season_id).first::<Season>(db)?)
    })?;
    let mut context = Context::new();
    context.insert("season", &season);
    Ok(app.views.render_page_with("seasons/single", &context)?)
}