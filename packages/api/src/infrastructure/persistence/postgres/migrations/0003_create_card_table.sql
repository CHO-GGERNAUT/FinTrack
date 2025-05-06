CREATE TYPE card_issuer AS ENUM (
    'samsung', 'bc', 'woori', 'hana', 'shinhan', 'hyundai', 'kb', 'lotte', 'nh'
);

CREATE TYPE card_brand AS ENUM (
    'visa', 'mastercard', 'amex', 'jcb', 'unionpay', 'discover', 'etc'
);

CREATE TYPE card_type AS ENUM (
    'credit', 'debit', 'prepaid'
);
CREATE TYPE card_status AS ENUM ('active', 'inactive', 'expired', 'closed');


CREATE TABLE cards (
    id UUID PRIMARY KEY,
    version BIGINT NOT NULL,
    user_id UUID NOT NULL REFERENCES "users"(id),

    name TEXT,
    
    card_number BYTEA NOT NULL,
    last_four_digits VARCHAR(4) NOT NULL,
    card_fingerprint TEXT,
    
    card_type card_type NOT NULL,
    card_brand card_brand NOT NULL,
    card_issuer card_issuer NOT NULL,
    status card_status NOT NULL,

    expiration_date DATE NOT NULL,
    issuance_date DATE NOT NULL,
    
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    deleted_at TIMESTAMPTZ
);

CREATE INDEX idx_cards_user_id ON cards(user_id);
CREATE INDEX idx_cards_status ON cards(status);
CREATE INDEX idx_cards_last_number ON cards(last_four_digits);
CREATE UNIQUE INDEX idx_cards_user_fingerprint ON cards (user_id, card_fingerprint);