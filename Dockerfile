FROM rust:1.73 as builder

WORKDIR /usr/src/todo-web
# https://stackoverflow.com/a/58474618/9641551
RUN echo "fn main() {}" > dummy.rs
COPY Cargo.toml .
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN cargo build --release
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml
COPY . .
RUN cargo install --path .


FROM rust:1.73
WORKDIR /usr/src/todo-web
COPY --from=builder /usr/local/cargo/bin/todo-web /usr/local/bin/todo-web
COPY --from=builder /usr/src/todo-web/public public
EXPOSE 8080
CMD ["todo-web"]
