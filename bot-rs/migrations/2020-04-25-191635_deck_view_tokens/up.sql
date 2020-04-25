-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
create table deck_view_tokens (
    id serial primary key,
    deck int not null references decks(id),
    token uuid not null default uuid_generate_v4()
);

create index deck_tokens_deck_id on deck_view_tokens (deck, token);