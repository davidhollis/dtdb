CREATE TABLE worked_on (
  "id" CHAR(29), -- e.g., work_123456789012345678901234
  "person_id" CHAR(31) NOT NULL REFERENCES people ("id") ON DELETE CASCADE,
  "show_id" CHAR(29) NOT NULL REFERENCES shows ("id") ON DELETE CASCADE,
  "role" VARCHAR(1024) NOT NULL,
  "index" SMALLINT NOT NULL,
  "context" VARCHAR(32) NOT NULL,
  "created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT current_timestamp,
  "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT current_timestamp,

  PRIMARY KEY ("id")
);

SELECT diesel_manage_updated_at('worked_on');