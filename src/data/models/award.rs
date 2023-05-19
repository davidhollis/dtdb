use chrono::{DateTime, Utc};
use diesel::prelude::*;
use identifier_prefix::identifier_prefix;

use crate::data::{identifiers::Identifier, schema::awards};

use super::{person::Person, show::Show, season::Season};

#[derive(Debug, Identifiable, Queryable, Selectable, Associations)]
#[diesel(belongs_to(Person))]
#[diesel(belongs_to(Season))]
#[diesel(belongs_to(Show))]
#[diesel(table_name = awards)]
#[identifier_prefix(award)]
pub struct Award {
    pub id: Identifier<Award>,
    pub person_id: Identifier<Person>,
    pub season_id: Identifier<Season>,
    pub show_id: Option<Identifier<Show>>,
    pub award_name: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Award {
    pub fn new(
        person_id: Identifier<Person>,
        season_id: Identifier<Season>,
        award_name: String,
    ) -> CreateOrUpdateAward {
        CreateOrUpdateAward {
            id: Identifier::generate(),
            person_id,
            season_id,
            show_id: None,
            award_name,
            notes: None
        }
    }

    pub fn update(&self) -> CreateOrUpdateAward {
        CreateOrUpdateAward {
            id: self.id.clone(),
            person_id: self.person_id.clone(),
            season_id: self.season_id.clone(),
            show_id: self.show_id.clone(),
            award_name: self.award_name.clone(),
            notes: self.notes.clone()
        }
    }
}

#[derive(Identifiable, Insertable, AsChangeset)]
#[diesel(table_name = awards)]
#[diesel(treat_none_as_null = true)]
pub struct CreateOrUpdateAward {
    id: Identifier<Award>,
    pub person_id: Identifier<Person>,
    pub season_id: Identifier<Season>,
    pub show_id: Option<Identifier<Show>>,
    pub award_name: String,
    pub notes: Option<String>,
}