-- Add migration script here
ALTER TABLE cards
  DROP COLUMN created_at,
  DROP COLUMN updated_at,
  DROP COLUMN deleted_at;