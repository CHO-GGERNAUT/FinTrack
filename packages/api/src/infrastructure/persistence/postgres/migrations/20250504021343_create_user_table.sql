CREATE TYPE user_status AS ENUM (
    'PendingActivation',
    'Active',
    'Inactive'
    -- 필요시 다른 상태 추가
);

CREATE TABLE "users" (
    id UUID PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    phone_number VARCHAR(50) NOT NULL,

    status user_status NOT NULL,

    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    deleted_at TIMESTAMPTZ

);

CREATE INDEX idx_users_email ON users(email);