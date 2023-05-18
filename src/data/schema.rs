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

diesel::table! {
    media (id) {
        id -> Bpchar,
        primary_url -> Varchar,
        thumbnail_url -> Nullable<Varchar>,
        icon_url -> Nullable<Varchar>,
        banner_url -> Nullable<Varchar>,
        notes -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    media,
);
