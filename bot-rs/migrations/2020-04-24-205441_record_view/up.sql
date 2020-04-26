-- Your SQL goes here
create view deck_records (id, match_wins, match_losses, game_wins, game_losses) as
(
    with
    deck_wins as (select decks.id as id, coalesce(sum(winner_wins), 0) as game_wins, count(matches.id) as match_wins from decks left join matches on decks.id = matches.winning_deck and matches.confirmed group by decks.id),
    deck_losses as (select decks.id as id, coalesce(sum(winner_wins), 0) as game_losses, count(matches.id) as match_losses from decks left join matches on decks.id = matches.losing_deck and matches.confirmed group by decks.id)
    select
    deck_wins.id as id,
    match_wins,
    match_losses,
    game_wins,
    game_losses
    from deck_wins left join deck_losses on deck_wins.id = deck_losses.id
    order by match_wins desc
);
