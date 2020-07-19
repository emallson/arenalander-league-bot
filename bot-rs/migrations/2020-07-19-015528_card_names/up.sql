alter table cards alter column uuid set not null;
alter table cards alter column uuid set data type uuid using uuid::uuid;

create table card_names (
       id serial primary key,
       uuid uuid not null references cards(uuid),
       scryfalloracleid uuid not null,
       language text not null,
       name text not null
);

create index on card_names (scryfalloracleid);
