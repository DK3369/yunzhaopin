-- Admin menu for seeker monthly VIP. RUN_MIGRATIONS_ON_BOOT=false: apply on `jobs` by hand.
INSERT INTO `phpyun_admin_navigation`
    (`id`, `name`, `keyid`, `url`, `menu`, `classname`, `sort`, `display`, `dids`, `level`, `path`)
SELECT 1067, '求职包月', 7, '', 1, '0', 7, 0, 1, 3, '/seekerVip'
FROM DUAL
WHERE NOT EXISTS (
    SELECT 1 FROM `phpyun_admin_navigation` WHERE `id` = 1067 OR `path` = '/seekerVip'
);
