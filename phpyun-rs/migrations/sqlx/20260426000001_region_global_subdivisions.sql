-- Major-market first-level subdivisions (US/JP/KR/DE/GB/CA/AU/MY/SG/TH/VN/ID/IN/FR).
-- Adds 226 level=1 rows + ~450 zh-CN/zh-TW translations to phpyun_dict_i18n.
-- ISO 3166-2 codes used as the stable `code` value.

-- Major-market first-level subdivisions (level=1).
-- parent_id resolves at runtime via subquery against country code.

-- US: 51 subdivisions
INSERT IGNORE INTO phpyun_region (parent_id, country_code, code, level, name, sort, status, created_at, updated_at) VALUES
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-AL', 1, 'Alabama', 1, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-AK', 1, 'Alaska', 2, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-AZ', 1, 'Arizona', 3, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-AR', 1, 'Arkansas', 4, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-CA', 1, 'California', 5, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-CO', 1, 'Colorado', 6, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-CT', 1, 'Connecticut', 7, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-DE', 1, 'Delaware', 8, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-DC', 1, 'District of Columbia', 9, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-FL', 1, 'Florida', 10, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-GA', 1, 'Georgia', 11, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-HI', 1, 'Hawaii', 12, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-ID', 1, 'Idaho', 13, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-IL', 1, 'Illinois', 14, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-IN', 1, 'Indiana', 15, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-IA', 1, 'Iowa', 16, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-KS', 1, 'Kansas', 17, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-KY', 1, 'Kentucky', 18, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-LA', 1, 'Louisiana', 19, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-ME', 1, 'Maine', 20, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-MD', 1, 'Maryland', 21, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-MA', 1, 'Massachusetts', 22, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-MI', 1, 'Michigan', 23, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-MN', 1, 'Minnesota', 24, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-MS', 1, 'Mississippi', 25, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-MO', 1, 'Missouri', 26, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-MT', 1, 'Montana', 27, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-NE', 1, 'Nebraska', 28, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-NV', 1, 'Nevada', 29, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-NH', 1, 'New Hampshire', 30, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-NJ', 1, 'New Jersey', 31, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-NM', 1, 'New Mexico', 32, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-NY', 1, 'New York', 33, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-NC', 1, 'North Carolina', 34, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-ND', 1, 'North Dakota', 35, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-OH', 1, 'Ohio', 36, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-OK', 1, 'Oklahoma', 37, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-OR', 1, 'Oregon', 38, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-PA', 1, 'Pennsylvania', 39, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-RI', 1, 'Rhode Island', 40, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-SC', 1, 'South Carolina', 41, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-SD', 1, 'South Dakota', 42, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-TN', 1, 'Tennessee', 43, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-TX', 1, 'Texas', 44, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-UT', 1, 'Utah', 45, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-VT', 1, 'Vermont', 46, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-VA', 1, 'Virginia', 47, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-WA', 1, 'Washington', 48, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-WV', 1, 'West Virginia', 49, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-WI', 1, 'Wisconsin', 50, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='US') AS p), 'US', 'US-WY', 1, 'Wyoming', 51, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP());

-- JP: 47 subdivisions
INSERT IGNORE INTO phpyun_region (parent_id, country_code, code, level, name, sort, status, created_at, updated_at) VALUES
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='JP') AS p), 'JP', 'JP-01', 1, 'Hokkaido', 1, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='JP') AS p), 'JP', 'JP-02', 1, 'Aomori', 2, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='JP') AS p), 'JP', 'JP-03', 1, 'Iwate', 3, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='JP') AS p), 'JP', 'JP-04', 1, 'Miyagi', 4, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='JP') AS p), 'JP', 'JP-05', 1, 'Akita', 5, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='JP') AS p), 'JP', 'JP-06', 1, 'Yamagata', 6, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='JP') AS p), 'JP', 'JP-07', 1, 'Fukushima', 7, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='JP') AS p), 'JP', 'JP-08', 1, 'Ibaraki', 8, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='JP') AS p), 'JP', 'JP-09', 1, 'Tochigi', 9, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='JP') AS p), 'JP', 'JP-10', 1, 'Gunma', 10, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='JP') AS p), 'JP', 'JP-11', 1, 'Saitama', 11, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='JP') AS p), 'JP', 'JP-12', 1, 'Chiba', 12, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='JP') AS p), 'JP', 'JP-13', 1, 'Tokyo', 13, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='JP') AS p), 'JP', 'JP-14', 1, 'Kanagawa', 14, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='JP') AS p), 'JP', 'JP-15', 1, 'Niigata', 15, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='JP') AS p), 'JP', 'JP-16', 1, 'Toyama', 16, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='JP') AS p), 'JP', 'JP-17', 1, 'Ishikawa', 17, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='JP') AS p), 'JP', 'JP-18', 1, 'Fukui', 18, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='JP') AS p), 'JP', 'JP-19', 1, 'Yamanashi', 19, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='JP') AS p), 'JP', 'JP-20', 1, 'Nagano', 20, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='JP') AS p), 'JP', 'JP-21', 1, 'Gifu', 21, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='JP') AS p), 'JP', 'JP-22', 1, 'Shizuoka', 22, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='JP') AS p), 'JP', 'JP-23', 1, 'Aichi', 23, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='JP') AS p), 'JP', 'JP-24', 1, 'Mie', 24, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='JP') AS p), 'JP', 'JP-25', 1, 'Shiga', 25, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='JP') AS p), 'JP', 'JP-26', 1, 'Kyoto', 26, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='JP') AS p), 'JP', 'JP-27', 1, 'Osaka', 27, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='JP') AS p), 'JP', 'JP-28', 1, 'Hyogo', 28, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='JP') AS p), 'JP', 'JP-29', 1, 'Nara', 29, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='JP') AS p), 'JP', 'JP-30', 1, 'Wakayama', 30, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='JP') AS p), 'JP', 'JP-31', 1, 'Tottori', 31, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='JP') AS p), 'JP', 'JP-32', 1, 'Shimane', 32, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='JP') AS p), 'JP', 'JP-33', 1, 'Okayama', 33, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='JP') AS p), 'JP', 'JP-34', 1, 'Hiroshima', 34, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='JP') AS p), 'JP', 'JP-35', 1, 'Yamaguchi', 35, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='JP') AS p), 'JP', 'JP-36', 1, 'Tokushima', 36, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='JP') AS p), 'JP', 'JP-37', 1, 'Kagawa', 37, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='JP') AS p), 'JP', 'JP-38', 1, 'Ehime', 38, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='JP') AS p), 'JP', 'JP-39', 1, 'Kochi', 39, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='JP') AS p), 'JP', 'JP-40', 1, 'Fukuoka', 40, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='JP') AS p), 'JP', 'JP-41', 1, 'Saga', 41, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='JP') AS p), 'JP', 'JP-42', 1, 'Nagasaki', 42, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='JP') AS p), 'JP', 'JP-43', 1, 'Kumamoto', 43, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='JP') AS p), 'JP', 'JP-44', 1, 'Oita', 44, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='JP') AS p), 'JP', 'JP-45', 1, 'Miyazaki', 45, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='JP') AS p), 'JP', 'JP-46', 1, 'Kagoshima', 46, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='JP') AS p), 'JP', 'JP-47', 1, 'Okinawa', 47, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP());

-- KR: 17 subdivisions
INSERT IGNORE INTO phpyun_region (parent_id, country_code, code, level, name, sort, status, created_at, updated_at) VALUES
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='KR') AS p), 'KR', 'KR-11', 1, 'Seoul', 1, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='KR') AS p), 'KR', 'KR-26', 1, 'Busan', 2, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='KR') AS p), 'KR', 'KR-27', 1, 'Daegu', 3, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='KR') AS p), 'KR', 'KR-28', 1, 'Incheon', 4, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='KR') AS p), 'KR', 'KR-29', 1, 'Gwangju', 5, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='KR') AS p), 'KR', 'KR-30', 1, 'Daejeon', 6, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='KR') AS p), 'KR', 'KR-31', 1, 'Ulsan', 7, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='KR') AS p), 'KR', 'KR-50', 1, 'Sejong', 8, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='KR') AS p), 'KR', 'KR-41', 1, 'Gyeonggi', 9, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='KR') AS p), 'KR', 'KR-42', 1, 'Gangwon', 10, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='KR') AS p), 'KR', 'KR-43', 1, 'North Chungcheong', 11, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='KR') AS p), 'KR', 'KR-44', 1, 'South Chungcheong', 12, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='KR') AS p), 'KR', 'KR-45', 1, 'North Jeolla', 13, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='KR') AS p), 'KR', 'KR-46', 1, 'South Jeolla', 14, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='KR') AS p), 'KR', 'KR-47', 1, 'North Gyeongsang', 15, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='KR') AS p), 'KR', 'KR-48', 1, 'South Gyeongsang', 16, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='KR') AS p), 'KR', 'KR-49', 1, 'Jeju', 17, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP());

-- DE: 16 subdivisions
INSERT IGNORE INTO phpyun_region (parent_id, country_code, code, level, name, sort, status, created_at, updated_at) VALUES
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='DE') AS p), 'DE', 'DE-BW', 1, 'Baden-Württemberg', 1, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='DE') AS p), 'DE', 'DE-BY', 1, 'Bavaria', 2, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='DE') AS p), 'DE', 'DE-BE', 1, 'Berlin', 3, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='DE') AS p), 'DE', 'DE-BB', 1, 'Brandenburg', 4, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='DE') AS p), 'DE', 'DE-HB', 1, 'Bremen', 5, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='DE') AS p), 'DE', 'DE-HH', 1, 'Hamburg', 6, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='DE') AS p), 'DE', 'DE-HE', 1, 'Hesse', 7, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='DE') AS p), 'DE', 'DE-MV', 1, 'Mecklenburg-Vorpommern', 8, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='DE') AS p), 'DE', 'DE-NI', 1, 'Lower Saxony', 9, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='DE') AS p), 'DE', 'DE-NW', 1, 'North Rhine-Westphalia', 10, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='DE') AS p), 'DE', 'DE-RP', 1, 'Rhineland-Palatinate', 11, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='DE') AS p), 'DE', 'DE-SL', 1, 'Saarland', 12, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='DE') AS p), 'DE', 'DE-SN', 1, 'Saxony', 13, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='DE') AS p), 'DE', 'DE-ST', 1, 'Saxony-Anhalt', 14, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='DE') AS p), 'DE', 'DE-SH', 1, 'Schleswig-Holstein', 15, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='DE') AS p), 'DE', 'DE-TH', 1, 'Thuringia', 16, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP());

-- GB: 4 subdivisions
INSERT IGNORE INTO phpyun_region (parent_id, country_code, code, level, name, sort, status, created_at, updated_at) VALUES
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='GB') AS p), 'GB', 'GB-ENG', 1, 'England', 1, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='GB') AS p), 'GB', 'GB-SCT', 1, 'Scotland', 2, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='GB') AS p), 'GB', 'GB-WLS', 1, 'Wales', 3, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='GB') AS p), 'GB', 'GB-NIR', 1, 'Northern Ireland', 4, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP());

-- CA: 13 subdivisions
INSERT IGNORE INTO phpyun_region (parent_id, country_code, code, level, name, sort, status, created_at, updated_at) VALUES
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='CA') AS p), 'CA', 'CA-AB', 1, 'Alberta', 1, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='CA') AS p), 'CA', 'CA-BC', 1, 'British Columbia', 2, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='CA') AS p), 'CA', 'CA-MB', 1, 'Manitoba', 3, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='CA') AS p), 'CA', 'CA-NB', 1, 'New Brunswick', 4, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='CA') AS p), 'CA', 'CA-NL', 1, 'Newfoundland and Labrador', 5, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='CA') AS p), 'CA', 'CA-NS', 1, 'Nova Scotia', 6, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='CA') AS p), 'CA', 'CA-NT', 1, 'Northwest Territories', 7, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='CA') AS p), 'CA', 'CA-NU', 1, 'Nunavut', 8, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='CA') AS p), 'CA', 'CA-ON', 1, 'Ontario', 9, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='CA') AS p), 'CA', 'CA-PE', 1, 'Prince Edward Island', 10, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='CA') AS p), 'CA', 'CA-QC', 1, 'Quebec', 11, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='CA') AS p), 'CA', 'CA-SK', 1, 'Saskatchewan', 12, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='CA') AS p), 'CA', 'CA-YT', 1, 'Yukon', 13, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP());

-- AU: 8 subdivisions
INSERT IGNORE INTO phpyun_region (parent_id, country_code, code, level, name, sort, status, created_at, updated_at) VALUES
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='AU') AS p), 'AU', 'AU-NSW', 1, 'New South Wales', 1, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='AU') AS p), 'AU', 'AU-QLD', 1, 'Queensland', 2, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='AU') AS p), 'AU', 'AU-SA', 1, 'South Australia', 3, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='AU') AS p), 'AU', 'AU-TAS', 1, 'Tasmania', 4, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='AU') AS p), 'AU', 'AU-VIC', 1, 'Victoria', 5, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='AU') AS p), 'AU', 'AU-WA', 1, 'Western Australia', 6, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='AU') AS p), 'AU', 'AU-ACT', 1, 'Australian Capital Territory', 7, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='AU') AS p), 'AU', 'AU-NT', 1, 'Northern Territory', 8, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP());

-- MY: 16 subdivisions
INSERT IGNORE INTO phpyun_region (parent_id, country_code, code, level, name, sort, status, created_at, updated_at) VALUES
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='MY') AS p), 'MY', 'MY-01', 1, 'Johor', 1, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='MY') AS p), 'MY', 'MY-02', 1, 'Kedah', 2, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='MY') AS p), 'MY', 'MY-03', 1, 'Kelantan', 3, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='MY') AS p), 'MY', 'MY-04', 1, 'Melaka', 4, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='MY') AS p), 'MY', 'MY-05', 1, 'Negeri Sembilan', 5, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='MY') AS p), 'MY', 'MY-06', 1, 'Pahang', 6, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='MY') AS p), 'MY', 'MY-07', 1, 'Pulau Pinang', 7, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='MY') AS p), 'MY', 'MY-08', 1, 'Perak', 8, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='MY') AS p), 'MY', 'MY-09', 1, 'Perlis', 9, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='MY') AS p), 'MY', 'MY-10', 1, 'Selangor', 10, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='MY') AS p), 'MY', 'MY-11', 1, 'Terengganu', 11, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='MY') AS p), 'MY', 'MY-12', 1, 'Sabah', 12, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='MY') AS p), 'MY', 'MY-13', 1, 'Sarawak', 13, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='MY') AS p), 'MY', 'MY-14', 1, 'Kuala Lumpur', 14, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='MY') AS p), 'MY', 'MY-15', 1, 'Labuan', 15, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='MY') AS p), 'MY', 'MY-16', 1, 'Putrajaya', 16, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP());

-- SG: 5 subdivisions
INSERT IGNORE INTO phpyun_region (parent_id, country_code, code, level, name, sort, status, created_at, updated_at) VALUES
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='SG') AS p), 'SG', 'SG-01', 1, 'Central Singapore', 1, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='SG') AS p), 'SG', 'SG-02', 1, 'North East', 2, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='SG') AS p), 'SG', 'SG-03', 1, 'North West', 3, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='SG') AS p), 'SG', 'SG-04', 1, 'South East', 4, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='SG') AS p), 'SG', 'SG-05', 1, 'South West', 5, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP());

-- TH: 8 subdivisions
INSERT IGNORE INTO phpyun_region (parent_id, country_code, code, level, name, sort, status, created_at, updated_at) VALUES
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='TH') AS p), 'TH', 'TH-10', 1, 'Bangkok', 1, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='TH') AS p), 'TH', 'TH-50', 1, 'Chiang Mai', 2, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='TH') AS p), 'TH', 'TH-83', 1, 'Phuket', 3, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='TH') AS p), 'TH', 'TH-20', 1, 'Chonburi', 4, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='TH') AS p), 'TH', 'TH-90', 1, 'Songkhla', 5, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='TH') AS p), 'TH', 'TH-30', 1, 'Nakhon Ratchasima', 6, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='TH') AS p), 'TH', 'TH-40', 1, 'Khon Kaen', 7, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='TH') AS p), 'TH', 'TH-80', 1, 'Nakhon Si Thammarat', 8, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP());

-- VN: 5 subdivisions
INSERT IGNORE INTO phpyun_region (parent_id, country_code, code, level, name, sort, status, created_at, updated_at) VALUES
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='VN') AS p), 'VN', 'VN-HN', 1, 'Hanoi', 1, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='VN') AS p), 'VN', 'VN-SG', 1, 'Ho Chi Minh City', 2, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='VN') AS p), 'VN', 'VN-DN', 1, 'Da Nang', 3, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='VN') AS p), 'VN', 'VN-HP', 1, 'Hai Phong', 4, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='VN') AS p), 'VN', 'VN-CT', 1, 'Can Tho', 5, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP());

-- ID: 7 subdivisions
INSERT IGNORE INTO phpyun_region (parent_id, country_code, code, level, name, sort, status, created_at, updated_at) VALUES
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='ID') AS p), 'ID', 'ID-JK', 1, 'Jakarta', 1, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='ID') AS p), 'ID', 'ID-JB', 1, 'West Java', 2, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='ID') AS p), 'ID', 'ID-JT', 1, 'Central Java', 3, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='ID') AS p), 'ID', 'ID-JI', 1, 'East Java', 4, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='ID') AS p), 'ID', 'ID-BA', 1, 'Bali', 5, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='ID') AS p), 'ID', 'ID-SU', 1, 'North Sumatra', 6, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='ID') AS p), 'ID', 'ID-YO', 1, 'Yogyakarta', 7, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP());

-- IN: 16 subdivisions
INSERT IGNORE INTO phpyun_region (parent_id, country_code, code, level, name, sort, status, created_at, updated_at) VALUES
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='IN') AS p), 'IN', 'IN-DL', 1, 'Delhi', 1, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='IN') AS p), 'IN', 'IN-MH', 1, 'Maharashtra', 2, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='IN') AS p), 'IN', 'IN-KA', 1, 'Karnataka', 3, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='IN') AS p), 'IN', 'IN-TN', 1, 'Tamil Nadu', 4, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='IN') AS p), 'IN', 'IN-WB', 1, 'West Bengal', 5, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='IN') AS p), 'IN', 'IN-GJ', 1, 'Gujarat', 6, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='IN') AS p), 'IN', 'IN-UP', 1, 'Uttar Pradesh', 7, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='IN') AS p), 'IN', 'IN-AP', 1, 'Andhra Pradesh', 8, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='IN') AS p), 'IN', 'IN-TG', 1, 'Telangana', 9, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='IN') AS p), 'IN', 'IN-RJ', 1, 'Rajasthan', 10, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='IN') AS p), 'IN', 'IN-KL', 1, 'Kerala', 11, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='IN') AS p), 'IN', 'IN-PB', 1, 'Punjab', 12, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='IN') AS p), 'IN', 'IN-HR', 1, 'Haryana', 13, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='IN') AS p), 'IN', 'IN-MP', 1, 'Madhya Pradesh', 14, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='IN') AS p), 'IN', 'IN-BR', 1, 'Bihar', 15, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='IN') AS p), 'IN', 'IN-OD', 1, 'Odisha', 16, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP());

-- FR: 13 subdivisions
INSERT IGNORE INTO phpyun_region (parent_id, country_code, code, level, name, sort, status, created_at, updated_at) VALUES
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='FR') AS p), 'FR', 'FR-ARA', 1, 'Auvergne-Rhône-Alpes', 1, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='FR') AS p), 'FR', 'FR-BFC', 1, 'Bourgogne-Franche-Comté', 2, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='FR') AS p), 'FR', 'FR-BRE', 1, 'Brittany', 3, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='FR') AS p), 'FR', 'FR-CVL', 1, 'Centre-Val de Loire', 4, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='FR') AS p), 'FR', 'FR-COR', 1, 'Corsica', 5, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='FR') AS p), 'FR', 'FR-GES', 1, 'Grand Est', 6, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='FR') AS p), 'FR', 'FR-HDF', 1, 'Hauts-de-France', 7, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='FR') AS p), 'FR', 'FR-IDF', 1, 'Île-de-France', 8, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='FR') AS p), 'FR', 'FR-NOR', 1, 'Normandy', 9, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='FR') AS p), 'FR', 'FR-NAQ', 1, 'Nouvelle-Aquitaine', 10, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='FR') AS p), 'FR', 'FR-OCC', 1, 'Occitanie', 11, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='FR') AS p), 'FR', 'FR-PDL', 1, 'Pays de la Loire', 12, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP()),
((SELECT id FROM (SELECT id FROM phpyun_region WHERE code='FR') AS p), 'FR', 'FR-PAC', 1, 'Provence-Alpes-Côte dAzur', 13, 0, UNIX_TIMESTAMP(), UNIX_TIMESTAMP());

-- Translations (zh-CN + zh-TW) for the new subdivisions
-- US translations
INSERT IGNORE INTO phpyun_dict_i18n (kind, item_id, lang, text)
SELECT 'region', id, 'zh-CN', '阿拉巴马' FROM phpyun_region WHERE code='US-AL'
UNION ALL
SELECT 'region', id, 'zh-TW', '阿拉巴馬' FROM phpyun_region WHERE code='US-AL'
UNION ALL
SELECT 'region', id, 'zh-CN', '阿拉斯加' FROM phpyun_region WHERE code='US-AK'
UNION ALL
SELECT 'region', id, 'zh-TW', '阿拉斯加' FROM phpyun_region WHERE code='US-AK'
UNION ALL
SELECT 'region', id, 'zh-CN', '亚利桑那' FROM phpyun_region WHERE code='US-AZ'
UNION ALL
SELECT 'region', id, 'zh-TW', '亞利桑那' FROM phpyun_region WHERE code='US-AZ'
UNION ALL
SELECT 'region', id, 'zh-CN', '阿肯色' FROM phpyun_region WHERE code='US-AR'
UNION ALL
SELECT 'region', id, 'zh-TW', '阿肯色' FROM phpyun_region WHERE code='US-AR'
UNION ALL
SELECT 'region', id, 'zh-CN', '加利福尼亚' FROM phpyun_region WHERE code='US-CA'
UNION ALL
SELECT 'region', id, 'zh-TW', '加利福尼亞' FROM phpyun_region WHERE code='US-CA'
UNION ALL
SELECT 'region', id, 'zh-CN', '科罗拉多' FROM phpyun_region WHERE code='US-CO'
UNION ALL
SELECT 'region', id, 'zh-TW', '科羅拉多' FROM phpyun_region WHERE code='US-CO'
UNION ALL
SELECT 'region', id, 'zh-CN', '康涅狄格' FROM phpyun_region WHERE code='US-CT'
UNION ALL
SELECT 'region', id, 'zh-TW', '康乃狄克' FROM phpyun_region WHERE code='US-CT'
UNION ALL
SELECT 'region', id, 'zh-CN', '特拉华' FROM phpyun_region WHERE code='US-DE'
UNION ALL
SELECT 'region', id, 'zh-TW', '德拉瓦' FROM phpyun_region WHERE code='US-DE'
UNION ALL
SELECT 'region', id, 'zh-CN', '哥伦比亚特区' FROM phpyun_region WHERE code='US-DC'
UNION ALL
SELECT 'region', id, 'zh-TW', '哥倫比亞特區' FROM phpyun_region WHERE code='US-DC'
UNION ALL
SELECT 'region', id, 'zh-CN', '佛罗里达' FROM phpyun_region WHERE code='US-FL'
UNION ALL
SELECT 'region', id, 'zh-TW', '佛羅里達' FROM phpyun_region WHERE code='US-FL'
UNION ALL
SELECT 'region', id, 'zh-CN', '佐治亚' FROM phpyun_region WHERE code='US-GA'
UNION ALL
SELECT 'region', id, 'zh-TW', '喬治亞' FROM phpyun_region WHERE code='US-GA'
UNION ALL
SELECT 'region', id, 'zh-CN', '夏威夷' FROM phpyun_region WHERE code='US-HI'
UNION ALL
SELECT 'region', id, 'zh-TW', '夏威夷' FROM phpyun_region WHERE code='US-HI'
UNION ALL
SELECT 'region', id, 'zh-CN', '爱达荷' FROM phpyun_region WHERE code='US-ID'
UNION ALL
SELECT 'region', id, 'zh-TW', '愛達荷' FROM phpyun_region WHERE code='US-ID'
UNION ALL
SELECT 'region', id, 'zh-CN', '伊利诺伊' FROM phpyun_region WHERE code='US-IL'
UNION ALL
SELECT 'region', id, 'zh-TW', '伊利諾' FROM phpyun_region WHERE code='US-IL'
UNION ALL
SELECT 'region', id, 'zh-CN', '印第安纳' FROM phpyun_region WHERE code='US-IN'
UNION ALL
SELECT 'region', id, 'zh-TW', '印第安納' FROM phpyun_region WHERE code='US-IN'
UNION ALL
SELECT 'region', id, 'zh-CN', '爱荷华' FROM phpyun_region WHERE code='US-IA'
UNION ALL
SELECT 'region', id, 'zh-TW', '愛荷華' FROM phpyun_region WHERE code='US-IA'
UNION ALL
SELECT 'region', id, 'zh-CN', '堪萨斯' FROM phpyun_region WHERE code='US-KS'
UNION ALL
SELECT 'region', id, 'zh-TW', '堪薩斯' FROM phpyun_region WHERE code='US-KS'
UNION ALL
SELECT 'region', id, 'zh-CN', '肯塔基' FROM phpyun_region WHERE code='US-KY'
UNION ALL
SELECT 'region', id, 'zh-TW', '肯塔基' FROM phpyun_region WHERE code='US-KY'
UNION ALL
SELECT 'region', id, 'zh-CN', '路易斯安那' FROM phpyun_region WHERE code='US-LA'
UNION ALL
SELECT 'region', id, 'zh-TW', '路易斯安那' FROM phpyun_region WHERE code='US-LA'
UNION ALL
SELECT 'region', id, 'zh-CN', '缅因' FROM phpyun_region WHERE code='US-ME'
UNION ALL
SELECT 'region', id, 'zh-TW', '緬因' FROM phpyun_region WHERE code='US-ME'
UNION ALL
SELECT 'region', id, 'zh-CN', '马里兰' FROM phpyun_region WHERE code='US-MD'
UNION ALL
SELECT 'region', id, 'zh-TW', '馬里蘭' FROM phpyun_region WHERE code='US-MD'
UNION ALL
SELECT 'region', id, 'zh-CN', '马萨诸塞' FROM phpyun_region WHERE code='US-MA'
UNION ALL
SELECT 'region', id, 'zh-TW', '麻薩諸塞' FROM phpyun_region WHERE code='US-MA'
UNION ALL
SELECT 'region', id, 'zh-CN', '密歇根' FROM phpyun_region WHERE code='US-MI'
UNION ALL
SELECT 'region', id, 'zh-TW', '密西根' FROM phpyun_region WHERE code='US-MI'
UNION ALL
SELECT 'region', id, 'zh-CN', '明尼苏达' FROM phpyun_region WHERE code='US-MN'
UNION ALL
SELECT 'region', id, 'zh-TW', '明尼蘇達' FROM phpyun_region WHERE code='US-MN'
UNION ALL
SELECT 'region', id, 'zh-CN', '密西西比' FROM phpyun_region WHERE code='US-MS'
UNION ALL
SELECT 'region', id, 'zh-TW', '密西西比' FROM phpyun_region WHERE code='US-MS'
UNION ALL
SELECT 'region', id, 'zh-CN', '密苏里' FROM phpyun_region WHERE code='US-MO'
UNION ALL
SELECT 'region', id, 'zh-TW', '密蘇里' FROM phpyun_region WHERE code='US-MO'
UNION ALL
SELECT 'region', id, 'zh-CN', '蒙大拿' FROM phpyun_region WHERE code='US-MT'
UNION ALL
SELECT 'region', id, 'zh-TW', '蒙大拿' FROM phpyun_region WHERE code='US-MT'
UNION ALL
SELECT 'region', id, 'zh-CN', '内布拉斯加' FROM phpyun_region WHERE code='US-NE'
UNION ALL
SELECT 'region', id, 'zh-TW', '內布拉斯加' FROM phpyun_region WHERE code='US-NE'
UNION ALL
SELECT 'region', id, 'zh-CN', '内华达' FROM phpyun_region WHERE code='US-NV'
UNION ALL
SELECT 'region', id, 'zh-TW', '內華達' FROM phpyun_region WHERE code='US-NV'
UNION ALL
SELECT 'region', id, 'zh-CN', '新罕布什尔' FROM phpyun_region WHERE code='US-NH'
UNION ALL
SELECT 'region', id, 'zh-TW', '新罕布夏' FROM phpyun_region WHERE code='US-NH'
UNION ALL
SELECT 'region', id, 'zh-CN', '新泽西' FROM phpyun_region WHERE code='US-NJ'
UNION ALL
SELECT 'region', id, 'zh-TW', '新澤西' FROM phpyun_region WHERE code='US-NJ'
UNION ALL
SELECT 'region', id, 'zh-CN', '新墨西哥' FROM phpyun_region WHERE code='US-NM'
UNION ALL
SELECT 'region', id, 'zh-TW', '新墨西哥' FROM phpyun_region WHERE code='US-NM'
UNION ALL
SELECT 'region', id, 'zh-CN', '纽约' FROM phpyun_region WHERE code='US-NY'
UNION ALL
SELECT 'region', id, 'zh-TW', '紐約' FROM phpyun_region WHERE code='US-NY'
UNION ALL
SELECT 'region', id, 'zh-CN', '北卡罗来纳' FROM phpyun_region WHERE code='US-NC'
UNION ALL
SELECT 'region', id, 'zh-TW', '北卡羅來納' FROM phpyun_region WHERE code='US-NC'
UNION ALL
SELECT 'region', id, 'zh-CN', '北达科他' FROM phpyun_region WHERE code='US-ND'
UNION ALL
SELECT 'region', id, 'zh-TW', '北達科他' FROM phpyun_region WHERE code='US-ND'
UNION ALL
SELECT 'region', id, 'zh-CN', '俄亥俄' FROM phpyun_region WHERE code='US-OH'
UNION ALL
SELECT 'region', id, 'zh-TW', '俄亥俄' FROM phpyun_region WHERE code='US-OH'
UNION ALL
SELECT 'region', id, 'zh-CN', '俄克拉荷马' FROM phpyun_region WHERE code='US-OK'
UNION ALL
SELECT 'region', id, 'zh-TW', '奧克拉荷馬' FROM phpyun_region WHERE code='US-OK'
UNION ALL
SELECT 'region', id, 'zh-CN', '俄勒冈' FROM phpyun_region WHERE code='US-OR'
UNION ALL
SELECT 'region', id, 'zh-TW', '奧勒岡' FROM phpyun_region WHERE code='US-OR'
UNION ALL
SELECT 'region', id, 'zh-CN', '宾夕法尼亚' FROM phpyun_region WHERE code='US-PA'
UNION ALL
SELECT 'region', id, 'zh-TW', '賓夕法尼亞' FROM phpyun_region WHERE code='US-PA'
UNION ALL
SELECT 'region', id, 'zh-CN', '罗得岛' FROM phpyun_region WHERE code='US-RI'
UNION ALL
SELECT 'region', id, 'zh-TW', '羅德島' FROM phpyun_region WHERE code='US-RI'
UNION ALL
SELECT 'region', id, 'zh-CN', '南卡罗来纳' FROM phpyun_region WHERE code='US-SC'
UNION ALL
SELECT 'region', id, 'zh-TW', '南卡羅來納' FROM phpyun_region WHERE code='US-SC'
UNION ALL
SELECT 'region', id, 'zh-CN', '南达科他' FROM phpyun_region WHERE code='US-SD'
UNION ALL
SELECT 'region', id, 'zh-TW', '南達科他' FROM phpyun_region WHERE code='US-SD'
UNION ALL
SELECT 'region', id, 'zh-CN', '田纳西' FROM phpyun_region WHERE code='US-TN'
UNION ALL
SELECT 'region', id, 'zh-TW', '田納西' FROM phpyun_region WHERE code='US-TN'
UNION ALL
SELECT 'region', id, 'zh-CN', '得克萨斯' FROM phpyun_region WHERE code='US-TX'
UNION ALL
SELECT 'region', id, 'zh-TW', '德克薩斯' FROM phpyun_region WHERE code='US-TX'
UNION ALL
SELECT 'region', id, 'zh-CN', '犹他' FROM phpyun_region WHERE code='US-UT'
UNION ALL
SELECT 'region', id, 'zh-TW', '猶他' FROM phpyun_region WHERE code='US-UT'
UNION ALL
SELECT 'region', id, 'zh-CN', '佛蒙特' FROM phpyun_region WHERE code='US-VT'
UNION ALL
SELECT 'region', id, 'zh-TW', '佛蒙特' FROM phpyun_region WHERE code='US-VT'
UNION ALL
SELECT 'region', id, 'zh-CN', '弗吉尼亚' FROM phpyun_region WHERE code='US-VA'
UNION ALL
SELECT 'region', id, 'zh-TW', '維吉尼亞' FROM phpyun_region WHERE code='US-VA'
UNION ALL
SELECT 'region', id, 'zh-CN', '华盛顿州' FROM phpyun_region WHERE code='US-WA'
UNION ALL
SELECT 'region', id, 'zh-TW', '華盛頓州' FROM phpyun_region WHERE code='US-WA'
UNION ALL
SELECT 'region', id, 'zh-CN', '西弗吉尼亚' FROM phpyun_region WHERE code='US-WV'
UNION ALL
SELECT 'region', id, 'zh-TW', '西維吉尼亞' FROM phpyun_region WHERE code='US-WV'
UNION ALL
SELECT 'region', id, 'zh-CN', '威斯康星' FROM phpyun_region WHERE code='US-WI'
UNION ALL
SELECT 'region', id, 'zh-TW', '威斯康辛' FROM phpyun_region WHERE code='US-WI'
UNION ALL
SELECT 'region', id, 'zh-CN', '怀俄明' FROM phpyun_region WHERE code='US-WY'
UNION ALL
SELECT 'region', id, 'zh-TW', '懷俄明' FROM phpyun_region WHERE code='US-WY';

-- JP translations
INSERT IGNORE INTO phpyun_dict_i18n (kind, item_id, lang, text)
SELECT 'region', id, 'zh-CN', '北海道' FROM phpyun_region WHERE code='JP-01'
UNION ALL
SELECT 'region', id, 'zh-TW', '北海道' FROM phpyun_region WHERE code='JP-01'
UNION ALL
SELECT 'region', id, 'zh-CN', '青森县' FROM phpyun_region WHERE code='JP-02'
UNION ALL
SELECT 'region', id, 'zh-TW', '青森縣' FROM phpyun_region WHERE code='JP-02'
UNION ALL
SELECT 'region', id, 'zh-CN', '岩手县' FROM phpyun_region WHERE code='JP-03'
UNION ALL
SELECT 'region', id, 'zh-TW', '岩手縣' FROM phpyun_region WHERE code='JP-03'
UNION ALL
SELECT 'region', id, 'zh-CN', '宫城县' FROM phpyun_region WHERE code='JP-04'
UNION ALL
SELECT 'region', id, 'zh-TW', '宮城縣' FROM phpyun_region WHERE code='JP-04'
UNION ALL
SELECT 'region', id, 'zh-CN', '秋田县' FROM phpyun_region WHERE code='JP-05'
UNION ALL
SELECT 'region', id, 'zh-TW', '秋田縣' FROM phpyun_region WHERE code='JP-05'
UNION ALL
SELECT 'region', id, 'zh-CN', '山形县' FROM phpyun_region WHERE code='JP-06'
UNION ALL
SELECT 'region', id, 'zh-TW', '山形縣' FROM phpyun_region WHERE code='JP-06'
UNION ALL
SELECT 'region', id, 'zh-CN', '福岛县' FROM phpyun_region WHERE code='JP-07'
UNION ALL
SELECT 'region', id, 'zh-TW', '福島縣' FROM phpyun_region WHERE code='JP-07'
UNION ALL
SELECT 'region', id, 'zh-CN', '茨城县' FROM phpyun_region WHERE code='JP-08'
UNION ALL
SELECT 'region', id, 'zh-TW', '茨城縣' FROM phpyun_region WHERE code='JP-08'
UNION ALL
SELECT 'region', id, 'zh-CN', '栃木县' FROM phpyun_region WHERE code='JP-09'
UNION ALL
SELECT 'region', id, 'zh-TW', '栃木縣' FROM phpyun_region WHERE code='JP-09'
UNION ALL
SELECT 'region', id, 'zh-CN', '群马县' FROM phpyun_region WHERE code='JP-10'
UNION ALL
SELECT 'region', id, 'zh-TW', '群馬縣' FROM phpyun_region WHERE code='JP-10'
UNION ALL
SELECT 'region', id, 'zh-CN', '埼玉县' FROM phpyun_region WHERE code='JP-11'
UNION ALL
SELECT 'region', id, 'zh-TW', '埼玉縣' FROM phpyun_region WHERE code='JP-11'
UNION ALL
SELECT 'region', id, 'zh-CN', '千叶县' FROM phpyun_region WHERE code='JP-12'
UNION ALL
SELECT 'region', id, 'zh-TW', '千葉縣' FROM phpyun_region WHERE code='JP-12'
UNION ALL
SELECT 'region', id, 'zh-CN', '东京都' FROM phpyun_region WHERE code='JP-13'
UNION ALL
SELECT 'region', id, 'zh-TW', '東京都' FROM phpyun_region WHERE code='JP-13'
UNION ALL
SELECT 'region', id, 'zh-CN', '神奈川县' FROM phpyun_region WHERE code='JP-14'
UNION ALL
SELECT 'region', id, 'zh-TW', '神奈川縣' FROM phpyun_region WHERE code='JP-14'
UNION ALL
SELECT 'region', id, 'zh-CN', '新潟县' FROM phpyun_region WHERE code='JP-15'
UNION ALL
SELECT 'region', id, 'zh-TW', '新潟縣' FROM phpyun_region WHERE code='JP-15'
UNION ALL
SELECT 'region', id, 'zh-CN', '富山县' FROM phpyun_region WHERE code='JP-16'
UNION ALL
SELECT 'region', id, 'zh-TW', '富山縣' FROM phpyun_region WHERE code='JP-16'
UNION ALL
SELECT 'region', id, 'zh-CN', '石川县' FROM phpyun_region WHERE code='JP-17'
UNION ALL
SELECT 'region', id, 'zh-TW', '石川縣' FROM phpyun_region WHERE code='JP-17'
UNION ALL
SELECT 'region', id, 'zh-CN', '福井县' FROM phpyun_region WHERE code='JP-18'
UNION ALL
SELECT 'region', id, 'zh-TW', '福井縣' FROM phpyun_region WHERE code='JP-18'
UNION ALL
SELECT 'region', id, 'zh-CN', '山梨县' FROM phpyun_region WHERE code='JP-19'
UNION ALL
SELECT 'region', id, 'zh-TW', '山梨縣' FROM phpyun_region WHERE code='JP-19'
UNION ALL
SELECT 'region', id, 'zh-CN', '长野县' FROM phpyun_region WHERE code='JP-20'
UNION ALL
SELECT 'region', id, 'zh-TW', '長野縣' FROM phpyun_region WHERE code='JP-20'
UNION ALL
SELECT 'region', id, 'zh-CN', '岐阜县' FROM phpyun_region WHERE code='JP-21'
UNION ALL
SELECT 'region', id, 'zh-TW', '岐阜縣' FROM phpyun_region WHERE code='JP-21'
UNION ALL
SELECT 'region', id, 'zh-CN', '静冈县' FROM phpyun_region WHERE code='JP-22'
UNION ALL
SELECT 'region', id, 'zh-TW', '靜岡縣' FROM phpyun_region WHERE code='JP-22'
UNION ALL
SELECT 'region', id, 'zh-CN', '爱知县' FROM phpyun_region WHERE code='JP-23'
UNION ALL
SELECT 'region', id, 'zh-TW', '愛知縣' FROM phpyun_region WHERE code='JP-23'
UNION ALL
SELECT 'region', id, 'zh-CN', '三重县' FROM phpyun_region WHERE code='JP-24'
UNION ALL
SELECT 'region', id, 'zh-TW', '三重縣' FROM phpyun_region WHERE code='JP-24'
UNION ALL
SELECT 'region', id, 'zh-CN', '滋贺县' FROM phpyun_region WHERE code='JP-25'
UNION ALL
SELECT 'region', id, 'zh-TW', '滋賀縣' FROM phpyun_region WHERE code='JP-25'
UNION ALL
SELECT 'region', id, 'zh-CN', '京都府' FROM phpyun_region WHERE code='JP-26'
UNION ALL
SELECT 'region', id, 'zh-TW', '京都府' FROM phpyun_region WHERE code='JP-26'
UNION ALL
SELECT 'region', id, 'zh-CN', '大阪府' FROM phpyun_region WHERE code='JP-27'
UNION ALL
SELECT 'region', id, 'zh-TW', '大阪府' FROM phpyun_region WHERE code='JP-27'
UNION ALL
SELECT 'region', id, 'zh-CN', '兵库县' FROM phpyun_region WHERE code='JP-28'
UNION ALL
SELECT 'region', id, 'zh-TW', '兵庫縣' FROM phpyun_region WHERE code='JP-28'
UNION ALL
SELECT 'region', id, 'zh-CN', '奈良县' FROM phpyun_region WHERE code='JP-29'
UNION ALL
SELECT 'region', id, 'zh-TW', '奈良縣' FROM phpyun_region WHERE code='JP-29'
UNION ALL
SELECT 'region', id, 'zh-CN', '和歌山县' FROM phpyun_region WHERE code='JP-30'
UNION ALL
SELECT 'region', id, 'zh-TW', '和歌山縣' FROM phpyun_region WHERE code='JP-30'
UNION ALL
SELECT 'region', id, 'zh-CN', '鸟取县' FROM phpyun_region WHERE code='JP-31'
UNION ALL
SELECT 'region', id, 'zh-TW', '鳥取縣' FROM phpyun_region WHERE code='JP-31'
UNION ALL
SELECT 'region', id, 'zh-CN', '岛根县' FROM phpyun_region WHERE code='JP-32'
UNION ALL
SELECT 'region', id, 'zh-TW', '島根縣' FROM phpyun_region WHERE code='JP-32'
UNION ALL
SELECT 'region', id, 'zh-CN', '冈山县' FROM phpyun_region WHERE code='JP-33'
UNION ALL
SELECT 'region', id, 'zh-TW', '岡山縣' FROM phpyun_region WHERE code='JP-33'
UNION ALL
SELECT 'region', id, 'zh-CN', '广岛县' FROM phpyun_region WHERE code='JP-34'
UNION ALL
SELECT 'region', id, 'zh-TW', '廣島縣' FROM phpyun_region WHERE code='JP-34'
UNION ALL
SELECT 'region', id, 'zh-CN', '山口县' FROM phpyun_region WHERE code='JP-35'
UNION ALL
SELECT 'region', id, 'zh-TW', '山口縣' FROM phpyun_region WHERE code='JP-35'
UNION ALL
SELECT 'region', id, 'zh-CN', '德岛县' FROM phpyun_region WHERE code='JP-36'
UNION ALL
SELECT 'region', id, 'zh-TW', '德島縣' FROM phpyun_region WHERE code='JP-36'
UNION ALL
SELECT 'region', id, 'zh-CN', '香川县' FROM phpyun_region WHERE code='JP-37'
UNION ALL
SELECT 'region', id, 'zh-TW', '香川縣' FROM phpyun_region WHERE code='JP-37'
UNION ALL
SELECT 'region', id, 'zh-CN', '爱媛县' FROM phpyun_region WHERE code='JP-38'
UNION ALL
SELECT 'region', id, 'zh-TW', '愛媛縣' FROM phpyun_region WHERE code='JP-38'
UNION ALL
SELECT 'region', id, 'zh-CN', '高知县' FROM phpyun_region WHERE code='JP-39'
UNION ALL
SELECT 'region', id, 'zh-TW', '高知縣' FROM phpyun_region WHERE code='JP-39'
UNION ALL
SELECT 'region', id, 'zh-CN', '福冈县' FROM phpyun_region WHERE code='JP-40'
UNION ALL
SELECT 'region', id, 'zh-TW', '福岡縣' FROM phpyun_region WHERE code='JP-40'
UNION ALL
SELECT 'region', id, 'zh-CN', '佐贺县' FROM phpyun_region WHERE code='JP-41'
UNION ALL
SELECT 'region', id, 'zh-TW', '佐賀縣' FROM phpyun_region WHERE code='JP-41'
UNION ALL
SELECT 'region', id, 'zh-CN', '长崎县' FROM phpyun_region WHERE code='JP-42'
UNION ALL
SELECT 'region', id, 'zh-TW', '長崎縣' FROM phpyun_region WHERE code='JP-42'
UNION ALL
SELECT 'region', id, 'zh-CN', '熊本县' FROM phpyun_region WHERE code='JP-43'
UNION ALL
SELECT 'region', id, 'zh-TW', '熊本縣' FROM phpyun_region WHERE code='JP-43'
UNION ALL
SELECT 'region', id, 'zh-CN', '大分县' FROM phpyun_region WHERE code='JP-44'
UNION ALL
SELECT 'region', id, 'zh-TW', '大分縣' FROM phpyun_region WHERE code='JP-44'
UNION ALL
SELECT 'region', id, 'zh-CN', '宫崎县' FROM phpyun_region WHERE code='JP-45'
UNION ALL
SELECT 'region', id, 'zh-TW', '宮崎縣' FROM phpyun_region WHERE code='JP-45'
UNION ALL
SELECT 'region', id, 'zh-CN', '鹿儿岛县' FROM phpyun_region WHERE code='JP-46'
UNION ALL
SELECT 'region', id, 'zh-TW', '鹿兒島縣' FROM phpyun_region WHERE code='JP-46'
UNION ALL
SELECT 'region', id, 'zh-CN', '冲绳县' FROM phpyun_region WHERE code='JP-47'
UNION ALL
SELECT 'region', id, 'zh-TW', '沖繩縣' FROM phpyun_region WHERE code='JP-47';

-- KR translations
INSERT IGNORE INTO phpyun_dict_i18n (kind, item_id, lang, text)
SELECT 'region', id, 'zh-CN', '首尔' FROM phpyun_region WHERE code='KR-11'
UNION ALL
SELECT 'region', id, 'zh-TW', '首爾' FROM phpyun_region WHERE code='KR-11'
UNION ALL
SELECT 'region', id, 'zh-CN', '釜山' FROM phpyun_region WHERE code='KR-26'
UNION ALL
SELECT 'region', id, 'zh-TW', '釜山' FROM phpyun_region WHERE code='KR-26'
UNION ALL
SELECT 'region', id, 'zh-CN', '大邱' FROM phpyun_region WHERE code='KR-27'
UNION ALL
SELECT 'region', id, 'zh-TW', '大邱' FROM phpyun_region WHERE code='KR-27'
UNION ALL
SELECT 'region', id, 'zh-CN', '仁川' FROM phpyun_region WHERE code='KR-28'
UNION ALL
SELECT 'region', id, 'zh-TW', '仁川' FROM phpyun_region WHERE code='KR-28'
UNION ALL
SELECT 'region', id, 'zh-CN', '光州' FROM phpyun_region WHERE code='KR-29'
UNION ALL
SELECT 'region', id, 'zh-TW', '光州' FROM phpyun_region WHERE code='KR-29'
UNION ALL
SELECT 'region', id, 'zh-CN', '大田' FROM phpyun_region WHERE code='KR-30'
UNION ALL
SELECT 'region', id, 'zh-TW', '大田' FROM phpyun_region WHERE code='KR-30'
UNION ALL
SELECT 'region', id, 'zh-CN', '蔚山' FROM phpyun_region WHERE code='KR-31'
UNION ALL
SELECT 'region', id, 'zh-TW', '蔚山' FROM phpyun_region WHERE code='KR-31'
UNION ALL
SELECT 'region', id, 'zh-CN', '世宗' FROM phpyun_region WHERE code='KR-50'
UNION ALL
SELECT 'region', id, 'zh-TW', '世宗' FROM phpyun_region WHERE code='KR-50'
UNION ALL
SELECT 'region', id, 'zh-CN', '京畿道' FROM phpyun_region WHERE code='KR-41'
UNION ALL
SELECT 'region', id, 'zh-TW', '京畿道' FROM phpyun_region WHERE code='KR-41'
UNION ALL
SELECT 'region', id, 'zh-CN', '江原道' FROM phpyun_region WHERE code='KR-42'
UNION ALL
SELECT 'region', id, 'zh-TW', '江原道' FROM phpyun_region WHERE code='KR-42'
UNION ALL
SELECT 'region', id, 'zh-CN', '忠清北道' FROM phpyun_region WHERE code='KR-43'
UNION ALL
SELECT 'region', id, 'zh-TW', '忠清北道' FROM phpyun_region WHERE code='KR-43'
UNION ALL
SELECT 'region', id, 'zh-CN', '忠清南道' FROM phpyun_region WHERE code='KR-44'
UNION ALL
SELECT 'region', id, 'zh-TW', '忠清南道' FROM phpyun_region WHERE code='KR-44'
UNION ALL
SELECT 'region', id, 'zh-CN', '全罗北道' FROM phpyun_region WHERE code='KR-45'
UNION ALL
SELECT 'region', id, 'zh-TW', '全羅北道' FROM phpyun_region WHERE code='KR-45'
UNION ALL
SELECT 'region', id, 'zh-CN', '全罗南道' FROM phpyun_region WHERE code='KR-46'
UNION ALL
SELECT 'region', id, 'zh-TW', '全羅南道' FROM phpyun_region WHERE code='KR-46'
UNION ALL
SELECT 'region', id, 'zh-CN', '庆尚北道' FROM phpyun_region WHERE code='KR-47'
UNION ALL
SELECT 'region', id, 'zh-TW', '慶尚北道' FROM phpyun_region WHERE code='KR-47'
UNION ALL
SELECT 'region', id, 'zh-CN', '庆尚南道' FROM phpyun_region WHERE code='KR-48'
UNION ALL
SELECT 'region', id, 'zh-TW', '慶尚南道' FROM phpyun_region WHERE code='KR-48'
UNION ALL
SELECT 'region', id, 'zh-CN', '济州' FROM phpyun_region WHERE code='KR-49'
UNION ALL
SELECT 'region', id, 'zh-TW', '濟州' FROM phpyun_region WHERE code='KR-49';

-- DE translations
INSERT IGNORE INTO phpyun_dict_i18n (kind, item_id, lang, text)
SELECT 'region', id, 'zh-CN', '巴登-符腾堡' FROM phpyun_region WHERE code='DE-BW'
UNION ALL
SELECT 'region', id, 'zh-TW', '巴登-符騰堡' FROM phpyun_region WHERE code='DE-BW'
UNION ALL
SELECT 'region', id, 'zh-CN', '巴伐利亚' FROM phpyun_region WHERE code='DE-BY'
UNION ALL
SELECT 'region', id, 'zh-TW', '巴伐利亞' FROM phpyun_region WHERE code='DE-BY'
UNION ALL
SELECT 'region', id, 'zh-CN', '柏林' FROM phpyun_region WHERE code='DE-BE'
UNION ALL
SELECT 'region', id, 'zh-TW', '柏林' FROM phpyun_region WHERE code='DE-BE'
UNION ALL
SELECT 'region', id, 'zh-CN', '勃兰登堡' FROM phpyun_region WHERE code='DE-BB'
UNION ALL
SELECT 'region', id, 'zh-TW', '布蘭登堡' FROM phpyun_region WHERE code='DE-BB'
UNION ALL
SELECT 'region', id, 'zh-CN', '不来梅' FROM phpyun_region WHERE code='DE-HB'
UNION ALL
SELECT 'region', id, 'zh-TW', '不來梅' FROM phpyun_region WHERE code='DE-HB'
UNION ALL
SELECT 'region', id, 'zh-CN', '汉堡' FROM phpyun_region WHERE code='DE-HH'
UNION ALL
SELECT 'region', id, 'zh-TW', '漢堡' FROM phpyun_region WHERE code='DE-HH'
UNION ALL
SELECT 'region', id, 'zh-CN', '黑森' FROM phpyun_region WHERE code='DE-HE'
UNION ALL
SELECT 'region', id, 'zh-TW', '黑森' FROM phpyun_region WHERE code='DE-HE'
UNION ALL
SELECT 'region', id, 'zh-CN', '梅克伦堡-前波美拉尼亚' FROM phpyun_region WHERE code='DE-MV'
UNION ALL
SELECT 'region', id, 'zh-TW', '梅克倫堡-前波美拉尼亞' FROM phpyun_region WHERE code='DE-MV'
UNION ALL
SELECT 'region', id, 'zh-CN', '下萨克森' FROM phpyun_region WHERE code='DE-NI'
UNION ALL
SELECT 'region', id, 'zh-TW', '下薩克森' FROM phpyun_region WHERE code='DE-NI'
UNION ALL
SELECT 'region', id, 'zh-CN', '北莱茵-威斯特法伦' FROM phpyun_region WHERE code='DE-NW'
UNION ALL
SELECT 'region', id, 'zh-TW', '北萊茵-威斯特法倫' FROM phpyun_region WHERE code='DE-NW'
UNION ALL
SELECT 'region', id, 'zh-CN', '莱茵兰-普法尔茨' FROM phpyun_region WHERE code='DE-RP'
UNION ALL
SELECT 'region', id, 'zh-TW', '萊茵蘭-普法爾茨' FROM phpyun_region WHERE code='DE-RP'
UNION ALL
SELECT 'region', id, 'zh-CN', '萨尔' FROM phpyun_region WHERE code='DE-SL'
UNION ALL
SELECT 'region', id, 'zh-TW', '薩爾' FROM phpyun_region WHERE code='DE-SL'
UNION ALL
SELECT 'region', id, 'zh-CN', '萨克森' FROM phpyun_region WHERE code='DE-SN'
UNION ALL
SELECT 'region', id, 'zh-TW', '薩克森' FROM phpyun_region WHERE code='DE-SN'
UNION ALL
SELECT 'region', id, 'zh-CN', '萨克森-安哈尔特' FROM phpyun_region WHERE code='DE-ST'
UNION ALL
SELECT 'region', id, 'zh-TW', '薩克森-安哈爾特' FROM phpyun_region WHERE code='DE-ST'
UNION ALL
SELECT 'region', id, 'zh-CN', '石勒苏益格-荷尔斯泰因' FROM phpyun_region WHERE code='DE-SH'
UNION ALL
SELECT 'region', id, 'zh-TW', '石勒蘇益格-荷爾斯泰因' FROM phpyun_region WHERE code='DE-SH'
UNION ALL
SELECT 'region', id, 'zh-CN', '图林根' FROM phpyun_region WHERE code='DE-TH'
UNION ALL
SELECT 'region', id, 'zh-TW', '圖林根' FROM phpyun_region WHERE code='DE-TH';

-- GB translations
INSERT IGNORE INTO phpyun_dict_i18n (kind, item_id, lang, text)
SELECT 'region', id, 'zh-CN', '英格兰' FROM phpyun_region WHERE code='GB-ENG'
UNION ALL
SELECT 'region', id, 'zh-TW', '英格蘭' FROM phpyun_region WHERE code='GB-ENG'
UNION ALL
SELECT 'region', id, 'zh-CN', '苏格兰' FROM phpyun_region WHERE code='GB-SCT'
UNION ALL
SELECT 'region', id, 'zh-TW', '蘇格蘭' FROM phpyun_region WHERE code='GB-SCT'
UNION ALL
SELECT 'region', id, 'zh-CN', '威尔士' FROM phpyun_region WHERE code='GB-WLS'
UNION ALL
SELECT 'region', id, 'zh-TW', '威爾斯' FROM phpyun_region WHERE code='GB-WLS'
UNION ALL
SELECT 'region', id, 'zh-CN', '北爱尔兰' FROM phpyun_region WHERE code='GB-NIR'
UNION ALL
SELECT 'region', id, 'zh-TW', '北愛爾蘭' FROM phpyun_region WHERE code='GB-NIR';

-- CA translations
INSERT IGNORE INTO phpyun_dict_i18n (kind, item_id, lang, text)
SELECT 'region', id, 'zh-CN', '阿尔伯塔' FROM phpyun_region WHERE code='CA-AB'
UNION ALL
SELECT 'region', id, 'zh-TW', '艾伯塔' FROM phpyun_region WHERE code='CA-AB'
UNION ALL
SELECT 'region', id, 'zh-CN', '不列颠哥伦比亚' FROM phpyun_region WHERE code='CA-BC'
UNION ALL
SELECT 'region', id, 'zh-TW', '卑詩省' FROM phpyun_region WHERE code='CA-BC'
UNION ALL
SELECT 'region', id, 'zh-CN', '马尼托巴' FROM phpyun_region WHERE code='CA-MB'
UNION ALL
SELECT 'region', id, 'zh-TW', '曼尼托巴' FROM phpyun_region WHERE code='CA-MB'
UNION ALL
SELECT 'region', id, 'zh-CN', '新不伦瑞克' FROM phpyun_region WHERE code='CA-NB'
UNION ALL
SELECT 'region', id, 'zh-TW', '新不倫瑞克' FROM phpyun_region WHERE code='CA-NB'
UNION ALL
SELECT 'region', id, 'zh-CN', '纽芬兰与拉布拉多' FROM phpyun_region WHERE code='CA-NL'
UNION ALL
SELECT 'region', id, 'zh-TW', '紐芬蘭與拉布拉多' FROM phpyun_region WHERE code='CA-NL'
UNION ALL
SELECT 'region', id, 'zh-CN', '新斯科舍' FROM phpyun_region WHERE code='CA-NS'
UNION ALL
SELECT 'region', id, 'zh-TW', '新斯科細亞' FROM phpyun_region WHERE code='CA-NS'
UNION ALL
SELECT 'region', id, 'zh-CN', '西北地区' FROM phpyun_region WHERE code='CA-NT'
UNION ALL
SELECT 'region', id, 'zh-TW', '西北地區' FROM phpyun_region WHERE code='CA-NT'
UNION ALL
SELECT 'region', id, 'zh-CN', '努纳武特' FROM phpyun_region WHERE code='CA-NU'
UNION ALL
SELECT 'region', id, 'zh-TW', '努納武特' FROM phpyun_region WHERE code='CA-NU'
UNION ALL
SELECT 'region', id, 'zh-CN', '安大略' FROM phpyun_region WHERE code='CA-ON'
UNION ALL
SELECT 'region', id, 'zh-TW', '安大略' FROM phpyun_region WHERE code='CA-ON'
UNION ALL
SELECT 'region', id, 'zh-CN', '爱德华王子岛' FROM phpyun_region WHERE code='CA-PE'
UNION ALL
SELECT 'region', id, 'zh-TW', '愛德華王子島' FROM phpyun_region WHERE code='CA-PE'
UNION ALL
SELECT 'region', id, 'zh-CN', '魁北克' FROM phpyun_region WHERE code='CA-QC'
UNION ALL
SELECT 'region', id, 'zh-TW', '魁北克' FROM phpyun_region WHERE code='CA-QC'
UNION ALL
SELECT 'region', id, 'zh-CN', '萨斯喀彻温' FROM phpyun_region WHERE code='CA-SK'
UNION ALL
SELECT 'region', id, 'zh-TW', '薩斯喀徹溫' FROM phpyun_region WHERE code='CA-SK'
UNION ALL
SELECT 'region', id, 'zh-CN', '育空' FROM phpyun_region WHERE code='CA-YT'
UNION ALL
SELECT 'region', id, 'zh-TW', '育空' FROM phpyun_region WHERE code='CA-YT';

-- AU translations
INSERT IGNORE INTO phpyun_dict_i18n (kind, item_id, lang, text)
SELECT 'region', id, 'zh-CN', '新南威尔士' FROM phpyun_region WHERE code='AU-NSW'
UNION ALL
SELECT 'region', id, 'zh-TW', '新南威爾斯' FROM phpyun_region WHERE code='AU-NSW'
UNION ALL
SELECT 'region', id, 'zh-CN', '昆士兰' FROM phpyun_region WHERE code='AU-QLD'
UNION ALL
SELECT 'region', id, 'zh-TW', '昆士蘭' FROM phpyun_region WHERE code='AU-QLD'
UNION ALL
SELECT 'region', id, 'zh-CN', '南澳大利亚' FROM phpyun_region WHERE code='AU-SA'
UNION ALL
SELECT 'region', id, 'zh-TW', '南澳大利亞' FROM phpyun_region WHERE code='AU-SA'
UNION ALL
SELECT 'region', id, 'zh-CN', '塔斯马尼亚' FROM phpyun_region WHERE code='AU-TAS'
UNION ALL
SELECT 'region', id, 'zh-TW', '塔斯馬尼亞' FROM phpyun_region WHERE code='AU-TAS'
UNION ALL
SELECT 'region', id, 'zh-CN', '维多利亚' FROM phpyun_region WHERE code='AU-VIC'
UNION ALL
SELECT 'region', id, 'zh-TW', '維多利亞' FROM phpyun_region WHERE code='AU-VIC'
UNION ALL
SELECT 'region', id, 'zh-CN', '西澳大利亚' FROM phpyun_region WHERE code='AU-WA'
UNION ALL
SELECT 'region', id, 'zh-TW', '西澳大利亞' FROM phpyun_region WHERE code='AU-WA'
UNION ALL
SELECT 'region', id, 'zh-CN', '澳大利亚首都领地' FROM phpyun_region WHERE code='AU-ACT'
UNION ALL
SELECT 'region', id, 'zh-TW', '澳大利亞首都領地' FROM phpyun_region WHERE code='AU-ACT'
UNION ALL
SELECT 'region', id, 'zh-CN', '北领地' FROM phpyun_region WHERE code='AU-NT'
UNION ALL
SELECT 'region', id, 'zh-TW', '北領地' FROM phpyun_region WHERE code='AU-NT';

-- MY translations
INSERT IGNORE INTO phpyun_dict_i18n (kind, item_id, lang, text)
SELECT 'region', id, 'zh-CN', '柔佛' FROM phpyun_region WHERE code='MY-01'
UNION ALL
SELECT 'region', id, 'zh-TW', '柔佛' FROM phpyun_region WHERE code='MY-01'
UNION ALL
SELECT 'region', id, 'zh-CN', '吉打' FROM phpyun_region WHERE code='MY-02'
UNION ALL
SELECT 'region', id, 'zh-TW', '吉打' FROM phpyun_region WHERE code='MY-02'
UNION ALL
SELECT 'region', id, 'zh-CN', '吉兰丹' FROM phpyun_region WHERE code='MY-03'
UNION ALL
SELECT 'region', id, 'zh-TW', '吉蘭丹' FROM phpyun_region WHERE code='MY-03'
UNION ALL
SELECT 'region', id, 'zh-CN', '马六甲' FROM phpyun_region WHERE code='MY-04'
UNION ALL
SELECT 'region', id, 'zh-TW', '馬六甲' FROM phpyun_region WHERE code='MY-04'
UNION ALL
SELECT 'region', id, 'zh-CN', '森美兰' FROM phpyun_region WHERE code='MY-05'
UNION ALL
SELECT 'region', id, 'zh-TW', '森美蘭' FROM phpyun_region WHERE code='MY-05'
UNION ALL
SELECT 'region', id, 'zh-CN', '彭亨' FROM phpyun_region WHERE code='MY-06'
UNION ALL
SELECT 'region', id, 'zh-TW', '彭亨' FROM phpyun_region WHERE code='MY-06'
UNION ALL
SELECT 'region', id, 'zh-CN', '槟城' FROM phpyun_region WHERE code='MY-07'
UNION ALL
SELECT 'region', id, 'zh-TW', '檳城' FROM phpyun_region WHERE code='MY-07'
UNION ALL
SELECT 'region', id, 'zh-CN', '霹雳' FROM phpyun_region WHERE code='MY-08'
UNION ALL
SELECT 'region', id, 'zh-TW', '霹靂' FROM phpyun_region WHERE code='MY-08'
UNION ALL
SELECT 'region', id, 'zh-CN', '玻璃市' FROM phpyun_region WHERE code='MY-09'
UNION ALL
SELECT 'region', id, 'zh-TW', '玻璃市' FROM phpyun_region WHERE code='MY-09'
UNION ALL
SELECT 'region', id, 'zh-CN', '雪兰莪' FROM phpyun_region WHERE code='MY-10'
UNION ALL
SELECT 'region', id, 'zh-TW', '雪蘭莪' FROM phpyun_region WHERE code='MY-10'
UNION ALL
SELECT 'region', id, 'zh-CN', '登嘉楼' FROM phpyun_region WHERE code='MY-11'
UNION ALL
SELECT 'region', id, 'zh-TW', '登嘉樓' FROM phpyun_region WHERE code='MY-11'
UNION ALL
SELECT 'region', id, 'zh-CN', '沙巴' FROM phpyun_region WHERE code='MY-12'
UNION ALL
SELECT 'region', id, 'zh-TW', '沙巴' FROM phpyun_region WHERE code='MY-12'
UNION ALL
SELECT 'region', id, 'zh-CN', '砂拉越' FROM phpyun_region WHERE code='MY-13'
UNION ALL
SELECT 'region', id, 'zh-TW', '砂拉越' FROM phpyun_region WHERE code='MY-13'
UNION ALL
SELECT 'region', id, 'zh-CN', '吉隆坡' FROM phpyun_region WHERE code='MY-14'
UNION ALL
SELECT 'region', id, 'zh-TW', '吉隆坡' FROM phpyun_region WHERE code='MY-14'
UNION ALL
SELECT 'region', id, 'zh-CN', '纳闽' FROM phpyun_region WHERE code='MY-15'
UNION ALL
SELECT 'region', id, 'zh-TW', '納閩' FROM phpyun_region WHERE code='MY-15'
UNION ALL
SELECT 'region', id, 'zh-CN', '布城' FROM phpyun_region WHERE code='MY-16'
UNION ALL
SELECT 'region', id, 'zh-TW', '布城' FROM phpyun_region WHERE code='MY-16';

-- SG translations
INSERT IGNORE INTO phpyun_dict_i18n (kind, item_id, lang, text)
SELECT 'region', id, 'zh-CN', '新加坡中区' FROM phpyun_region WHERE code='SG-01'
UNION ALL
SELECT 'region', id, 'zh-TW', '新加坡中區' FROM phpyun_region WHERE code='SG-01'
UNION ALL
SELECT 'region', id, 'zh-CN', '东北区' FROM phpyun_region WHERE code='SG-02'
UNION ALL
SELECT 'region', id, 'zh-TW', '東北區' FROM phpyun_region WHERE code='SG-02'
UNION ALL
SELECT 'region', id, 'zh-CN', '西北区' FROM phpyun_region WHERE code='SG-03'
UNION ALL
SELECT 'region', id, 'zh-TW', '西北區' FROM phpyun_region WHERE code='SG-03'
UNION ALL
SELECT 'region', id, 'zh-CN', '东南区' FROM phpyun_region WHERE code='SG-04'
UNION ALL
SELECT 'region', id, 'zh-TW', '東南區' FROM phpyun_region WHERE code='SG-04'
UNION ALL
SELECT 'region', id, 'zh-CN', '西南区' FROM phpyun_region WHERE code='SG-05'
UNION ALL
SELECT 'region', id, 'zh-TW', '西南區' FROM phpyun_region WHERE code='SG-05';

-- TH translations
INSERT IGNORE INTO phpyun_dict_i18n (kind, item_id, lang, text)
SELECT 'region', id, 'zh-CN', '曼谷' FROM phpyun_region WHERE code='TH-10'
UNION ALL
SELECT 'region', id, 'zh-TW', '曼谷' FROM phpyun_region WHERE code='TH-10'
UNION ALL
SELECT 'region', id, 'zh-CN', '清迈' FROM phpyun_region WHERE code='TH-50'
UNION ALL
SELECT 'region', id, 'zh-TW', '清邁' FROM phpyun_region WHERE code='TH-50'
UNION ALL
SELECT 'region', id, 'zh-CN', '普吉' FROM phpyun_region WHERE code='TH-83'
UNION ALL
SELECT 'region', id, 'zh-TW', '普吉' FROM phpyun_region WHERE code='TH-83'
UNION ALL
SELECT 'region', id, 'zh-CN', '春武里' FROM phpyun_region WHERE code='TH-20'
UNION ALL
SELECT 'region', id, 'zh-TW', '春武里' FROM phpyun_region WHERE code='TH-20'
UNION ALL
SELECT 'region', id, 'zh-CN', '宋卡' FROM phpyun_region WHERE code='TH-90'
UNION ALL
SELECT 'region', id, 'zh-TW', '宋卡' FROM phpyun_region WHERE code='TH-90'
UNION ALL
SELECT 'region', id, 'zh-CN', '呵叻' FROM phpyun_region WHERE code='TH-30'
UNION ALL
SELECT 'region', id, 'zh-TW', '呵叻' FROM phpyun_region WHERE code='TH-30'
UNION ALL
SELECT 'region', id, 'zh-CN', '孔敬' FROM phpyun_region WHERE code='TH-40'
UNION ALL
SELECT 'region', id, 'zh-TW', '孔敬' FROM phpyun_region WHERE code='TH-40'
UNION ALL
SELECT 'region', id, 'zh-CN', '那空是贪玛叻' FROM phpyun_region WHERE code='TH-80'
UNION ALL
SELECT 'region', id, 'zh-TW', '那空是貪瑪叻' FROM phpyun_region WHERE code='TH-80';

-- VN translations
INSERT IGNORE INTO phpyun_dict_i18n (kind, item_id, lang, text)
SELECT 'region', id, 'zh-CN', '河内' FROM phpyun_region WHERE code='VN-HN'
UNION ALL
SELECT 'region', id, 'zh-TW', '河內' FROM phpyun_region WHERE code='VN-HN'
UNION ALL
SELECT 'region', id, 'zh-CN', '胡志明市' FROM phpyun_region WHERE code='VN-SG'
UNION ALL
SELECT 'region', id, 'zh-TW', '胡志明市' FROM phpyun_region WHERE code='VN-SG'
UNION ALL
SELECT 'region', id, 'zh-CN', '岘港' FROM phpyun_region WHERE code='VN-DN'
UNION ALL
SELECT 'region', id, 'zh-TW', '峴港' FROM phpyun_region WHERE code='VN-DN'
UNION ALL
SELECT 'region', id, 'zh-CN', '海防' FROM phpyun_region WHERE code='VN-HP'
UNION ALL
SELECT 'region', id, 'zh-TW', '海防' FROM phpyun_region WHERE code='VN-HP'
UNION ALL
SELECT 'region', id, 'zh-CN', '芹苴' FROM phpyun_region WHERE code='VN-CT'
UNION ALL
SELECT 'region', id, 'zh-TW', '芹苴' FROM phpyun_region WHERE code='VN-CT';

-- ID translations
INSERT IGNORE INTO phpyun_dict_i18n (kind, item_id, lang, text)
SELECT 'region', id, 'zh-CN', '雅加达' FROM phpyun_region WHERE code='ID-JK'
UNION ALL
SELECT 'region', id, 'zh-TW', '雅加達' FROM phpyun_region WHERE code='ID-JK'
UNION ALL
SELECT 'region', id, 'zh-CN', '西爪哇' FROM phpyun_region WHERE code='ID-JB'
UNION ALL
SELECT 'region', id, 'zh-TW', '西爪哇' FROM phpyun_region WHERE code='ID-JB'
UNION ALL
SELECT 'region', id, 'zh-CN', '中爪哇' FROM phpyun_region WHERE code='ID-JT'
UNION ALL
SELECT 'region', id, 'zh-TW', '中爪哇' FROM phpyun_region WHERE code='ID-JT'
UNION ALL
SELECT 'region', id, 'zh-CN', '东爪哇' FROM phpyun_region WHERE code='ID-JI'
UNION ALL
SELECT 'region', id, 'zh-TW', '東爪哇' FROM phpyun_region WHERE code='ID-JI'
UNION ALL
SELECT 'region', id, 'zh-CN', '巴厘' FROM phpyun_region WHERE code='ID-BA'
UNION ALL
SELECT 'region', id, 'zh-TW', '峇里' FROM phpyun_region WHERE code='ID-BA'
UNION ALL
SELECT 'region', id, 'zh-CN', '北苏门答腊' FROM phpyun_region WHERE code='ID-SU'
UNION ALL
SELECT 'region', id, 'zh-TW', '北蘇門答臘' FROM phpyun_region WHERE code='ID-SU'
UNION ALL
SELECT 'region', id, 'zh-CN', '日惹' FROM phpyun_region WHERE code='ID-YO'
UNION ALL
SELECT 'region', id, 'zh-TW', '日惹' FROM phpyun_region WHERE code='ID-YO';

-- IN translations
INSERT IGNORE INTO phpyun_dict_i18n (kind, item_id, lang, text)
SELECT 'region', id, 'zh-CN', '德里' FROM phpyun_region WHERE code='IN-DL'
UNION ALL
SELECT 'region', id, 'zh-TW', '德里' FROM phpyun_region WHERE code='IN-DL'
UNION ALL
SELECT 'region', id, 'zh-CN', '马哈拉施特拉' FROM phpyun_region WHERE code='IN-MH'
UNION ALL
SELECT 'region', id, 'zh-TW', '馬哈拉施特拉' FROM phpyun_region WHERE code='IN-MH'
UNION ALL
SELECT 'region', id, 'zh-CN', '卡纳塔克' FROM phpyun_region WHERE code='IN-KA'
UNION ALL
SELECT 'region', id, 'zh-TW', '卡納塔克' FROM phpyun_region WHERE code='IN-KA'
UNION ALL
SELECT 'region', id, 'zh-CN', '泰米尔纳德' FROM phpyun_region WHERE code='IN-TN'
UNION ALL
SELECT 'region', id, 'zh-TW', '泰米爾納德' FROM phpyun_region WHERE code='IN-TN'
UNION ALL
SELECT 'region', id, 'zh-CN', '西孟加拉' FROM phpyun_region WHERE code='IN-WB'
UNION ALL
SELECT 'region', id, 'zh-TW', '西孟加拉' FROM phpyun_region WHERE code='IN-WB'
UNION ALL
SELECT 'region', id, 'zh-CN', '古吉拉特' FROM phpyun_region WHERE code='IN-GJ'
UNION ALL
SELECT 'region', id, 'zh-TW', '古吉拉特' FROM phpyun_region WHERE code='IN-GJ'
UNION ALL
SELECT 'region', id, 'zh-CN', '北方邦' FROM phpyun_region WHERE code='IN-UP'
UNION ALL
SELECT 'region', id, 'zh-TW', '北方邦' FROM phpyun_region WHERE code='IN-UP'
UNION ALL
SELECT 'region', id, 'zh-CN', '安得拉邦' FROM phpyun_region WHERE code='IN-AP'
UNION ALL
SELECT 'region', id, 'zh-TW', '安得拉邦' FROM phpyun_region WHERE code='IN-AP'
UNION ALL
SELECT 'region', id, 'zh-CN', '泰伦加纳' FROM phpyun_region WHERE code='IN-TG'
UNION ALL
SELECT 'region', id, 'zh-TW', '泰倫加納' FROM phpyun_region WHERE code='IN-TG'
UNION ALL
SELECT 'region', id, 'zh-CN', '拉贾斯坦' FROM phpyun_region WHERE code='IN-RJ'
UNION ALL
SELECT 'region', id, 'zh-TW', '拉賈斯坦' FROM phpyun_region WHERE code='IN-RJ'
UNION ALL
SELECT 'region', id, 'zh-CN', '喀拉拉' FROM phpyun_region WHERE code='IN-KL'
UNION ALL
SELECT 'region', id, 'zh-TW', '喀拉拉' FROM phpyun_region WHERE code='IN-KL'
UNION ALL
SELECT 'region', id, 'zh-CN', '旁遮普' FROM phpyun_region WHERE code='IN-PB'
UNION ALL
SELECT 'region', id, 'zh-TW', '旁遮普' FROM phpyun_region WHERE code='IN-PB'
UNION ALL
SELECT 'region', id, 'zh-CN', '哈里亚纳' FROM phpyun_region WHERE code='IN-HR'
UNION ALL
SELECT 'region', id, 'zh-TW', '哈里亞納' FROM phpyun_region WHERE code='IN-HR'
UNION ALL
SELECT 'region', id, 'zh-CN', '中央邦' FROM phpyun_region WHERE code='IN-MP'
UNION ALL
SELECT 'region', id, 'zh-TW', '中央邦' FROM phpyun_region WHERE code='IN-MP'
UNION ALL
SELECT 'region', id, 'zh-CN', '比哈尔' FROM phpyun_region WHERE code='IN-BR'
UNION ALL
SELECT 'region', id, 'zh-TW', '比哈爾' FROM phpyun_region WHERE code='IN-BR'
UNION ALL
SELECT 'region', id, 'zh-CN', '奥里萨' FROM phpyun_region WHERE code='IN-OD'
UNION ALL
SELECT 'region', id, 'zh-TW', '奧里薩' FROM phpyun_region WHERE code='IN-OD';

-- FR translations
INSERT IGNORE INTO phpyun_dict_i18n (kind, item_id, lang, text)
SELECT 'region', id, 'zh-CN', '奥弗涅-罗讷-阿尔卑斯' FROM phpyun_region WHERE code='FR-ARA'
UNION ALL
SELECT 'region', id, 'zh-TW', '奧弗涅-羅訥-阿爾卑斯' FROM phpyun_region WHERE code='FR-ARA'
UNION ALL
SELECT 'region', id, 'zh-CN', '勃艮第-弗朗什-孔泰' FROM phpyun_region WHERE code='FR-BFC'
UNION ALL
SELECT 'region', id, 'zh-TW', '勃艮第-弗朗什-孔泰' FROM phpyun_region WHERE code='FR-BFC'
UNION ALL
SELECT 'region', id, 'zh-CN', '布列塔尼' FROM phpyun_region WHERE code='FR-BRE'
UNION ALL
SELECT 'region', id, 'zh-TW', '布列塔尼' FROM phpyun_region WHERE code='FR-BRE'
UNION ALL
SELECT 'region', id, 'zh-CN', '中央-卢瓦尔河谷' FROM phpyun_region WHERE code='FR-CVL'
UNION ALL
SELECT 'region', id, 'zh-TW', '中央-盧瓦爾河谷' FROM phpyun_region WHERE code='FR-CVL'
UNION ALL
SELECT 'region', id, 'zh-CN', '科西嘉' FROM phpyun_region WHERE code='FR-COR'
UNION ALL
SELECT 'region', id, 'zh-TW', '科西嘉' FROM phpyun_region WHERE code='FR-COR'
UNION ALL
SELECT 'region', id, 'zh-CN', '大东部' FROM phpyun_region WHERE code='FR-GES'
UNION ALL
SELECT 'region', id, 'zh-TW', '大東部' FROM phpyun_region WHERE code='FR-GES'
UNION ALL
SELECT 'region', id, 'zh-CN', '上法兰西' FROM phpyun_region WHERE code='FR-HDF'
UNION ALL
SELECT 'region', id, 'zh-TW', '上法蘭西' FROM phpyun_region WHERE code='FR-HDF'
UNION ALL
SELECT 'region', id, 'zh-CN', '法兰西岛' FROM phpyun_region WHERE code='FR-IDF'
UNION ALL
SELECT 'region', id, 'zh-TW', '法蘭西島' FROM phpyun_region WHERE code='FR-IDF'
UNION ALL
SELECT 'region', id, 'zh-CN', '诺曼底' FROM phpyun_region WHERE code='FR-NOR'
UNION ALL
SELECT 'region', id, 'zh-TW', '諾曼第' FROM phpyun_region WHERE code='FR-NOR'
UNION ALL
SELECT 'region', id, 'zh-CN', '新阿基坦' FROM phpyun_region WHERE code='FR-NAQ'
UNION ALL
SELECT 'region', id, 'zh-TW', '新阿基坦' FROM phpyun_region WHERE code='FR-NAQ'
UNION ALL
SELECT 'region', id, 'zh-CN', '奥克西塔尼' FROM phpyun_region WHERE code='FR-OCC'
UNION ALL
SELECT 'region', id, 'zh-TW', '奧克西塔尼' FROM phpyun_region WHERE code='FR-OCC'
UNION ALL
SELECT 'region', id, 'zh-CN', '卢瓦尔河地区' FROM phpyun_region WHERE code='FR-PDL'
UNION ALL
SELECT 'region', id, 'zh-TW', '盧瓦爾河地區' FROM phpyun_region WHERE code='FR-PDL'
UNION ALL
SELECT 'region', id, 'zh-CN', '普罗旺斯-阿尔卑斯-蓝色海岸' FROM phpyun_region WHERE code='FR-PAC'
UNION ALL
SELECT 'region', id, 'zh-TW', '普羅旺斯-阿爾卑斯-蔚藍海岸' FROM phpyun_region WHERE code='FR-PAC';

-- Total subdivisions: 226
