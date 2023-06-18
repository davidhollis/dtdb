use chrono::{DateTime, Utc};
use diesel::prelude::*;
use identifier_prefix::identifier_prefix;
use serde::Serialize;

use crate::data::{identifiers::Identifier, schema::people};

use super::{media::Media, account::Account};

#[derive(Debug, Identifiable, Queryable, Selectable, Associations, Serialize)]
#[diesel(belongs_to(Account))]
#[diesel(table_name = people)]
#[identifier_prefix(person)]
pub struct Person {
    pub id: Identifier<Person>,
    pub name: String,
    pub picture_id: Option<Identifier<Media>>,
    pub bio: Option<String>,
    pub account_id: Option<Identifier<Account>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Person {
    pub fn create(name: String) -> CreateOrUpdatePerson {
        CreateOrUpdatePerson {
            id: Identifier::generate(),
            name,
            picture_id: None,
            bio: None
        }
    }

    pub fn update(&self) -> CreateOrUpdatePerson {
        CreateOrUpdatePerson {
            id: self.id.clone(),
            name: self.name.clone(),
            picture_id: self.picture_id.clone(),
            bio: self.bio.clone()
        }
    }

    pub fn link_account(&self, account_id: Identifier<Account>) -> AssociateAccountWithPerson {
        AssociateAccountWithPerson {
            id: self.id.clone(),
            account_id
        }
    }
}

#[derive(Identifiable, Insertable, AsChangeset)]
#[diesel(table_name = people)]
#[diesel(treat_none_as_null = true)]
pub struct CreateOrUpdatePerson {
    id: Identifier<Person>,
    pub name: String,
    pub picture_id: Option<Identifier<Media>>,
    pub bio: Option<String>,
}

#[derive(Identifiable, AsChangeset)]
#[diesel(table_name = people)]
#[diesel(treat_none_as_null = true)]
pub struct AssociateAccountWithPerson {
    id: Identifier<Person>,
    account_id: Identifier<Account>,
}