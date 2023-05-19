use chrono::{NaiveDate, DateTime, Utc, Datelike};
use diesel::prelude::*;
use identifier_prefix::identifier_prefix;

use crate::data::{identifiers::Identifier, schema::ec_terms};

use super::person::Person;

#[derive(Debug, Identifiable, Queryable, Selectable, Associations)]
#[diesel(belongs_to(Person))]
#[diesel(table_name = ec_terms)]
#[identifier_prefix(ecterm)]
pub struct ECTerm {
    pub id: Identifier<ECTerm>,
    pub person_id: Identifier<Person>,
    pub role: String,
    pub start_year: NaiveDate,
    pub end_year: NaiveDate,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ECTerm {
    pub fn create(
        person_id: Identifier<Person>,
        role: String,
        start_year: u16,
        end_year: u16
    ) -> Option<CreateOrUpdateECTerm> {
        Some(CreateOrUpdateECTerm {
            id: Identifier::generate(),
            person_id,
            role,
            start_year: NaiveDate::from_ymd_opt(start_year.into(), 1, 1)?,
            end_year: NaiveDate::from_ymd_opt(end_year.into(), 1, 1)?,
        })
    }

    pub fn update(&self) -> CreateOrUpdateECTerm {
        CreateOrUpdateECTerm {
            id: self.id.clone(),
            person_id: self.person_id.clone(),
            role: self.role.clone(),
            start_year: self.start_year,
            end_year: self.end_year
        }
    }

    pub fn format_year_range(&self) -> String {
        let start_year = self.start_year.year();
        let end_year = self.end_year.year();

        if start_year == end_year {
            format!("{}", start_year)
        } else {
            format!("{}–{}", start_year, end_year)
        }
    }
}

#[derive(Identifiable, Insertable, AsChangeset)]
#[diesel(table_name = ec_terms)]
pub struct CreateOrUpdateECTerm {
    id: Identifier<ECTerm>,
    pub person_id: Identifier<Person>,
    pub role: String,
    pub start_year: NaiveDate,
    pub end_year: NaiveDate,
}