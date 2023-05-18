CREATE TABLE people (
  "id" CHAR(31), -- e.g., person_123456789012345678901234

  -- As with accounts, I'm making some assumptions about the length of names.
  -- We can always make this longer later if we need to.
  "name" VARCHAR(1024) NOT NULL,

  "picture_id" CHAR(30) REFERENCES media ("id") ON DELETE SET NULL,
  "bio" TEXT,
  "account_id" CHAR(29) REFERENCES accounts ("id") ON DELETE RESTRICT,
  "created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT current_timestamp,
  "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT current_timestamp,

  PRIMARY KEY ("id")
);

SELECT diesel_manage_updated_at('people');