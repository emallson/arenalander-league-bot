FROM rust
RUN apt update && apt install libpq5
RUN cargo install diesel_cli --no-default-features --features postgres
CMD cd /opt/bot && cargo run --release