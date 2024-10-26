FROM rust

RUN useradd -u 1010 -m rusty_psql_user

WORKDIR /usr/src/rusty_psql
COPY . .

RUN cargo install --path .

CMD ["rusty_psql"]
