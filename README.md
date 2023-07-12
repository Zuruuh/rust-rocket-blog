# Installation
```bash
git clone https://github.com/Zuruuh/rust-rocket-blog.git
cd rust-rocket-blog
cp .env.example .env
cargo install diesel_cli
diesel migration run
cargo build
```

# Run
```bash
docker compose up -d
cargo run
```

```bash
curlie http://localhost:8000/blog
```

<img src="https://cdn.discordapp.com/attachments/785207461379833859/1128777267896664185/image.png" />
