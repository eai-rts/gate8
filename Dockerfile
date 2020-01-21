FROM rust:1.40 as builder

WORKDIR /usr/src/myapp

COPY . .

RUN cargo install --path .

EXPOSE 8080
