use chrono::{NaiveDate, Datelike};
use diesel::prelude::*;
use ordinal::Ordinal;
use serde::{Serialize, ser::SerializeStruct};

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

impl Serialize for Season {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let mut season = serializer.serialize_struct("Season", 3)?;
        season.serialize_field("id", &self.id)?;
        season.serialize_field("season_number", &self.season_number)?;
        season.serialize_field("title", &self.title())?;
        season.end()
    }
}