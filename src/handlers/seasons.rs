use std::sync::Arc;

use axum::extract::State;
use diesel::prelude::*;
use tera::Context;

use crate::data::models::season::Season;
use crate::data::schema::seasons::dsl::*;
use crate::application::Application;

use super::HandlerResult;

pub async fn list(
    State(app): State<Arc<Application>>
) -> HandlerResult {
    let all_seasons = app.with_db_connection(|db| {
        Ok(seasons.load::<Season>(db)?)
    })?;
    let context = Context::new();
    context.insert("seasons", &all_seasons);
    Ok(app.views.render_page_with("seasons/list", &context)?)
}