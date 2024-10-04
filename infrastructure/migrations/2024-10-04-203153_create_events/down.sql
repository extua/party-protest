-- This file should undo anything in `up.sql`
ALTER TABLE `events` ADD COLUMN `start_time` INTEGER NOT NULL;
ALTER TABLE `events` ADD COLUMN `end_time` INTEGER;

