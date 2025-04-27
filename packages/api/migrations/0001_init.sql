CREATE TABLE "users" (
    id UUID PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
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

CREATE TABLE accounts (
    id UUID PRIMARY KEY,
    owner_id UUID NOT NULL REFERENCES "users"(id),
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    deleted_at TIMESTAMPTZ,
    account_type TEXT CHECK (account_type IN ('card', 'bank')) NOT NULL
);

CREATE TABLE cards (
    account_id UUID PRIMARY KEY REFERENCES accounts(id),
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    deleted_at TIMESTAMPTZ,
    card_number_last4 CHAR(4) NOT NULL,
    encrypted_card_number BYTEA UNIQUE NOT NULL,
    issued_at DATE,
    expires_at DATE,
    billing_day INT CHECK (billing_day BETWEEN 1 AND 31),
    brand card_brand NOT NULL,
    issuer card_issuer NOT NULL,
    card_type card_type NOT NULL,
    name TEXT,
    memo TEXT
);

CREATE TABLE banks (
    account_id UUID PRIMARY KEY REFERENCES accounts(id),
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    deleted_at TIMESTAMPTZ,
    account_number TEXT NOT NULL,
    name TEXT,
    memo TEXT
    
);
CREATE TABLE categories (
    id UUID PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    deleted_at TIMESTAMPTZ,
    name TEXT NOT NULL,
    path LTREE NOT NULL UNIQUE
);

CREATE TABLE transactions (
    id UUID PRIMARY KEY,
    account_id UUID NOT NULL REFERENCES accounts(id),
    category_id UUID REFERENCES categories(id),
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    deleted_at TIMESTAMPTZ,
    amount NUMERIC NOT NULL,
    approved_at TIMESTAMPTZ NOT NULL,
    memo TEXT,
    transaction_type TEXT CHECK (transaction_type IN ('income', 'expense')) NOT NULL
);

-- 사업자/가맹점
CREATE TABLE merchants (
    id UUID PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    deleted_at TIMESTAMPTZ,

    name TEXT NOT NULL,
    biz_number TEXT UNIQUE NOT NULL,
    address TEXT,
    phone TEXT
    
);

CREATE TABLE transaction_card_detail (
    transaction_id UUID PRIMARY KEY REFERENCES transactions(id),
    merchant_id UUID REFERENCES merchants(id),
    installment_months INT
);



CREATE INDEX idx_categories_path_gist ON categories USING GIST (path);

-- account table index
CREATE INDEX idx_accounts_owner_id ON accounts(owner_id);

-- transaction table index
CREATE INDEX idx_transactions_account_id ON transactions(account_id);
CREATE INDEX idx_transactions_date ON transactions(transaction_type);
CREATE INDEX idx_transactions_approved_at ON transactions(approved_at);

-- merchant table index
CREATE INDEX idx_merchants_biz_number ON merchants(biz_number);

-- card table index
CREATE INDEX idx_cards_card_number_last4 ON cards(card_number_last4);
CREATE INDEX idx_cards_encrypted_card_number ON cards(encrypted_card_number);
