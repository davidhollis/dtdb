CREATE TABLE awards (
  "id" CHAR(30), -- e.g., award_123456789012345678901234
  "person_id" CHAR(31) NOT NULL REFERENCES people ("id") ON DELETE CASCADE,

  -- If an award has:
  --   * just a season_id, then that award was given at that season's banquet
  --     and doesn't apply to a specific show (e.g., a scholarship or MVP award)
  --   * both a season_id and a show_id, then that award was given at that
  --     season's banquet for that show, even if that show belongs to a
  --     different season
  "season_id" CHAR(31) NOT NULL REFERENCES seasons ("id") ON DELETE RESTRICT,
  "show_id" CHAR(29) REFERENCES shows ("id") ON DELETE RESTRICT,

  "award_name" VARCHAR(1024) NOT NULL,
  "notes" TEXT,
  "created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT current_timestamp,
  "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT current_timestamp,

  PRIMARY KEY ("id")
);

CREATE INDEX awards__award_names ON awards ("award_name");

SELECT diesel_manage_updated_at('awards');