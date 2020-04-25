-- Your SQL goes here
create table deck_view_tokens (
    id serial primary key,
    deck int not null references decks(id),
    token uuid not null
);

create index deck_tokens_deck_id on deck_view_tokens (deck, token);