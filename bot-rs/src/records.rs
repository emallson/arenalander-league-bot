use crate::schema::decks;

table! {
    deck_records (id) {
        id -> Int4,
        match_wins -> Int4,
        match_losses -> Int4,
        game_wins -> Int4,
        game_losses -> Int4,
    }
}

joinable!(deck_records -> decks (id));