use super::schema::*;

table! {
    cards (id) {
        id -> Int8,
        name -> Text,
        number -> Text,
        setcode -> Text,
        isarena -> Int8,
    }
}

joinable!(deck_contents -> cards (card));