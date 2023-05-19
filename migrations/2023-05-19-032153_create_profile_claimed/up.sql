CREATE TABLE profile_claimed (
  "id" CHAR(30), -- e.g., claim_123456789012345678901234
  "account_id" CHAR(29) NOT NULL REFERENCES accounts ("id") ON DELETE CASCADE,
  "person_id" CHAR(31) NOT NULL REFERENCES people ("id") ON DELETE CASCADE,
  "created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT current_timestamp,

  PRIMARY KEY ("id")
);
