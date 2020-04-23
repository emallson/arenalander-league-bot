create table users (
    id serial primary key,
    discordId bigint not null
);

create table leagues (
    id serial primary key,
    title text not null,
    start_date timestamp with time zone not null,
    end_date timestamp with time zone not null
);

create table decks (
    id serial primary key,
    league int not null references leagues(id),
    owner int not null references users(id),
    creation_date timestamp with time zone not null,
    wins int not null default 0 check (wins >= 0),
    losses int not null default 0 check (losses >= 0),
    resigned boolean not null default false
);

create table card_names (
    id serial primary key,
    name text unique not null
);

create table deck_contents (
    id serial primary key,
    deck int references decks(id),
    card int references card_names(id),
    count int not null default 0 check (count > 0)
);