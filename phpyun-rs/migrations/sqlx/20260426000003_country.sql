-- Curated list of major world countries for UI selectors (~50 entries).
-- Distinct from `phpyun_region` (the full 250-country hierarchical tree):
-- this is a flat, denormalized lookup with rich per-country metadata
-- (zh-CN name, phone code, currency, flag emoji) that callers expect to
-- bundle into a single response — the use case is forms/dropdowns, not
-- region navigation.

CREATE TABLE IF NOT EXISTS `phpyun_country` (
  `id`           BIGINT UNSIGNED   NOT NULL AUTO_INCREMENT,
  `code`         CHAR(2)           NOT NULL COMMENT 'ISO 3166-1 alpha-2: CN/US/JP/...',
  `code3`        CHAR(3)           NOT NULL COMMENT 'ISO 3166-1 alpha-3: CHN/USA/JPN/...',
  `numeric_code` SMALLINT UNSIGNED NOT NULL COMMENT 'ISO 3166-1 numeric',
  `name_en`      VARCHAR(120)      NOT NULL COMMENT 'English name',
  `name_zh`      VARCHAR(120)      NOT NULL COMMENT 'Simplified Chinese name',
  `continent`    CHAR(2)           NOT NULL COMMENT 'AF/AN/AS/EU/NA/OC/SA',
  `phone_code`   VARCHAR(8)        NOT NULL COMMENT 'International dialing prefix without +',
  `currency`     CHAR(3)           NOT NULL COMMENT 'ISO 4217: CNY/USD/...',
  `flag`         VARCHAR(8)        NOT NULL COMMENT 'Unicode flag emoji',
  `sort`         INT               NOT NULL DEFAULT 0,
  `status`       TINYINT           NOT NULL DEFAULT 0 COMMENT '0/1=active, 2=deleted',
  `created_at`   BIGINT            NOT NULL DEFAULT 0,
  `updated_at`   BIGINT            NOT NULL DEFAULT 0,
  PRIMARY KEY (`id`),
  UNIQUE KEY `uniq_code` (`code`),
  UNIQUE KEY `uniq_code3` (`code3`),
  KEY `idx_continent_sort` (`continent`, `sort`),
  KEY `idx_sort` (`sort`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='Curated major-country lookup for UI selectors';

INSERT INTO `phpyun_country`
  (code, code3, numeric_code, name_en, name_zh, continent, phone_code, currency, flag, sort, status, created_at, updated_at)
VALUES
  -- Greater China (sorted first)
  ('CN', 'CHN', 156, 'China',          '中国',       'AS', '86',   'CNY', '🇨🇳',  1, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('HK', 'HKG', 344, 'Hong Kong',      '中国香港',   'AS', '852',  'HKD', '🇭🇰',  2, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('MO', 'MAC', 446, 'Macao',          '中国澳门',   'AS', '853',  'MOP', '🇲🇴',  3, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('TW', 'TWN', 158, 'Taiwan',         '中国台湾',   'AS', '886',  'TWD', '🇹🇼',  4, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  -- East / Southeast Asia
  ('JP', 'JPN', 392, 'Japan',          '日本',       'AS', '81',   'JPY', '🇯🇵', 10, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('KR', 'KOR', 410, 'South Korea',    '韩国',       'AS', '82',   'KRW', '🇰🇷', 11, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('KP', 'PRK', 408, 'North Korea',    '朝鲜',       'AS', '850',  'KPW', '🇰🇵', 12, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('MN', 'MNG', 496, 'Mongolia',       '蒙古',       'AS', '976',  'MNT', '🇲🇳', 13, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('SG', 'SGP', 702, 'Singapore',      '新加坡',     'AS', '65',   'SGD', '🇸🇬', 14, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('MY', 'MYS', 458, 'Malaysia',       '马来西亚',   'AS', '60',   'MYR', '🇲🇾', 15, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('TH', 'THA', 764, 'Thailand',       '泰国',       'AS', '66',   'THB', '🇹🇭', 16, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('VN', 'VNM', 704, 'Vietnam',        '越南',       'AS', '84',   'VND', '🇻🇳', 17, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('ID', 'IDN', 360, 'Indonesia',      '印度尼西亚', 'AS', '62',   'IDR', '🇮🇩', 18, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('PH', 'PHL', 608, 'Philippines',    '菲律宾',     'AS', '63',   'PHP', '🇵🇭', 19, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('MM', 'MMR', 104, 'Myanmar',        '缅甸',       'AS', '95',   'MMK', '🇲🇲', 20, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('KH', 'KHM', 116, 'Cambodia',       '柬埔寨',     'AS', '855',  'KHR', '🇰🇭', 21, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('LA', 'LAO', 418, 'Laos',           '老挝',       'AS', '856',  'LAK', '🇱🇦', 22, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('BN', 'BRN',  96, 'Brunei',         '文莱',       'AS', '673',  'BND', '🇧🇳', 23, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  -- South Asia
  ('IN', 'IND', 356, 'India',          '印度',       'AS', '91',   'INR', '🇮🇳', 30, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('PK', 'PAK', 586, 'Pakistan',       '巴基斯坦',   'AS', '92',   'PKR', '🇵🇰', 31, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('BD', 'BGD',  50, 'Bangladesh',     '孟加拉国',   'AS', '880',  'BDT', '🇧🇩', 32, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('LK', 'LKA', 144, 'Sri Lanka',      '斯里兰卡',   'AS', '94',   'LKR', '🇱🇰', 33, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('NP', 'NPL', 524, 'Nepal',          '尼泊尔',     'AS', '977',  'NPR', '🇳🇵', 34, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  -- Middle East
  ('AE', 'ARE', 784, 'United Arab Emirates', '阿联酋', 'AS', '971', 'AED', '🇦🇪', 40, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('SA', 'SAU', 682, 'Saudi Arabia',   '沙特阿拉伯', 'AS', '966',  'SAR', '🇸🇦', 41, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('IL', 'ISR', 376, 'Israel',         '以色列',     'AS', '972',  'ILS', '🇮🇱', 42, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('TR', 'TUR', 792, 'Türkiye',        '土耳其',     'AS', '90',   'TRY', '🇹🇷', 43, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('IR', 'IRN', 364, 'Iran',           '伊朗',       'AS', '98',   'IRR', '🇮🇷', 44, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('QA', 'QAT', 634, 'Qatar',          '卡塔尔',     'AS', '974',  'QAR', '🇶🇦', 45, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('KW', 'KWT', 414, 'Kuwait',         '科威特',     'AS', '965',  'KWD', '🇰🇼', 46, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  -- Europe
  ('GB', 'GBR', 826, 'United Kingdom', '英国',       'EU', '44',   'GBP', '🇬🇧', 50, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('DE', 'DEU', 276, 'Germany',        '德国',       'EU', '49',   'EUR', '🇩🇪', 51, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('FR', 'FRA', 250, 'France',         '法国',       'EU', '33',   'EUR', '🇫🇷', 52, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('IT', 'ITA', 380, 'Italy',          '意大利',     'EU', '39',   'EUR', '🇮🇹', 53, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('ES', 'ESP', 724, 'Spain',          '西班牙',     'EU', '34',   'EUR', '🇪🇸', 54, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('NL', 'NLD', 528, 'The Netherlands','荷兰',       'EU', '31',   'EUR', '🇳🇱', 55, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('CH', 'CHE', 756, 'Switzerland',    '瑞士',       'EU', '41',   'CHF', '🇨🇭', 56, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('SE', 'SWE', 752, 'Sweden',         '瑞典',       'EU', '46',   'SEK', '🇸🇪', 57, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('NO', 'NOR', 578, 'Norway',         '挪威',       'EU', '47',   'NOK', '🇳🇴', 58, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('DK', 'DNK', 208, 'Denmark',        '丹麦',       'EU', '45',   'DKK', '🇩🇰', 59, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('FI', 'FIN', 246, 'Finland',        '芬兰',       'EU', '358',  'EUR', '🇫🇮', 60, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('BE', 'BEL',  56, 'Belgium',        '比利时',     'EU', '32',   'EUR', '🇧🇪', 61, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('AT', 'AUT',  40, 'Austria',        '奥地利',     'EU', '43',   'EUR', '🇦🇹', 62, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('IE', 'IRL', 372, 'Ireland',        '爱尔兰',     'EU', '353',  'EUR', '🇮🇪', 63, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('PT', 'PRT', 620, 'Portugal',       '葡萄牙',     'EU', '351',  'EUR', '🇵🇹', 64, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('GR', 'GRC', 300, 'Greece',         '希腊',       'EU', '30',   'EUR', '🇬🇷', 65, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('PL', 'POL', 616, 'Poland',         '波兰',       'EU', '48',   'PLN', '🇵🇱', 66, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('CZ', 'CZE', 203, 'Czechia',        '捷克',       'EU', '420',  'CZK', '🇨🇿', 67, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('HU', 'HUN', 348, 'Hungary',        '匈牙利',     'EU', '36',   'HUF', '🇭🇺', 68, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('RO', 'ROU', 642, 'Romania',        '罗马尼亚',   'EU', '40',   'RON', '🇷🇴', 69, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('RU', 'RUS', 643, 'Russia',         '俄罗斯',     'EU', '7',    'RUB', '🇷🇺', 70, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('UA', 'UKR', 804, 'Ukraine',        '乌克兰',     'EU', '380',  'UAH', '🇺🇦', 71, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  -- Africa
  ('EG', 'EGY', 818, 'Egypt',          '埃及',       'AF', '20',   'EGP', '🇪🇬', 80, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('ZA', 'ZAF', 710, 'South Africa',   '南非',       'AF', '27',   'ZAR', '🇿🇦', 81, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('NG', 'NGA', 566, 'Nigeria',        '尼日利亚',   'AF', '234',  'NGN', '🇳🇬', 82, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('KE', 'KEN', 404, 'Kenya',          '肯尼亚',     'AF', '254',  'KES', '🇰🇪', 83, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('MA', 'MAR', 504, 'Morocco',        '摩洛哥',     'AF', '212',  'MAD', '🇲🇦', 84, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  -- North America
  ('US', 'USA', 840, 'United States',  '美国',       'NA', '1',    'USD', '🇺🇸', 90, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('CA', 'CAN', 124, 'Canada',         '加拿大',     'NA', '1',    'CAD', '🇨🇦', 91, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('MX', 'MEX', 484, 'Mexico',         '墨西哥',     'NA', '52',   'MXN', '🇲🇽', 92, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  -- South America
  ('BR', 'BRA',  76, 'Brazil',         '巴西',       'SA', '55',   'BRL', '🇧🇷',100, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('AR', 'ARG',  32, 'Argentina',      '阿根廷',     'SA', '54',   'ARS', '🇦🇷',101, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('CL', 'CHL', 152, 'Chile',          '智利',       'SA', '56',   'CLP', '🇨🇱',102, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('CO', 'COL', 170, 'Colombia',       '哥伦比亚',   'SA', '57',   'COP', '🇨🇴',103, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('PE', 'PER', 604, 'Peru',           '秘鲁',       'SA', '51',   'PEN', '🇵🇪',104, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  -- Oceania
  ('AU', 'AUS',  36, 'Australia',      '澳大利亚',   'OC', '61',   'AUD', '🇦🇺',110, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
  ('NZ', 'NZL', 554, 'New Zealand',    '新西兰',     'OC', '64',   'NZD', '🇳🇿',111, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP());
