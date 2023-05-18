use chrono::{NaiveDate, DateTime, Utc};
use diesel::prelude::*;
use identifier_prefix::identifier_prefix;

use crate::data::{identifiers::Identifier, schema::shows};

use super::{season::Season, media::Media};

#[derive(Debug, Identifiable, Queryable, Selectable, Associations)]
#[diesel(belongs_to(Season))]
#[diesel(table_name = shows)]
#[identifier_prefix(show)]
pub struct Show {
    pub id: Identifier<Show>,
    pub title: String,
    pub season_id: Identifier<Season>,
    pub author: String,
    pub description: Option<String>,
    pub fun_facts: Option<String>,
    pub opening_date: NaiveDate,
    pub closing_date: NaiveDate,
    pub use_legacy_date_rendering: bool,
    pub poster: Option<Identifier<Media>>,
    pub banner: Option<Identifier<Media>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Show {
    pub fn new(
        title: String,
        season_id: Identifier<Season>,
        author: String,
        opening_date: NaiveDate,
        closing_date: NaiveDate,
    ) -> NewShow {
        NewShow {
            id: Identifier::generate(),
            title,
            season_id,
            author,
            description: None,
            fun_facts: None,
            opening_date,
            closing_date,
            use_legacy_date_rendering: false,
            poster: None,
            banner: None,
        }
    }
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = shows)]
#[diesel(treat_none_as_null = true)]
pub struct NewShow {
    id: Identifier<Show>,
    pub title: String,
    pub season_id: Identifier<Season>,
    pub author: String,
    pub description: Option<String>,
    pub fun_facts: Option<String>,
    pub opening_date: NaiveDate,
    pub closing_date: NaiveDate,
    pub use_legacy_date_rendering: bool,
    pub poster: Option<Identifier<Media>>,
    pub banner: Option<Identifier<Media>>,
}