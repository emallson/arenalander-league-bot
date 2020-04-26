FROM rust
RUN apt update && apt install libpq5
RUN cargo install diesel_cli --no-default-features --features postgres
WORKDIR /opt/bot
CMD cargo build --release --bin dummy
CMD cargo run --release --bin bot