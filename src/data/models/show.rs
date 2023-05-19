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
    pub poster_id: Option<Identifier<Media>>,
    pub banner_id: Option<Identifier<Media>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Show {
    pub fn create(
        title: String,
        season_id: Identifier<Season>,
        author: String,
        opening_date: NaiveDate,
        closing_date: NaiveDate,
    ) -> CreateOrUpdateShow {
        CreateOrUpdateShow {
            id: Identifier::generate(),
            title,
            season_id,
            author,
            description: None,
            fun_facts: None,
            opening_date,
            closing_date,
            use_legacy_date_rendering: false,
            poster_id: None,
            banner_id: None,
        }
    }

    pub fn update(&self) -> CreateOrUpdateShow {
        CreateOrUpdateShow {
            id: self.id.clone(),
            title: self.title.clone(),
            season_id: self.season_id.clone(),
            author: self.author.clone(),
            description: self.description.clone(),
            fun_facts: self.fun_facts.clone(),
            opening_date: self.opening_date.clone(),
            closing_date: self.closing_date.clone(),
            use_legacy_date_rendering: self.use_legacy_date_rendering,
            poster_id: self.poster_id.clone(),
            banner_id: self.banner_id.clone(),
        }
    }
}

#[derive(Identifiable, Insertable, AsChangeset)]
#[diesel(table_name = shows)]
#[diesel(treat_none_as_null = true)]
pub struct CreateOrUpdateShow {
    id: Identifier<Show>,
    pub title: String,
    pub season_id: Identifier<Season>,
    pub author: String,
    pub description: Option<String>,
    pub fun_facts: Option<String>,
    pub opening_date: NaiveDate,
    pub closing_date: NaiveDate,
    pub use_legacy_date_rendering: bool,
    pub poster_id: Option<Identifier<Media>>,
    pub banner_id: Option<Identifier<Media>>,
}