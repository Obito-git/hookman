# Project Overview

The main goal of this project is to learn and experiment with different technologies and approaches.

## Inspiration

This idea came to me unexpectedly while I was working on a Jira ticket at my job. I needed to test a fix, so I used
Postman to send a request to my application. That request triggered a webhook, which I was testing
with [webhook.site](https://webhook.site). Then I thought—it would be fun to build a similar tool myself using Rust and
Tokio.

If the project becomes good enough, it could be useful for testing, debugging, monitoring, CI/CD pipelines, and more.
But my main goal is to learn new things and have fun while building it.

## Main Ideas

- **Use a structured design pattern**  
  I want to try **hexagonal architecture** because it seems like a good fit. The core domain will be simple, but I can
  experiment with different adapters.

- **Support multiple interfaces**  
  I plan to create **a web client, a local web client, and a desktop client**. Since I’m not very good at frontend
  development, I'll use **Svelte** for the web client and **Tauri** for the desktop client, so I can reuse the same
  code.
    - The **desktop and local web versions** will have all features.
    - The **web version** will be limited for some features and require registration to use them.

- **Core features**
    - Accept and parse incoming HTTP requests
    - Notify clients about new requests in real time in the UI
    - Support custom responses, including running JavaScript or Python scripts
    - Store request data in a database
    - Provide a REST API for retrieving requests and responses
    - Allow as much configuration as possible

## Future Features

Once the basic version is working, I may add:

- An **email server**
- A **gRPC server**
- More request processing actions, like forwarding requests, exporting data, etc.

This project is mostly for learning, but if it turns out well, it could become a useful tool.

## SeaORM adapter

### Install sqlx-cli and sea-orm-cli

```shell
cargo install sqlx-cli
cargo install sea-entity-cli
```

### Manual migration

Migration is automatic on startup, but it can be done manually.

#### SQLite file

```shell
export DATABASE_URL="sqlite:hookman.db"
cargo run --bin migration
```

#### PostgreSQL

```shell
export DATABASE_URL="postgres://postgres:postgres@localhost:5432/postgres"
cargo run --bin migration
```

### Generate entities files

Connects to `DATABASE_URL` from env, and generates entities from DB. DATABASE_URL can be postgres or sqlite file.

```shell
export DATABASE_URL="postgres://postgres:postgres@localhost:5432/postgres"
sea-orm-cli generate entity -o crates/secondary_adapter_persistence_seaorm/src/entity
```

### Create new migration

```shell
sea-orm-cli migrate generate "my_migration_name" --migration-dir crates/migration 
```

## Commands to not forget

### Run REST API version with Postgres

```shell
export RUST_LOG=INFO; cargo run -p app_back -- postgres --user postgres --password postgres --database postgres --host localhost --port 5432
```

### Run REST API version with SQLite file

```shell
export RUST_LOG=INFO; cargo run -p app_back -- sq-lite-file --database-name hookman --folder-path .
```

### Tauri desktop version

Install dependencies for frontend

```shell
pnpm install
```

Run frontend server

```shell
pnpm tauri dev
```

Run desktop app

```shell
RUST_LOG=INFO; cargo run --bin hookman --no-default-features
```

## Frontend
### Icons framework
https://github.com/unplugin/unplugin-icons
https://icones.js.org/
https://icones.js.org/collection/mingcute

### Modals
https://flowbite-svelte.com/docs/components/modal
example:
https://github.com/PurelyAnecdotal/gradevue/tree/dc485a5a89070ad17de79e9a8c62bf162e71d81e

## Sources for future

- How to embed svelte in rust binary https://fdeantoni.medium.com/single-rust-binary-with-vite-svelte-66944f9ac561
- Swagger for APIs https://medium.com/@nunocarvalhodossantos/generate-and-serve-swagger-in-rust-a7be97aeabfb
- embed JS runtime https://github.com/boa-dev/boa

## TODO

- [ ] Apply no unwrap policy
- [ ] add rules that check that nothing in adapters is pub except struct than implement adapter itself