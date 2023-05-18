use chrono::{Utc, DateTime};
use diesel::prelude::*;

use crate::data::{identifiers::Identifier, schema::media};
use identifier_prefix::identifier_prefix;

#[derive(Debug, Identifiable, Queryable, Selectable)]
#[diesel(table_name = media)]
#[identifier_prefix(media)]
pub struct Media {
    pub id: Identifier<Media>,
    pub primary_url: String,
    pub thumbnail_url: Option<String>,
    pub icon_url: Option<String>,
    pub banner_url: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Media {
    pub fn new(primary_url: String) -> NewMedia {
        NewMedia {
            id: Identifier::generate(),
            primary_url,
            thumbnail_url: None,
            icon_url: None,
            banner_url: None,
            notes: None
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = media)]
pub struct NewMedia {
    id: Identifier<Media>,
    pub primary_url: String,
    pub thumbnail_url: Option<String>,
    pub icon_url: Option<String>,
    pub banner_url: Option<String>,
    pub notes: Option<String>,
}