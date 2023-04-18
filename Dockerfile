FROM rust:1.68.1 as builder

WORKDIR /var/www
COPY . /var/www

# cargo build rust
RUN cargo build --release --bin server

FROM rust:slim-bullseye as runtime

RUN apt-get update && apt-get install -y libssl1.1 ca-certificates

COPY --from=builder /var/www/target/release/server /usr/local/bin/server

RUN groupadd -r user && useradd -r -g user user
RUN chown -R user:user /usr/local/bin/server

RUN mkdir -p /var/www/storage/temp && mkdir -p /var/www/storage/logs && chown -R user:user /var/www/storage

USER user

CMD ["/usr/local/bin/server"]

EXPOSE 9000
