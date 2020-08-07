alter table decks
      drop column if exists symbols_w,
      drop column if exists symbols_u,
      drop column if exists symbols_b,
      drop column if exists symbols_r,
      drop column if exists symbols_g;
