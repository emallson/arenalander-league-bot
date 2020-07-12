-- Your SQL goes here
alter table cards add constraint cards_uuid_unique unique(uuid);
