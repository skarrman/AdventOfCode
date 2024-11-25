FROM rust:latest as builder
WORKDIR /usr/src/day21
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/day21 /usr/local/bin/day21
COPY input.txt .
CMD ["day21"]