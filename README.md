# [todo-web](https://todo-web-rust.fly.dev/)

- Simple Todo list CRUD app to experiment with `rust` backend development
- Uses [actix](https://actix.rs/), [turso](https://turso.tech/) and [fly.io](https://fly.io/)

## Running the app

To setup the database, use [turso](https://turso.tech/) and get the following credentials in a `.env` file at the root.

```bash
LIBSQL_CLIENT_URL=
LIBSQL_CLIENT_TOKEN=
```

Use the `turso` CLI and run the following sql initialization script:

```sql
CREATE TABLE IF NOT EXISTS tasks (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  completed INTEGER NOT NULL DEFAULT 0
)
```

- To start the app, run:

```bash
cargo watch -x run
```

- To run a test script on the running server using `curl` and `jq`, run the following. Note that it will reset the database!

```bash
chmod +x try.sh
./try.sh
```

- Navigate to `http://localhost:8080/` to interact with the UI


- To run the app inside docker:

```bash
docker build -t todo-web .
docker run -p  8080:8080 --env-file ./.env -it --rm todo-web
```

- To run it on [fly.io](fly.io), setup `.env` file contents as `fly secrets`
