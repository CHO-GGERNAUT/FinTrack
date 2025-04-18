CREATE TABLE "user" (
    id UUID PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    deleted_at TIMESTAMPTZ,
    name TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password TEXT NOT NULL
);

CREATE EXTENSION IF NOT EXISTS ltree;

CREATE TYPE card_issuer AS ENUM (
    'samsung', 'hyundai', 'kb', 'shinhan', 'lotte', 'hana', 'bc', 'etc'
);

CREATE TYPE card_brand AS ENUM (
    'visa', 'mastercard', 'amex', 'jcb', 'unionpay', 'etc'
);

CREATE TYPE card_type AS ENUM (
    'credit', 'debit', 'prepaid'
);

CREATE TABLE account (
    id UUID PRIMARY KEY,
    owner_id UUID NOT NULL REFERENCES "user"(id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    deleted_at TIMESTAMPTZ,
    name TEXT NOT NULL,
    account_type TEXT CHECK (account_type IN ('card', 'bank')) NOT NULL
);

CREATE TABLE card (
    account_id UUID PRIMARY KEY REFERENCES account(id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    deleted_at TIMESTAMPTZ,
    card_number_last4 CHAR(4) NOT NULL,
    encrypted_card_number BYTEA NOT NULL,
    issued_at DATE,
    expires_at DATE,
    billing_day INT CHECK (billing_day BETWEEN 1 AND 31),
    credit_limit NUMERIC,
    brand card_brand NOT NULL,
    issuer card_issuer NOT NULL,
    card_type card_type NOT NULL
);

CREATE TABLE bank (
    account_id UUID PRIMARY KEY REFERENCES account(id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    deleted_at TIMESTAMPTZ,

    name TEXT,
    account_number TEXT
);
CREATE TABLE category (
    id UUID PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    deleted_at TIMESTAMPTZ,
    name TEXT NOT NULL,
    path LTREE NOT NULL UNIQUE
);

CREATE TABLE transaction (
    id UUID PRIMARY KEY,
    account_id UUID NOT NULL REFERENCES account(id),
    category_id UUID REFERENCES category(id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    deleted_at TIMESTAMPTZ,
    amount NUMERIC NOT NULL,
    approved_at TIMESTAMPTZ NOT NULL,
    memo TEXT,
    transaction_type TEXT CHECK (transaction_type IN ('income', 'expense')) NOT NULL
);

-- 사업자/가맹점
CREATE TABLE merchant (
    id UUID PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    deleted_at TIMESTAMPTZ,

    name TEXT NOT NULL,
    biz_number TEXT UNIQUE NOT NULL,
    address TEXT,
    phone TEXT
    
);

CREATE TABLE transaction_card_detail (
    transaction_id UUID PRIMARY KEY REFERENCES transaction(id),
    merchant_id UUID REFERENCES merchant(id),
    installment_months INT
);



CREATE INDEX idx_category_path_gist ON category USING GIST (path);

-- account table index
CREATE INDEX idx_account_owner_id ON account(owner_id);

-- transaction table index
CREATE INDEX idx_transaction_account_id ON transaction(account_id);
CREATE INDEX idx_transaction_date ON transaction(transaction_type);
CREATE INDEX idx_transaction_approved_at ON transaction(approved_at);

-- merchant table index
CREATE INDEX idx_merchant_biz_number ON merchant(biz_number);

-- card table index
CREATE INDEX idx_card_card_number_last4 ON card(card_number_last4);
CREATE INDEX idx_card_encrypted_card_number ON card(encrypted_card_number);
