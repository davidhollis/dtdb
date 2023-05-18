CREATE TABLE media (
  "id" CHAR(30), -- e.g., media_123456789012345678901234

  -- I'm expecting these will look like an s3 URL with a uuid in there somewhere
  -- e.g.,
  --   https://some-dtdb-bucket.s3.amazonaws.com/media/0346fc15-4577-49aa-95ba-7269cc4b8187/primary.jpg
  -- or maybe a content checksum. Not sure. Either way, that weighs in at
  -- well under 255 characters.
  "primary_url" VARCHAR(255) NOT NULL,
  "thumbnail_url" VARCHAR(255),
  "icon_url" VARCHAR(255),
  "banner_url" VARCHAR(255),
  
  "notes" TEXT,
  "created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT current_timestamp,
  "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT current_timestamp,

  PRIMARY KEY ("id")
);

SELECT diesel_manage_updated_at('media');
