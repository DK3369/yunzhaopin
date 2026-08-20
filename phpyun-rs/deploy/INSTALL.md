# PHPYun Rust production package

This archive is a deployment input only. It does not install files, run SQL,
or restart a service.

## Install

1. Extract the archive and verify `SHA256SUMS` with `sha256sum -c SHA256SUMS`.
2. Copy the extracted directory to the chosen installation directory. The
   `phpyun-rs` executable is at the archive root.
3. Copy `.env.pro.example` to the production configuration location as
   `.env.pro`, replace every `CHANGE_ME`, set mode `0640` with ownership
   `root:www`, and rotate any credential that has ever appeared in a repository
   or backup file.
4. Replace `@@INSTALL_DIR@@` and `@@ENV_FILE@@` in
   `systemd/phpyun-rs.service`, then install the rendered unit under
   `/etc/systemd/system/`.
5. Review and apply `sqlx/` through the release pipeline before
   starting the new binary. Production defaults to
   `RUN_MIGRATIONS_ON_BOOT=false`.
6. Run `systemctl daemon-reload`, start or restart the service, and check
   `/health`, `/ready`, application logs, and Prometheus metrics.

The unit defaults to `User=www` and `Group=www`. Change both only when the
target host uses a dedicated service account, and ensure that account can write
to the configured storage directory.

## Rollback

Keep the previous extracted release intact. Point the rendered service unit (or
an installation symlink managed outside this package) back to the previous
directory, reload systemd, restart, and re-check `/health` and `/ready`.
Database rollback must follow the migration-specific operational procedure;
this package never reverses migrations automatically.
