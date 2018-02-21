FROM rust:1.24

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install

CMD ["myapp"]
