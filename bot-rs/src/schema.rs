table! {
    card_names (id) {
        id -> Int4,
        name -> Text,
    }
}

table! {
    deck_contents (id) {
        id -> Int4,
        deck -> Nullable<Int4>,
        card -> Nullable<Int4>,
        count -> Int4,
    }
}

table! {
    decks (id) {
        id -> Int4,
        league -> Int4,
        owner -> Int4,
        creation_date -> Timestamptz,
        wins -> Int4,
        losses -> Int4,
        resigned -> Bool,
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
    users (id) {
        id -> Int4,
        discordid -> Int8,
    }
}

joinable!(deck_contents -> card_names (card));
joinable!(deck_contents -> decks (deck));
joinable!(decks -> leagues (league));
joinable!(decks -> users (owner));

allow_tables_to_appear_in_same_query!(
    card_names,
    deck_contents,
    decks,
    leagues,
    users,
);
