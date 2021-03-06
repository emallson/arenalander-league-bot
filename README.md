Discord bot for [Gladiator][discord] to run
leagues and display results.

## Setting Up

This codebase uses [Docker Compose](https://docs.docker.com/compose/) as the
primary build system and can be run on any system with that installed:

    docker-compose build

It is often helpful to be able to run things (like `cargo check` or `cargo
test`) outside of docker. This project is implemented in (mostly)
[Rust](https://rustup.rs), so installing that---while not required---is
encouraged.

### Environment Variables

The bot uses 3 environment variables during its run, only one of which
is required:

- `DATABASE_URL` (required): The connection string for the (PostgreSQL) database
- `DISCORD_TOKEN`: The access token for the discord bot. See their API docs for details.
- `SENTRY_TOKEN`: The access token for [sentry](https://sentry.io), which is used for logging in production.
- `PREFIX`: The command prefix used by the bot. Defaults to `!`.

For development, you'll want to copy the [example `.env`
file](bot-rs/.env.example) to `bot-rs/.env`. The example value provided uses
the default login credentials for the database. These credentials are obviously
NOT SECURE and should be changed if you are running outside of a local
development machine.

If `DISCORD_TOKEN` is unset, the only thing you'll be able to do is work with
existing results via the site (which is accessible at `localhost:3000` by
default)

### Database Initialization

The base docker images should set up *almost* everything. After successfully building, you will need to run:

```bash
docker-compose up -d db                                        # start the DB first so postgres has time to start
docker-compose run botrs diesel database setup --locked-schema # setup the DB
```

Changes to the schema should be committed to version control using `diesel migration` commands. For example:

```bash
diesel migration generate example # generation migration named "example"
# edit your migration
diesel migration run # run the migration
diesel migration redo # revert and re-run the migration as necessary to get it right
```

Some test data is included in `db-setup/test_data.sql`. If you have
`psql` installed, you can import it with:

```bash
psql $DATABASE_URL < db-setup/test_data.sql
```

### Running Tests

The full test suite requires the database to be running and the `DATABASE_URL` environment variable to be set.

```bash
docker-compose up db -d             # start the DB in the background
export DATABASE_URL=....            # set the DB connection info
cargo test                          # run the tests
docker-compose run botrs cargo test # alternative to cargo test if you didn't install rust locally
```

## Deploying

Contact `emallson` via the [Gladiator Discord][discord] if you are
interested in deploying your own league bot.

[discord]: https://discord.gg/ncpJVm5
