-- App version table (Rust-side; PHP install SQL never had it) + 系统最下面「App配置」导航.
-- Apply on jobs (`RUN_MIGRATIONS_ON_BOOT=false`). Do not run on phpyun.

CREATE TABLE IF NOT EXISTS `phpyun_app_version` (
    `id`           INT          NOT NULL AUTO_INCREMENT,
    `platform`     VARCHAR(16)  NOT NULL DEFAULT '' COMMENT 'ios / android / harmony',
    `version`      VARCHAR(32)  NOT NULL DEFAULT '' COMMENT '1.2.3',
    `version_code` INT          NOT NULL DEFAULT 0 COMMENT '10203',
    `is_force`     INT          NOT NULL DEFAULT 0 COMMENT '0=提示 1=强制',
    `download_url` VARCHAR(255) NOT NULL DEFAULT '' COMMENT '下载地址（iOS 可为商店 deeplink）',
    `changelog`    MEDIUMTEXT   NULL COMMENT '更新说明',
    `status`       INT          NOT NULL DEFAULT 1 COMMENT '0=下架 1=生效',
    `released_at`  INT          NOT NULL DEFAULT 0 COMMENT '发布时间（UNIX 秒）',
    `ctime`        INT          NOT NULL DEFAULT 0 COMMENT '创建时间',
    `deleted`      TINYINT(1)   NOT NULL DEFAULT 0 COMMENT '0=active,1=deleted',
    PRIMARY KEY (`id`),
    KEY `idx_platform_status` (`platform`, `status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='移动端 App 版本管理';

-- 二级：系统最下面（信息 sort=9 之后）
INSERT INTO `phpyun_admin_navigation`
    (`id`, `name`, `keyid`, `url`, `menu`, `classname`, `sort`, `display`, `dids`, `level`, `path`)
SELECT 1065, 'App配置', 5, '', 1, 'nav_app', 10, 0, 1, 2, ''
FROM DUAL
WHERE NOT EXISTS (
    SELECT 1 FROM `phpyun_admin_navigation` WHERE `id` = 1065 OR (`keyid` = 5 AND `name` = 'App配置')
);

-- 三级叶子
INSERT INTO `phpyun_admin_navigation`
    (`id`, `name`, `keyid`, `url`, `menu`, `classname`, `sort`, `display`, `dids`, `level`, `path`)
SELECT 1066, 'App配置', 1065, 'index.php?m=system&c=appset', 1, '0', 1, 0, 1, 3, '/appset'
FROM DUAL
WHERE NOT EXISTS (
    SELECT 1 FROM `phpyun_admin_navigation` WHERE `id` = 1066 OR `path` = '/appset'
);

-- 追加超级管理员权限，不整段覆盖 group_power
UPDATE `phpyun_admin_user_group`
SET `group_power` = CONCAT(
    'a:',
    CAST(SUBSTRING_INDEX(SUBSTRING_INDEX(`group_power`, ':', 2), ':', -1) AS UNSIGNED) + 2,
    ':{',
    TRIM(TRAILING '}' FROM SUBSTRING(`group_power`, LOCATE('{', `group_power`) + 1)),
    'i:',
    CAST(SUBSTRING_INDEX(SUBSTRING_INDEX(`group_power`, ':', 2), ':', -1) AS UNSIGNED),
    ';i:1065;i:',
    CAST(SUBSTRING_INDEX(SUBSTRING_INDEX(`group_power`, ':', 2), ':', -1) AS UNSIGNED) + 1,
    ';i:1066;}'
)
WHERE `id` = 1
  AND `group_power` LIKE 'a:%'
  AND `group_power` NOT LIKE '%i:1065;%'
  AND `group_power` NOT LIKE '%s:4:"1065"%';
