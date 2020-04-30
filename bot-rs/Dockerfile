FROM rust
RUN apt update && apt install libpq5
RUN cargo install diesel_cli --no-default-features --features postgres
WORKDIR /opt/bot
COPY dummy.rs .
COPY Cargo.toml .
COPY Cargo.lock .
RUN cargo build --release --bin dummy --locked
COPY src src
COPY templates templates
RUN cargo build --release --bin bot --locked
COPY resources resources
COPY migrations migrations 
CMD cargo run --release --bin bot --locked
