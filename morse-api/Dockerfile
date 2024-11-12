FROM rust:latest

RUN adduser --system morse
WORKDIR /opt/morse-api

# Compile only the dependencies (for docker to cache them)
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs && cargo build --release

# Replace the source and build the app
COPY ./src ./src
RUN touch -a -m src/main.rs && cargo build --release

USER morse
CMD ["./target/release/morse-api"]
