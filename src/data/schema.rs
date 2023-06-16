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
    awards (id) {
        id -> Bpchar,
        person_id -> Bpchar,
        season_id -> Bpchar,
        show_id -> Nullable<Bpchar>,
        award_name -> Varchar,
        notes -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    ec_terms (id) {
        id -> Bpchar,
        person_id -> Bpchar,
        role -> Varchar,
        start_year -> Date,
        end_year -> Date,
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

diesel::table! {
    people (id) {
        id -> Bpchar,
        name -> Varchar,
        picture_id -> Nullable<Bpchar>,
        bio -> Nullable<Text>,
        account_id -> Nullable<Bpchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    profile_claimed (id) {
        id -> Bpchar,
        account_id -> Bpchar,
        person_id -> Bpchar,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    seasons (id) {
        id -> Bpchar,
        season_number -> Int2,
        start_year -> Date,
        end_year -> Date,
    }
}

diesel::table! {
    shows (id) {
        id -> Bpchar,
        title -> Varchar,
        season_id -> Bpchar,
        author -> Varchar,
        description -> Nullable<Text>,
        fun_facts -> Nullable<Text>,
        opening_date -> Date,
        closing_date -> Date,
        use_legacy_date_rendering -> Bool,
        poster_id -> Nullable<Bpchar>,
        banner_id -> Nullable<Bpchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    tagged_in (id) {
        id -> Bpchar,
        media_id -> Bpchar,
        subject_id -> Varchar,
    }
}

diesel::table! {
    worked_on (id) {
        id -> Bpchar,
        person_id -> Bpchar,
        show_id -> Bpchar,
        role -> Varchar,
        index -> Int2,
        context -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(awards -> people (person_id));
diesel::joinable!(awards -> seasons (season_id));
diesel::joinable!(awards -> shows (show_id));
diesel::joinable!(ec_terms -> people (person_id));
diesel::joinable!(people -> accounts (account_id));
diesel::joinable!(people -> media (picture_id));
diesel::joinable!(profile_claimed -> accounts (account_id));
diesel::joinable!(profile_claimed -> people (person_id));
diesel::joinable!(shows -> seasons (season_id));
diesel::joinable!(tagged_in -> media (media_id));
diesel::joinable!(worked_on -> people (person_id));
diesel::joinable!(worked_on -> shows (show_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    awards,
    ec_terms,
    media,
    people,
    profile_claimed,
    seasons,
    shows,
    tagged_in,
    worked_on,
);
