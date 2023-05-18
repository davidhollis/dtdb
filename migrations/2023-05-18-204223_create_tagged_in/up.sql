CREATE TABLE tagged_in (
  "id" CHAR(28), -- e.g., tag_123456789012345678901234
  "media_id" CHAR(30) NOT NULL REFERENCES media ("id") ON DELETE CASCADE,
  "subject_id" VARCHAR(31) NOT NULL, -- references people ("id") or shows ("id")

  UNIQUE ("media_id", "subject_id"),
  PRIMARY KEY ("id")
);