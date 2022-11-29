-- This file should undo anything in `up.sql`
ALTER TABLE posts ALTER COLUMN feed_id SET NULL;
