-- User login sessions: one row per login event.
-- Tracks device fingerprint (UA-derived label + raw UA), IP, geo, login time,
-- the CURRENT access-token jti (rotated on refresh), and the CURRENT refresh-
-- token jti (also rotated, used as the chain key on refresh). Frontend kicks
-- a session by its row `id`; we never expose raw jtis to clients.
--
-- Lookup paths:
--   * by uid              → list a user's active sessions ("我的设备")
--   * by jti_access       → "is this the row matching the current request"
--   * by jti_refresh      → rotate jtis on refresh
--   * by id + uid         → user-initiated revoke
--
-- Cleanup: a scheduler (every hour) prunes rows where expires_at < now AND
-- revoked_at != 0 to keep the table bounded; live unexpired rows stay.

CREATE TABLE IF NOT EXISTS `phpyun_user_session` (
    `id`             BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `uid`            INT UNSIGNED    NOT NULL,
    `usertype`       TINYINT         NOT NULL DEFAULT 1 COMMENT '1=jobseeker 2=employer 3=admin',
    `jti_access`     VARCHAR(64)     NOT NULL              COMMENT 'CURRENT access-token jti (rotates on refresh)',
    `jti_refresh`    VARCHAR(64)     NOT NULL              COMMENT 'CURRENT refresh-token jti (rotates on refresh)',
    `device`         VARCHAR(60)     NOT NULL DEFAULT ''   COMMENT 'human label: iPhone Safari / Chrome Win / WeChat ...',
    `device_raw`     VARCHAR(255)    NOT NULL DEFAULT ''   COMMENT 'raw UA',
    `ip`             VARCHAR(45)     NOT NULL DEFAULT ''   COMMENT 'IPv4 or IPv6',
    `ip_loc`         VARCHAR(80)     NOT NULL DEFAULT ''   COMMENT 'optional geo: 北京 / Shenzhen / ...',
    `login_at`       INT UNSIGNED    NOT NULL              COMMENT 'unix seconds',
    `last_seen_at`   INT UNSIGNED    NOT NULL              COMMENT 'unix seconds; touched on refresh',
    `access_exp`     INT UNSIGNED    NOT NULL              COMMENT 'access-token exp; cap on row liveness',
    `refresh_exp`    INT UNSIGNED    NOT NULL              COMMENT 'refresh-token exp',
    `revoked_at`     INT UNSIGNED    NOT NULL DEFAULT 0    COMMENT '0=active, otherwise unix ts of revocation',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_jti_access`  (`jti_access`),
    UNIQUE KEY `uk_jti_refresh` (`jti_refresh`),
    KEY `ix_uid_revoked_login` (`uid`, `revoked_at`, `login_at`),
    KEY `ix_refresh_exp` (`refresh_exp`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
