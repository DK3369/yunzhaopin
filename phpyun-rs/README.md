# phpyun-rs

A Rust rewrite of the PHPYun backend. See the parent [PROJECT_PLAN.md](../PROJECT_PLAN.md) for the detailed project plan, and [WAP_API_SPEC.md](../WAP_API_SPEC.md) for the API specification.

## Quick Start

```bash
# 1. Copy the environment variables file
cp .env.example .env
# Edit .env and fill in the actual DATABASE_URL / JWT_SECRET

# 2. Start dependencies (if MySQL/Redis are not available locally)
# You can use the host machine's aapanel MySQL directly
# Redis: apt install redis-server, or run via docker

# 3. Build and run
cargo run -p phpyun-app

# 4. Test
curl -i http://localhost:3000/health
# -> {"ok":true}

curl -i http://localhost:3000/ready
# -> {"db":true}

# 5. Login example (requires a matching user in the database)
curl -X POST http://localhost:3000/wap/login \
  -d "username=testuser&password=123456"
```

## Workspace

```
crates/
  core/      # Config / Error / State / Response / Telemetry
  auth/      # JWT + argon2 + md5 compatibility
  models/    # DB entities + repositories (per table)
  services/  # Business logic (cross-model)
  handlers/  # HTTP handlers
  app/       # Main binary
```

## Milestones

- **M0** — Bootstrap preparation (current scaffolding) [done]
- **M1** — Core infrastructure (DB / Redis / rate limiting / CORS / metrics)
- **M2** — Authentication + user system (login / register / password recovery / captcha) <- next up
- M3 — Public content endpoints
- M4 — OAuth
- M5 ~ M8 — Business modules & member center
- M9 — Nginx routing & dual-stack compatibility
- M10 — Load testing / production launch

## Development Commands

```bash
cargo fmt                    # Format
cargo clippy -- -D warnings  # Static analysis
cargo test                   # Unit tests
cargo run -p phpyun-app      # Run

# Prepare sqlx offline data (run once after connecting to a real DB)
cargo install sqlx-cli --no-default-features --features mysql
cargo sqlx prepare --workspace
```

## Contribution Guidelines

- Code must pass `cargo fmt` + `cargo clippy -- -D warnings`
- PRs must pass `cargo test`
- New endpoints must be paired with updates to `WAP_API_SPEC.md`
- Commit messages should follow [Conventional Commits](https://www.conventionalcommits.org/)

## Production Deployment (Reference)

See PROJECT_PLAN.md §9 for the deployment strategy. At this stage, systemd is sufficient:

```bash
cargo build --release -p phpyun-app
sudo cp target/release/app /usr/local/bin/phpyun-rs
sudo cp deploy/systemd/phpyun-rs.service /etc/systemd/system/
sudo systemctl enable --now phpyun-rs
```
