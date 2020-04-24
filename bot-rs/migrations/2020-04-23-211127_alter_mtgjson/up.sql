-- Your SQL goes here
-- TODO: what was the fkey constraint on set? alter table cards drop constraint something
alter table cards
alter column scryfalloracleid type uuid using scryfalloracleid::uuid,
alter column scryfalloracleid set not null,
alter column scryfallid type uuid using scryfallid::uuid,
alter column scryfallid set not null,
alter column scryfallillustrationid type uuid using scryfallillustrationid::uuid,
alter column isarena drop default,
alter column isarena type boolean using isarena = 1,
alter column isarena set not null,
alter column name set not null,
alter column setcode set not null;