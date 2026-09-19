-- Payment nav is 3-level like 运营→财务→充值订单. Two-level 1070→pages
-- made the sidebar empty folders (layout only renders grandchildren as links).
-- Apply on jobs (`RUN_MIGRATIONS_ON_BOOT=false`).

INSERT INTO `phpyun_admin_navigation`
    (`id`, `name`, `keyid`, `url`, `menu`, `classname`, `sort`, `display`, `dids`, `level`, `path`)
SELECT 1074, '网关', 1070, '', 1, '0', 1, 0, 1, 2, ''
FROM DUAL
WHERE NOT EXISTS (
    SELECT 1 FROM `phpyun_admin_navigation` WHERE `id` = 1074
);

UPDATE `phpyun_admin_navigation`
SET `keyid` = 1074, `level` = 3
WHERE `id` IN (1071, 1072, 1073)
  AND `keyid` = 1070;

UPDATE `phpyun_admin_user_group`
SET `group_power` = CONCAT(
    'a:',
    CAST(SUBSTRING_INDEX(SUBSTRING_INDEX(`group_power`, ':', 2), ':', -1) AS UNSIGNED) + 1,
    ':{',
    TRIM(TRAILING '}' FROM SUBSTRING(`group_power`, LOCATE('{', `group_power`) + 1)),
    'i:',
    CAST(SUBSTRING_INDEX(SUBSTRING_INDEX(`group_power`, ':', 2), ':', -1) AS UNSIGNED),
    ';i:1074;}'
)
WHERE `id` = 1
  AND `group_power` LIKE 'a:%'
  AND `group_power` LIKE '%i:1070;%'
  AND `group_power` NOT LIKE '%i:1074;%';
