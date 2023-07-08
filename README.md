# Installation
```bash
git clone https://github.com/Zuruuh/rust-rocket-blog.git
cd rust-rocket-blog
cp .env.example .env
cargo install sqlx-cli
sqlx migrate up
cargo build
```

# Run
```bash
docker compose up -d
cargo run
```

```prod

```
