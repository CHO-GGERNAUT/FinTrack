-- Add migration script here

CREATE TYPE card_issuer_new AS ENUM ('samsung', 'bc', 'woori', 'hana', 'shinhan', 'hyundai', 'kb', 'lotte', 'nh');

ALTER TABLE card
    ALTER COLUMN issuer TYPE card_issuer_new
    USING issuer::text::card_issuer_new;

DROP TYPE card_issuer;

ALTER TYPE card_issuer_new RENAME TO card_issuer;