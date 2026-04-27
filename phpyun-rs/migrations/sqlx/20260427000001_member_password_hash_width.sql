-- Widen `phpyun_member.password` so it can hold argon2id hashes.
--
-- Background: PHPYun stores `md5(md5(pw)+salt)` (32 bytes). The Rust port uses
-- argon2id, which produces ~95-byte strings of the form
-- `$argon2id$v=19$m=65536,t=3,p=4$...$...`. Without this widening, every new
-- registration / password change fails with `1406 Data too long for column`.
--
-- Widen to 255 to leave headroom for future format upgrades. Existing 32-byte
-- md5 rows continue to verify through the legacy branch in
-- `phpyun_auth::password::verify_password`.

ALTER TABLE `phpyun_member`
    MODIFY COLUMN `password` VARCHAR(255) NOT NULL DEFAULT ''
    COMMENT 'argon2id (new) or legacy md5(md5(pw)+salt) — verified by phpyun_auth';

-- PHPYun's salt is 6 chars; Rust generates 16-char salts via UUIDv7 simple
-- form (see registration_service / password_reset_service). Widen so new
-- registrations & resets stop failing with `1406 Data too long for column 'salt'`.
ALTER TABLE `phpyun_member`
    MODIFY COLUMN `salt` VARCHAR(64) NOT NULL DEFAULT ''
    COMMENT '16-char (Rust) or 6-char (legacy PHP) salt';
