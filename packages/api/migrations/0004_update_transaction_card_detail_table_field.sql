-- Add migration script here
ALTER TABLE transaction_card_detail
    ALTER COLUMN merchant_id SET NOT NULL;
