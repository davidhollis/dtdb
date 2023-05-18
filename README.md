# dtdb
A potential future replacement for imdt: the archive of past shows put on by DramaTech Theatre in Atlanta, GA

## Development

### Reference Material

The backend for this site is written primarily in rust. It uses:

  * [Diesel](https://diesel.rs/guides/) as a database abstraction layer

### Environment Setup

First, get postgres running and accessible from your development environment in whatever manner works best for you. Note the connection details, then

```bash
echo "DATABASE_URL=postgres://user:pass@localhost:5432/dtdb_dev" >.env
```

where the credentials, host, and port are appropriate for your setup. The database name `dtdb_dev` is recommended, but you can name the database whatever you want. If it doesn't exist yet, it'll be created later as long as the user has permission to create a database.

<details>
  <summary>Example postgres setup with docker</summary>

  ```bash
  docker pull postgres
  docker run \
    --name dtdb-postgres \
    -e POSTGRES_USER=dtdb \
    -e POSTGRES_PASSWORD=dtdbpassword \
    -e POSTGRES_DB=dtdb_dev \
    -d -p 5437:5432 \
    postgres
  echo "DATABASE_URL=postgres://dtdb:dtdbpassword@localhost:5437/dtdb_dev" >.env
  ```
</details>

Next, make sure you have rust installed, then run the following to migrate the DB:

```bash
# Install the CLI for the database abstraction library
cargo install diesel_cli --no-default-features --features postgres

# Create the database if it doesn't exist and run all the migrations
diesel setup
```

Finally, make sure everything's working:

```bash
cargo build
cargo test
```

And you should be good to go!