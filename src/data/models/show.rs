use chrono::{NaiveDate, DateTime, Utc, Datelike};
use diesel::prelude::*;
use identifier_prefix::identifier_prefix;
use serde::{Serialize, ser::SerializeStruct};

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

    pub fn render_show_dates(&self) -> String {
        if self.use_legacy_date_rendering {
            let year = self.opening_date.year();
            let month = self.opening_date.month();

            match month {
                1..=3 => format!("Winter {year}"),
                4..=6 => format!("Spring {year}"),
                7..=9 => format!("Summer {year}"),
                10..=12 => format!("Fall {year}"),
                _ => format!("An unkowable time spread across the moments of {year}"),
            }
        } else {
            let opening_year = self.opening_date.year();
            let closing_year = self.closing_date.year();
            if opening_year == closing_year {
                let opening_month = self.opening_date.month();
                let closing_month = self.closing_date.month();
                if opening_month == closing_month {
                    // e.g., November 03–18, 2023
                    format!("{}–{}", self.opening_date.format("%B %d"), self.closing_date.format("%d, %Y"))
                } else {
                    // e.g., September 15—October 15, 2023
                    format!("{}–{}", self.opening_date.format("%B %d"), self.closing_date.format("%B %d, %Y"))
                }
            } else {
                // e.g., December 15, 2023—January 15, 2024
                format!("{}–{}", self.opening_date.format("%B %d, %Y"), self.closing_date.format("%B %d, %Y"))
            }
        }
    }
}

// pub struct Show {
//     pub id: Identifier<Show>,
//     pub title: String,
//     pub season_id: Identifier<Season>,
//     pub author: String,
//     pub description: Option<String>,
//     pub fun_facts: Option<String>,
//     pub opening_date: NaiveDate,
//     pub closing_date: NaiveDate,
//     pub use_legacy_date_rendering: bool,
//     pub poster_id: Option<Identifier<Media>>,
//     pub banner_id: Option<Identifier<Media>>,
//     pub created_at: DateTime<Utc>,
//     pub updated_at: DateTime<Utc>,
// }

impl Serialize for Show {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let mut show = serializer.serialize_struct("Show", 13)?;
        show.serialize_field("id", &self.id)?;
        show.serialize_field("title", &self.title)?;
        show.serialize_field("season_id", &self.season_id)?;
        show.serialize_field("author", &self.author)?;
        show.serialize_field("description", &self.description)?;
        show.serialize_field("fun_facts", &self.fun_facts)?;
        show.serialize_field("opening_date", &self.opening_date)?;
        show.serialize_field("closing_date", &self.closing_date)?;
        show.serialize_field("formatted_show_dates", &self.render_show_dates())?;
        show.serialize_field("poster_id", &self.poster_id)?;
        show.serialize_field("banner_id", &self.banner_id)?;
        show.serialize_field("created_at", &self.created_at)?;
        show.serialize_field("updated_at", &self.updated_at)?;
        show.end()
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