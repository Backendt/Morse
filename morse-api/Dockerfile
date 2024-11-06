FROM rust:latest

WORKDIR /opt/morse-api
COPY . .
RUN cargo install --path .

RUN adduser --system morse
USER morse

CMD ["morse-api"]
