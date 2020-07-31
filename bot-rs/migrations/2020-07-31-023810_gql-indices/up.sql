create index deck_contents_deck on deck_contents(deck);
create index matches_winner on matches(winning_deck);
create index matches_loser on matches(losing_deck);
create index deck_league on decks(league);
create index deck_contents_card on deck_contents(card);
