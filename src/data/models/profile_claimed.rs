use chrono::{DateTime, Utc};
use diesel::prelude::*;
use identifier_prefix::identifier_prefix;
use serde::Serialize;

use crate::data::{identifiers::Identifier, schema::profile_claimed};

use super::{account::Account, person::Person};

#[derive(Debug, Identifiable, Queryable, Selectable, Associations, Serialize)]
#[diesel(belongs_to(Account))]
#[diesel(belongs_to(Person))]
#[diesel(table_name = profile_claimed)]
#[identifier_prefix(claim)]
pub struct ProfileClaimed {
    pub id: Identifier<ProfileClaimed>,
    pub account_id: Identifier<Account>,
    pub person_id: Identifier<Person>,
    pub created_at: DateTime<Utc>,
}

impl ProfileClaimed {
    pub fn new_attempt(account_id: Identifier<Account>, person_id: Identifier<Person>) -> AttemptToClaim {
        AttemptToClaim {
            id: Identifier::generate(),
            account_id,
            person_id
        }
    }
}

#[derive(Identifiable, Insertable)]
#[diesel(table_name = profile_claimed)]
pub struct AttemptToClaim {
    id: Identifier<ProfileClaimed>,
    account_id: Identifier<Account>,
    person_id: Identifier<Person>,
}