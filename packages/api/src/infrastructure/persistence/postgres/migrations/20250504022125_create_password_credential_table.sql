-- Add migration script here
CREATE TABLE password_credentials (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL UNIQUE REFERENCES users(id),
    password_hash TEXT NOT NULL,
    is_locked BOOLEAN NOT NULL DEFAULT FALSE,

    last_used_at TIMESTAMPTZ,
    failed_attempts SMALLINT NOT NULL DEFAULT 0 CHECK (failed_attempts >= 0),

    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    deleted_at TIMESTAMPTZ
);

