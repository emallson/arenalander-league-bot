#!/usr/bin/env python3
"""Import a set into the League Bot's card database directly from scryfall.

This is always run as __main__. DO NOT IMPORT

Usage:
    import-set.py <set-code> [--database-url <url>]
    import-set.py (-h | --help)

Options:
    -h --help              Show this screen.
    --database-url <url>   Set the database URL. Defaults to $DATABASE_URL.
"""

import requests
from docopt import docopt
from sqlalchemy import create_engine, Table, MetaData
from sqlalchemy.dialects.postgresql import insert
from os import environ

args = docopt(__doc__)

engine = create_engine(args['--database-url'] or environ['DATABASE_URL'])
meta = MetaData()

cards = Table('cards', meta, autoload=True, autoload_with=engine)
card_names = Table('card_names', meta, autoload=True, autoload_with=engine)

code = args['<set-code>']


def load_cards(code):
    res = requests.get(f'https://api.scryfall.com/cards/search?q=set:{code}&include_multilingual=true').json()

    data = res['data']

    while res['has_more']:
        res = requests.get(res['next_page']).json()
        data += res['data']

    return data


data = load_cards(code)


def card_types(card):
    types = card['type_line'].split(' — ')[0]
    return types.replace(' ', ',')


def card_to_rows(card):
    """Return two rowsets for the given card:

    1. the row for the `cards` table
    2. the rowset (typically multiple rows) for the `card_names` table"""

    row = {
        'name': card['name'],
        'setcode': card['set'],
        'number': card['collector_number'],
        'isarena': 'arena' in card['games'],
        'scryfallid': card['id'],
        'scryfalloracleid': card['oracle_id'],
        'manacost': card['mana_cost'] if 'mana_cost' in card else None,
        'types': card_types(card),
        'convertedmanacost': card['cmc'],
        # when using mtgjson, this is the mtgjson uuid---but we obvs don't have that here
        'uuid': card['id'],
    }

    alts = []
    return row, alts


with engine.begin() as conn:
    new_cards = [card_to_rows(card) for card in data]
    query = insert(cards).on_conflict_do_nothing()
    # print(query)
    res = conn.execute(query, [row for (row, _) in new_cards])
    print(f'{res.rowcount} new cards inserted ({len(new_cards)} in set)')

    names = [row for (_, rows) in new_cards for row in rows]
    if len(names) > 0:
        res = conn.execute(insert(card_names).on_conflict_do_nothing(), names)
        print(f'{res.rowcount} new card names inserted')
