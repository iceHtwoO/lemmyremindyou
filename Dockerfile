FROM rust as builder

RUN cargo install diesel_cli --no-default-features --features postgres

COPY ./ ./

RUN cargo build --release

FROM debian:bullseye-slim

COPY --from=builder /target/release/lemmyremindyou .

RUN apt update
RUN apt install -y libpq-dev openssl 

ENTRYPOINT ["/bin/bash", "-c", "./lemmyremindyou"]