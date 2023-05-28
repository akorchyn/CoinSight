-- Step 1: Create a new column allowing null values
ALTER TABLE notifications
ADD COLUMN current_price DECIMAL(20, 8);

-- Step 2: Alter the column to disallow null values
ALTER TABLE notifications
ALTER COLUMN current_price SET NOT NULL;

