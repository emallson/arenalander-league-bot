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

code = args['<set-code>']

data = requests.get(f'https://mtgjson.com/api/v5/{code}.json').json()

def card_to_row(card):
    row = {col: card.get(col, None) for col, obj in cards.c.items()}
    del row['id']
    
    row['isarena'] = 'arena' in card['availability']
    row['scryfallid'] = card['identifiers']['scryfallId']
    row['scryfalloracleid'] = card['identifiers']['scryfallOracleId']
    row['setcode'] = code
    return row

with engine.begin() as conn:
    new_cards = [card_to_row(card) for card in data['data']['cards']]
    query = insert(cards).on_conflict_do_nothing()
    # print(query)
    res = conn.execute(query, new_cards)
    print(f'{len(new_cards)} cards inserted')
