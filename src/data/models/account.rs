use chrono::{DateTime, Utc};
use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::{prelude::*, FromSqlRow, AsExpression};
use diesel::serialize::ToSql;
use diesel::sql_types::{Nullable, Text};
use serde::Serialize;

use crate::data::{schema::accounts, identifiers::Identifier};
use identifier_prefix::identifier_prefix;

#[derive(Debug, FromSqlRow, AsExpression, Serialize)]
#[serde(rename_all = "snake_case")]
#[diesel(sql_type = Text)]
pub enum Role {
    Default,
    Editor,
    Admin,
}

#[derive(Debug, Identifiable, Queryable, Selectable)]
#[diesel(table_name = accounts)]
#[identifier_prefix(acct)]
pub struct Account {
    pub id: Identifier<Account>,
    pub oidc_subject: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub role: Role,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

sql_function! { fn coalesce(a: Nullable<Text>, b: Nullable<Text>) -> Nullable<Text>; }

impl Account {
    pub fn upsert_oidc_account(conn: &mut PgConnection, oidc_subject: String, name: Option<String>, email: Option<String>) -> QueryResult<Account> {
        let new_account = CreateOrUpdateAccount {
            id: Identifier::generate(),
            oidc_subject,
            name,
            email,
            role: Role::Default,
        };
        diesel::insert_into(accounts::table)
            .values(&new_account)
            .on_conflict(accounts::oidc_subject)
            .do_update()
            .set((
                accounts::name.eq(coalesce(accounts::name, &new_account.name)),
                accounts::email.eq(coalesce(accounts::email, &new_account.email))
            ))
            .get_result(conn)
    }
}

#[derive(Identifiable, Insertable, Serialize)]
#[diesel(table_name = accounts)]
struct CreateOrUpdateAccount {
    id: Identifier<Account>,
    #[serde(skip_serializing)]
    oidc_subject: String,
    name: Option<String>,
    email: Option<String>,
    role: Role,
}

impl FromSql<Text, Pg> for Role {
    fn from_sql(bytes: diesel::backend::RawValue<'_, Pg>) -> diesel::deserialize::Result<Self> {
        match String::from_sql(bytes)?.as_str() {
            "default" => Ok(Role::Default),
            "editor" => Ok(Role::Editor),
            "admin" => Ok(Role::Admin),
            invalid_role => Err(format!("invalid account role {}", invalid_role).into())
        }
    }
}

impl ToSql<Text, Pg> for Role {
    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, Pg>) -> diesel::serialize::Result {
        let repr = match *self {
            Role::Default => "default",
            Role::Editor => "editor",
            Role::Admin => "admin",
        };

        <str as ToSql<Text, Pg>>::to_sql(repr, out)
    }
}