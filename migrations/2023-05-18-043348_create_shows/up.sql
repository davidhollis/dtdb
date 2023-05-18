CREATE TABLE shows (
  "id" CHAR(29), -- e.g., show_123456789012345678901234
  "title" VARCHAR(255) NOT NULL,
  "season_id" CHAR(31) NOT NULL REFERENCES seasons ("id") ON DELETE RESTRICT,
  "author" VARCHAR(1024) NOT NULL,
  "description" TEXT,
  "fun_facts" TEXT,
  "opening_date" DATE NOT NULL,
  "closing_date" DATE NOT NULL,
  "use_legacy_date_rendering" BOOLEAN NOT NULL DEFAULT 'false',
  "poster" CHAR(30) REFERENCES media ("id") ON DELETE SET NULL,
  "banner" CHAR(30) REFERENCES media ("id") ON DELETE SET NULL,
  "created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT current_timestamp,
  "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT current_timestamp,

  PRIMARY KEY ("id")
);

SELECT diesel_manage_updated_at('shows');