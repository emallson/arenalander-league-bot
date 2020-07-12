-- This file should undo anything in `up.sql`
alter table cards drop constraint if exists cards_uuid_unique;
