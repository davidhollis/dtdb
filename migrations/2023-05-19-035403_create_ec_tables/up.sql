CREATE TABLE ec_terms (
  "id" CHAR(31), -- e.g., ecterm_123456789012345678901234
  "person_id" CHAR(31) NOT NULL REFERENCES people ("id") ON DELETE CASCADE,
  "role" VARCHAR(128) NOT NULL,
  "start_year" DATE NOT NULL,
  "end_year" DATE NOT NULL,
  "created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT current_timestamp,
  "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT current_timestamp,

  PRIMARY KEY ("id")
);

CREATE INDEX ec_terms_roles ON ec_terms ("role");

SELECT diesel_manage_updated_at('ec_terms');