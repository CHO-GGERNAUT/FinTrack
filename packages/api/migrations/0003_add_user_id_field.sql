-- 1. Add the user_id column, allowing NULLs initially
ALTER TABLE transactions ADD COLUMN user_id UUID REFERENCES "users"(id);

-- 2. Update existing transactions based on the associated account owner
--    (Adjust the logic if your schema differs)
UPDATE transactions t
SET user_id = a.owner_id
FROM accounts a
WHERE t.account_id = a.id AND t.user_id IS NULL;

-- 3. Add the NOT NULL constraint now that all rows have a user_id
--    (Ensure the UPDATE statement successfully populated all rows before running this)
ALTER TABLE transactions ALTER COLUMN user_id SET NOT NULL;
