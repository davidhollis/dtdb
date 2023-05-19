use chrono::{NaiveDate, Datelike};
use diesel::prelude::*;
use ordinal::Ordinal;

use crate::data::{identifiers::Identifier, schema::seasons};
use identifier_prefix::identifier_prefix;

#[derive(Debug, Identifiable, Queryable, Selectable)]
#[diesel(table_name = seasons)]
#[identifier_prefix(season)]
pub struct Season {
    pub id: Identifier<Season>,
    pub season_number: i16,
    pub start_year: NaiveDate,
    pub end_year: NaiveDate,
}

impl Season {
    pub fn create(number: i16, start_year: u16, end_year: u16) -> Option<Season> {
        Some(Season {
            id: Identifier::default(),
            season_number: number,
            start_year: NaiveDate::from_ymd_opt(start_year.into(), 08, 01)?,
            end_year: NaiveDate::from_ymd_opt(end_year.into(), 07, 31)?,
        })
    }

    pub fn title(&self) -> String {
        format!(
            "{} Season ({}–{})",
            Ordinal(self.season_number).to_string(),
            self.start_year.year(),
            self.end_year.year()
        )
    }
}