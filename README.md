# SUS - Selfhosted URL Shortener
Simple, selfhosted URL shortener written in Rust, that uses PostgreSQL

## How to set it up
0. Install and run PostgreSQL
1. [Install the Rust toolchain](https://rustup.rs)
2. Clone the repository, and open it
3. Create `.env` file and put the PostgreSQL credentials in it, in the following way:
```sh
DATABASE_URL="postgres://username:password@localhost/database_name"
```

4. Open the Terminal in the cloned repo and run the following commands:
```sh
cargo install sqlx-cli
sqlx migrate run
```

5. Run this command to compile and run the program:
```sh
cargo run
```
