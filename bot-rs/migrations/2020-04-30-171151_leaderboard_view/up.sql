-- Your SQL goes here
create view leaderboard (id, league, name, wins, complete_runs) as (
    select 
        users.id as id,
        decks.league as league,
        users.name as name,
        SUM(match_wins)::bigint as wins,
        count(case when not active and not resigned then 1 end) as complete_runs
    from users
    left join decks on users.id = decks.owner
    left join deck_records on deck_records.id = decks.id
    group by users.id, decks.league
);