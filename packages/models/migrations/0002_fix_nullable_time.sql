-- Add migration script here
ALTER TABLE "user" 
ALTER COLUMN created_at SET NOT NULL,
ALTER COLUMN updated_at SET NOT NULL;

