-- This file should undo anything in `up.sql`
alter table cards
alter column scryfalloracleid type text,
alter column scryfalloracleid drop not null,
alter column scryfallid type text,
alter column scryfallid drop not null,
alter column scryfallillustrationid type text,
alter column isarena type bigint using isarena::int,
alter column isarena set default 0,
alter column isarena drop not null,
alter column name drop not null,
alter column setcode drop not null;