-- Add column "status" to subscriptions table
ALTER TABLE subscriptions
ADD COLUMN status TEXT NULL;