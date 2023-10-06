# todo-web

- Simple Todo list CRUD app to experiment with `rust` backend development
- Uses [actix](https://actix.rs/) and [sqlx](https://github.com/launchbadge/sqlx)

## Running the app

To setup the database:

```bash
cargo install sqlx-cli # if not installed
sqlx database drop
sqlx database create
sqlx migrate run
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