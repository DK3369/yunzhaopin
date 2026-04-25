-- Dictionary translation table: one table covering translations for all 6 dict tables.
--
-- `kind` uses the logical name of the 6 tables that `load_all` in the service touches
-- (job/industry/comclass/city/part/question).
-- The primary tables (e.g. phpyun_industry) keep their `name` column unchanged as the zh-CN default value.
-- This _i18n table only stores non-default languages such as zh-TW / en;
-- when a translation is missing, runtime falls back to the primary table's name.
--
-- The full table is loaded once at startup into Arc<HashMap> and stays resident in the process;
-- translations changes require a restart, or a call to POST /v1/admin/dict-i18n/reload (TBD).

CREATE TABLE IF NOT EXISTS `phpyun_dict_i18n` (
    `kind`    VARCHAR(32) NOT NULL COMMENT 'Dictionary logical name: job/industry/comclass/city/part/question',
    `item_id` INT(11)     NOT NULL COMMENT 'Primary-table id',
    `lang`    VARCHAR(10) NOT NULL COMMENT 'BCP-47: zh-CN / zh-TW / en',
    `text`    VARCHAR(200) NOT NULL DEFAULT '' COMMENT 'Translated name',
    PRIMARY KEY (`kind`, `item_id`, `lang`),
    KEY `idx_lookup` (`kind`, `lang`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='Dictionary i18n translations';

-- Example translations: industries (fill in the rest as needed; this only inserts a few for demo).
-- In practice, translation work should be done by translators / machine-translation + review and inserted in one batch.
-- Note: `item_id` must match an existing id in the primary table, otherwise the DB will have orphan translations.
--
-- Using INSERT IGNORE below to avoid duplicates.
INSERT IGNORE INTO `phpyun_dict_i18n` (`kind`, `item_id`, `lang`, `text`) VALUES
    -- Industries (assumes the primary table has 1=Internet, 2=Finance, 3=Education; adjust for your DB).
    ('industry', 1, 'zh-TW', '互聯網/IT'),
    ('industry', 1, 'en',    'Internet / IT'),
    ('industry', 2, 'zh-TW', '金融'),
    ('industry', 2, 'en',    'Finance'),
    ('industry', 3, 'zh-TW', '教育'),
    ('industry', 3, 'en',    'Education');
