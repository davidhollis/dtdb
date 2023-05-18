use chrono::{DateTime, Utc};
use diesel::{prelude::*, pg::Pg, sql_types::Text, deserialize::{FromSql, FromSqlRow}, serialize::ToSql, expression::AsExpression};
use identifier_prefix::identifier_prefix;

use crate::data::{identifiers::Identifier, schema::worked_on};

use super::{person::Person, show::Show};

#[derive(Debug, Clone, FromSqlRow, AsExpression)]
#[diesel(sql_type = Text)]
pub enum Context {
    Cast,
    Crew,
    SpecialThanks,
}

#[derive(Debug, Identifiable, Queryable, Selectable, Associations)]
#[diesel(belongs_to(Person))]
#[diesel(belongs_to(Show))]
#[diesel(table_name = worked_on)]
#[identifier_prefix(work)]
pub struct WorkedOn {
    pub id: Identifier<WorkedOn>,
    pub person_id: Identifier<Person>,
    pub show_id: Identifier<Show>,
    pub role: String,
    pub index: i16,
    pub context: Context,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl WorkedOn {
    pub fn new(
        person_id: Identifier<Person>,
        show_id: Identifier<Show>,
        role: String,
        index: i16,
        context: Context,
    ) -> CreateOrUpdateWorkedOn {
        CreateOrUpdateWorkedOn {
            id: Identifier::generate(),
            person_id,
            show_id,
            role,
            index,
            context,
        }
    }

    pub fn update(&self) -> CreateOrUpdateWorkedOn {
        CreateOrUpdateWorkedOn {
            id: self.id.clone(),
            person_id: self.person_id.clone(),
            show_id: self.show_id.clone(),
            role: self.role.clone(),
            index: self.index,
            context: self.context.clone(),
        }
    }
}

#[derive(Identifiable, Insertable, AsChangeset)]
#[diesel(table_name = worked_on)]
#[diesel(treat_none_as_null = true)]
pub struct CreateOrUpdateWorkedOn {
    id: Identifier<WorkedOn>,
    pub person_id: Identifier<Person>,
    pub show_id: Identifier<Show>,
    pub role: String,
    pub index: i16,
    pub context: Context,
}

impl FromSql<Text, Pg> for Context {
    fn from_sql(bytes: diesel::backend::RawValue<'_, Pg>) -> diesel::deserialize::Result<Self> {
        match String::from_sql(bytes)?.as_str() {
            "cast" => Ok(Context::Cast),
            "crew" => Ok(Context::Crew),
            "special_thanks" => Ok(Context::SpecialThanks),
            invalid_context => Err(format!("invalid work context {}", invalid_context).into())
        }
    }
}

impl ToSql<Text, Pg> for Context {
    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, Pg>) -> diesel::serialize::Result {
        let repr = match *self {
            Context::Cast => "cast",
            Context::Crew => "crew",
            Context::SpecialThanks => "special_thanks",
        };

        <str as ToSql<Text, Pg>>::to_sql(repr, out)
    }
}