-- Step 1: Create a new column allowing null values
ALTER TABLE notifications
ADD COLUMN name VARCHAR(255);


