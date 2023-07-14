FROM rust as builder

RUN cargo install diesel_cli --no-default-features --features postgres

COPY ./ ./

RUN cargo build --release

RUN apt update
RUN apt install -y libpq-dev openssl 

CMD ["bash", "-c", "diesel migration run"]
ENTRYPOINT ["/bin/bash", "-c", "./target/release/lemmyremindyou"]