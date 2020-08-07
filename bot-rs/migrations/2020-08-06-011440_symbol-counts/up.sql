alter table decks
      add column symbols_w smallint not null default 0,
      add column symbols_u smallint not null default 0,
      add column symbols_b smallint not null default 0,
      add column symbols_r smallint not null default 0,
      add column symbols_g smallint not null default 0;

with symbol_counts as (
     select distinct on (scryfalloracleid) scryfalloracleid,
     (length(manacost) - length(replace(manacost, '{W}', ''))) / 3 as symbols_w,
     (length(manacost) - length(replace(manacost, '{U}', ''))) / 3 as symbols_u,
     (length(manacost) - length(replace(manacost, '{B}', ''))) / 3 as symbols_b,
     (length(manacost) - length(replace(manacost, '{R}', ''))) / 3 as symbols_r,
     (length(manacost) - length(replace(manacost, '{G}', ''))) / 3 as symbols_g
     from cards
     where side = 'a' or side is null
), deck_counts as (
   select deck,
   sum(symbols_w) as total_w,
   sum(symbols_u) as total_u,
   sum(symbols_b) as total_b,
   sum(symbols_r) as total_r,
   sum(symbols_g) as total_g
   from deck_contents
   left join symbol_counts on symbol_counts.scryfalloracleid = deck_contents.card
   group by deck
)
update decks
    set symbols_w = total_w,
        symbols_u = total_u,
        symbols_b = total_b,
        symbols_r = total_r,
        symbols_g = total_g
    from deck_counts
    where decks.id = deck;
