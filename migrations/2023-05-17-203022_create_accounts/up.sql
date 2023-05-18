CREATE TABLE accounts (
  "id" CHAR(29), -- e.g., acct_123456789012345678901234

  -- per https://developers.google.com/identity/openid-connect/openid-connect#obtainuserinfo,
  -- the subject (`sub`) claim in the id token is unique among all google accounts, never changes,
  -- and has a maximum length of 255 ASCII characters.
  "oidc_subject" VARCHAR(255) NOT NULL,

  -- I'm being lazy here with my VARCHAR lengths. If anyone complains we can resize them later.
  "name" VARCHAR(1024),
  "email" VARCHAR(1024),

  -- See data::models::account::Role
  "role" VARCHAR(32) NOT NULL DEFAULT 'default',

  "created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT current_timestamp,
  "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT current_timestamp,

  PRIMARY KEY ("id"),
  UNIQUE ("oidc_subject")
);

SELECT diesel_manage_updated_at('accounts');
