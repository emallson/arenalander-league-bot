# Bot Admin Notes

The goal of these notes is twofold. First: to have a quick reference for what
you can do with the admin commands and how. Second: what things cannot be done
via commands and instead need to be done by the person running the bot (aka
`emallson#6223`).

## Create a New League

The `?admin new_league` command is used to create a new league. It has an odd
format compared to other commands. Specifically:

```
?admin new_league <title>, <start-date>, <end-date>
```

Note the commas. They are important. For example:

```
?admin new_league July League, 16 July 2020, 1 August 2020
```

The `start-date` and `end-date` refer to **midnight UTC** on that date. So if
you want a league to end on July 31st (for example), then the `end-date` needs
to be *August 1st* (the next day).

Only one league can be running at a time. Creating a new league with overlapping
start/end dates will *probably* do weird things.

## Delete a League

```
?admin delete_league <id>
```

You can get the league ID with `?admin list_leagues`. This will remove the
league with that ID, but *not* the decks associated with it. The decks will no
longer be associated with any league, but all the deck links will still work.


## End a League

A league ends automatically when its `end-date` is reached. However, decks with
fewer than 5 matches (that have not been resigned) will not be publically
visible until you run this command:

```
?admin finalize_league <id>
```

This marks all league decks as *inactive* (meaning: they are publically
visible). This is *not* the same as resigning, and they won't show as resigned
on the league standings.

## Ban a Card

Banning a card requires modifying the `BANNED_CARDS` list in the code and then
redeploying. This needs to be done by `emallson`.

## Add New Cards

When a new set releases (including Historic Anthologies, or special Brawl
cards), `emallson` needs to run a script to import the new cards.

## Ban a User

The bot currently has no provisions for this. If it becomes necessary, contact
`emallson` and we'll get something figured out.
