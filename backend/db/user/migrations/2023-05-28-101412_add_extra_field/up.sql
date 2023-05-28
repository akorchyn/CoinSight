-- Step 1: Create a new column allowing null values
ALTER TABLE notifications
ADD COLUMN cryptocurrency VARCHAR(255);

-- Step 2: Alter the column to disallow null values
ALTER TABLE notifications
ALTER COLUMN cryptocurrency SET NOT NULL;

