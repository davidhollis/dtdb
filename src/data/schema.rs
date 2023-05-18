// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Bpchar,
        oidc_subject -> Varchar,
        name -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        role -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
