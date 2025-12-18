FROM rust:1.83.0-bookworm AS build
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*
COPY --from=build /app/target/release/actix-app /app/server
EXPOSE 8080
CMD ["/app/server"]

# docker build -t rust-actix-app:slim .
# docker run --rm -it -p8080:8080 rust-actix-app:slim
