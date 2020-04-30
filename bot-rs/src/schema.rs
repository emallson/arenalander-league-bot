table! {
    deck_contents (id) {
        id -> Int4,
        deck -> Int4,
        card -> Uuid,
        count -> Int4,
    }
}

table! {
    deck_view_tokens (id) {
        id -> Int4,
        deck -> Int4,
        token -> Uuid,
    }
}

table! {
    decks (id) {
        id -> Int4,
        league -> Nullable<Int4>,
        owner -> Int4,
        creation_date -> Timestamptz,
        resigned -> Bool,
        active -> Bool,
    }
}

table! {
    disputes (id) {
        id -> Int4,
        matchid -> Int4,
        disputer -> Int4,
        date -> Timestamptz,
        resolved -> Bool,
        note -> Text,
    }
}

table! {
    leagues (id) {
        id -> Int4,
        title -> Text,
        start_date -> Timestamptz,
        end_date -> Timestamptz,
    }
}

table! {
    matches (id) {
        id -> Int4,
        date -> Timestamptz,
        winning_deck -> Int4,
        losing_deck -> Int4,
        winner_wins -> Int4,
        loser_wins -> Int4,
        confirmed -> Bool,
    }
}

table! {
    users (id) {
        id -> Int4,
        discordid -> Int8,
        name -> Text,
    }
}

table! {
    deck_records (id) {
        id -> Int4,
        match_wins -> Int8,
        match_losses -> Int8,
        game_wins -> Int8,
        game_losses -> Int8,
    }
}

table! {
    leaderboard (id) {
        id -> Int4,
        name -> Text,
        league -> Int4,
        wins -> Int8,
        complete_runs -> Int8,
    }
}

table! {
    cards (id) {
        id -> Int8,
        name -> Text,
        number -> Text,
        setcode -> Text,
        isarena -> Int8,
        scryfalloracleid -> Uuid,
        manacost -> Nullable<Text>,
        types -> Text,
        convertedmanacost -> Float8,
    }
}

joinable!(deck_records -> decks (id));
joinable!(leaderboard -> users (id));
joinable!(leaderboard -> leagues (league));
joinable!(deck_contents -> decks (deck));
joinable!(deck_view_tokens -> decks (deck));
joinable!(decks -> leagues (league));
joinable!(decks -> users (owner));
joinable!(disputes -> matches (matchid));
joinable!(disputes -> users (disputer));

allow_tables_to_appear_in_same_query!(
    deck_contents,
    deck_view_tokens,
    decks,
    disputes,
    leagues,
    matches,
    users,
    cards,
    deck_records,
    leaderboard,
);
