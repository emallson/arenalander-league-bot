create table users (
    id serial primary key,
    discordId bigint unique not null,
    name text not null
);

create table leagues (
    id serial primary key,
    title text not null,
    start_date timestamp with time zone not null,
    end_date timestamp with time zone not null
);

create table decks (
    id serial primary key,
    league int references leagues(id) on delete set null,
    owner int not null references users(id),
    creation_date timestamp with time zone not null,
    resigned boolean not null default false,
    active boolean not null default true
);

create table matches (
    id serial primary key,
    date timestamp with time zone not null,
    winning_deck int not null references decks(id),
    losing_deck int not null references decks(id),
    winner_wins int not null check (winner_wins > 0 and winner_wins <= 2),
    loser_wins int not null check (loser_wins >= 0 and loser_wins <= 1),
    confirmed boolean not null default false
);

create table disputes (
    id serial primary key,
    -- name is not just 'match' because rust syntax wonkiness
    matchid int not null references matches(id),
    disputer int not null references users(id),
    date timestamp with time zone not null,
    resolved boolean not null default false,
    note text not null
);

create table deck_contents (
    id serial primary key,
    deck int not null references decks(id),
    card uuid not null, -- references card(scryfalloracleid). this field gives a unique uuid to each english name, but due to multiple printings the column is not unique. this constraint is not enforced
    count int not null
);