# BackEnd Project - Use Actix and SurrealDB to build a RESTful API

[![Actix](https://img.shields.io/badge/Actix-4.0.0-blue.svg)](https://actix.rs/)
[![SurrealDB](https://img.shields.io/badge/SurrealDB-1.0.0-blue.svg)](https://surrealdb.com)

## Project Description

This project is a RESTful API that uses Actix and SurrealDB to create a simple CRUD API.
The API will be used to create, read, update, and delete data from a database.

## Project Dependencies

```toml
[dependencies]
actix-cors = "0.6.4"
actix-web-lab = "0.19.1"
argon2 = "0.5.1"
anyhow = "1.0.71"
async-stream = "0.3.3"
async-trait = "0.1.68"
actix-governor = { git = "https://github.com/AaronErhardt/actix-governor", features = ["logger"] }
actix-identity = "0.5.2"
actix-session = { version = "0.7.2", features = ["cookie-session"] } 
actix-session-surrealdb = "0.1.3"
actix-web = { version = "4.2.1", features = ["rustls"] }
chrono = "0.4.24"
dotenv = "0.15.0"
env_logger = "0.10.0"
futures = "0.3.25"
log = "0.4.17"
rand_core = { version = "0.6.4", features = ["std"] }
reqwest =  { version = "0.11.14", features = ["json"] }
rustls = "0.21.6"
rustls-pemfile = "1"
serde = { version = "1.0.152", features = ["derive"] }
serde_json = "1.0.91"
surrealdb = "1.0.0-beta.9"
thiserror = "1.0.38"
tokio = { version = "1.28.2", features = ["fs"]}
uuid = { version = "1.3.3", features = ["v4"] }
```

## Command

### Start SurrealDB

```bash
surreal start --user ${username} --password ${password}
```

#### Arguments

- `--user` - SurrealDB username
- `--password` - SurrealDB password