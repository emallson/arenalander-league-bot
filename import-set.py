#!/usr/bin/env python3
"""Import a set into the League Bot's card database.

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

data = requests.get(f'https://mtgjson.com/api/v5/{code}.json').json()


def card_to_rows(card):
    """Return two rowsets for the given card:

    1. the row for the `cards` table
    2. the rowset (typically multiple rows) for the `card_names` table"""
    row = {col: card.get(col, None) for col, obj in cards.c.items()}
    del row['id']

    try:
        row['isarena'] = 'arena' in card['availability']
        row['scryfallid'] = card['identifiers']['scryfallId']
        row['scryfalloracleid'] = card['identifiers']['scryfallOracleId']
        row['setcode'] = code
        row['convertedmanacost'] = card['convertedManaCost']
        row['manacost'] = card.get('manaCost', None)
        row['types'] = ','.join(card['types'])
    except Exception as e:
        print(card)
        raise e

    alts = [{'uuid': row['uuid'],
             'scryfalloracleid': row['scryfalloracleid'],
             'language': alt['language'],
             'name': alt['name']}
            for alt in card['foreignData']]
    return row, alts


with engine.begin() as conn:
    new_cards = [card_to_rows(card) for card in data['data']['cards']]
    query = insert(cards).on_conflict_do_nothing()
    # print(query)
    res = conn.execute(query, [row for (row, _) in new_cards])
    print(f'{res.rowcount} new cards inserted ({len(new_cards)} in set)')

    names = [row for (_, rows) in new_cards for row in rows]
    if len(names) > 0:
        res = conn.execute(insert(card_names).on_conflict_do_nothing(), names)
        print(f'{res.rowcount} new card names inserted')
