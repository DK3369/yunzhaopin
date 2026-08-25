# test-jobs.ov6.com routing

## Topology

`test-jobs.ov6.com` terminates TLS at Cloudflare and reaches the remotely
managed Cloudflare Tunnel. The tunnel origin is `http://127.0.0.1:80`, where
Nginx is the only public application gateway.

Nginx routes normal website paths to the PHPYun document root and PHP-FPM.
Rust remains private on `127.0.0.1:3000` and is reached through Nginx.

The Rust process is managed by `test-jobs-phpyun-rs.service`. Its executable
is installed at `/opt/phpyun-rs/phpyun-rs`, it runs as the existing `aa` service
user, and it loads `phpyun-rs/.env`. The service is enabled at boot and uses
automatic restart on failure. Rust must bind only to `127.0.0.1:3000`; public
traffic always enters through Nginx. Prometheus metrics must bind only to
`127.0.0.1:9090`.

## Public routes

- `/`, `/wap/`, `/admin/`, and other PHPYun paths are handled by PHP/Nginx.
- `/yapi/*` is the canonical prefixed Rust API route. Nginx strips `/yapi`
  before proxying, so `/yapi/v1/wap/home` becomes `/v1/wap/home` in Rust.
- `/v1/*` and `/v2/*` are compatibility routes that preserve the request URI
  when proxying to Rust.
- `/health` and `/ready` are compatibility probe routes forwarded to Rust.
- `/yapi/dev/token` and `/dev/token` are never public and return 404 even when
  the retained test binary runs with its development runtime mode.

The compatibility routes prevent clients that used the old Tunnel-to-Rust
topology from breaking after the Tunnel origin moves from port 3000 to Nginx
on port 80.

## Runtime domain

PHPYun's database setting and generated `data/plus/config.php` value for
`sy_weburl` must both be `https://test-jobs.ov6.com`. Regenerate the runtime
configuration after changing the database setting; do not commit credentials
or print the complete generated configuration during diagnostics.

## Validation

Validate both locally with `Host: test-jobs.ov6.com` and through Cloudflare:

- `GET /` returns 200 and rendered asset URLs use the public HTTPS domain.
- `GET /wap/` does not redirect to `dev.test`.
- A desktop request to `/wap/` may follow the configured PC/WAP policy, but
  must never redirect back to the same `/wap/` URL. With the current policy it
  redirects once to the public PC homepage; a mobile request renders WAP.
- `GET /admin/` returns 200.
- `GET /yapi/ready` and `GET /ready` return healthy DB/Redis state.
- `POST /yapi/v1/wap/home` and `POST /v1/wap/home` return the same Rust API
  response status.
- `systemctl is-enabled test-jobs-phpyun-rs` and
  `systemctl is-active test-jobs-phpyun-rs` both succeed.
