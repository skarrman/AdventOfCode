FROM rust:1.56 as builder
WORKDIR /usr/src/day01
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/day01 /usr/local/bin/day01
COPY input.txt .
CMD ["day01"]