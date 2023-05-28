-- Step 2: Alter the column to disallow null values
ALTER TABLE notifications
ALTER COLUMN name SET NOT NULL;
