-- This file should undo anything in `up.sql`
drop table if exists card_names;
alter table cards alter column uuid drop not null;
alter table cards alter column uuid set data type text;
