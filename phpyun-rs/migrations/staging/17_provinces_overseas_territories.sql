-- ============================================================================
-- 17_provinces_overseas_territories.sql — 各国海外属地与无人/特殊领土
-- id 范围 11801-12200
-- 说明:这些地方人口稀少或人口为零(科考站、无人岛等),通常 1-5 行就够了
-- ----------------------------------------------------------------------------
-- 父 id (来自 01_countries.sql):
--   PR 波多黎各 = 4182        VI 美属维京 = 4240
--   VG 英属维京 = 4239        KY 开曼 = 4124
--   BM 百慕大 = 4027          AW 阿鲁巴 = 4014
--   CW 库拉索 = 4053          SX 荷属圣马丁 = 4211
--   AI 安圭拉 = 4005          MS 蒙特塞拉特 = 4152
--   BQ 博内尔 = 4030          TC 特克斯凯科斯 = 4214
--   BL 圣巴泰勒米 = 4026      MF 法属圣马丁 = 4141
--   PM 圣皮埃尔 = 4180        AS 美属萨摩亚 = 4011
--   GU 关岛 = 4092            MP 北马里亚纳 = 4149
--   UM 美外岛 = 4232          PF 法属波利尼西亚 = 4175
--   NC 新喀里多尼亚 = 4161    WF 瓦利斯富图纳 = 4243
--   CK 库克群岛 = 4045        NU 纽埃 = 4170
--   PN 皮特凯恩 = 4181        TK 托克劳 = 4220
--   GP 瓜德罗普 = 4087        MQ 马提尼克 = 4150
--   RE 留尼汪 = 4188          YT 马约特 = 4247
--   GF 法属圭亚那 = 4080      SH 圣赫勒拿 = 4199
--   FK 福克兰群岛 = 4072      GS 南乔治亚 = 4090
--   IO 英属印度洋 = 4106      EH 西撒哈拉 = 4066
--   AQ 南极洲 = 4009          BV 布维 = 4034
--   HM 赫德岛 = 4096          TF 法属南部 = 4216
--   CC 科科斯 = 4039          CX 圣诞岛 = 4054
--   NF 诺福克 = 4163          SJ 斯瓦尔巴 = 4201
-- ============================================================================

-- ===== Puerto Rico (4182) 主要 8 区, id 11801-11808 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11801, 4182, '圣胡安',             'S', 0, 1, 0, 1, 'San Juan'),
  (11802, 4182, '巴亚蒙',             'B', 0, 1, 0, 2, 'Bayamon'),
  (11803, 4182, '卡罗来纳',           'C', 0, 1, 0, 3, 'Carolina'),
  (11804, 4182, '波努塞',             'P', 0, 1, 0, 4, 'Ponce'),
  (11805, 4182, '卡瓜斯',             'C', 0, 1, 0, 5, 'Caguas'),
  (11806, 4182, '马亚圭斯',           'M', 0, 1, 0, 6, 'Mayaguez'),
  (11807, 4182, '阿雷西博',           'A', 0, 1, 0, 7, 'Arecibo'),
  (11808, 4182, '瓜伊纳沃',           'G', 0, 1, 0, 8, 'Guaynabo');

-- ===== US Virgin Islands (4240) 3 主岛, id 11811-11813 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11811, 4240, '圣托马斯',           'S', 0, 1, 0, 1, 'Saint Thomas'),
  (11812, 4240, '圣克鲁斯',           'S', 0, 1, 0, 2, 'Saint Croix'),
  (11813, 4240, '圣约翰',             'S', 0, 1, 0, 3, 'Saint John');

-- ===== British Virgin Islands (4239) 4 主岛, id 11815-11818 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11815, 4239, '罗德城',             'R', 0, 1, 0, 1, 'Road Town'),
  (11816, 4239, '维京戈达',           'V', 0, 1, 0, 2, 'Virgin Gorda'),
  (11817, 4239, '阿内加达',           'A', 0, 1, 0, 3, 'Anegada'),
  (11818, 4239, '约斯特',             'J', 0, 1, 0, 4, 'Jost Van Dyke');

-- ===== Cayman Islands (4124) 6 区, id 11821-11826 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11821, 4124, '乔治城',             'G', 0, 1, 0, 1, 'George Town'),
  (11822, 4124, '西湾',               'W', 0, 1, 0, 2, 'West Bay'),
  (11823, 4124, '保德登',             'B', 0, 1, 0, 3, 'Bodden Town'),
  (11824, 4124, '诺斯赛德',           'N', 0, 1, 0, 4, 'North Side'),
  (11825, 4124, '东端',               'E', 0, 1, 0, 5, 'East End'),
  (11826, 4124, '凯曼布拉克',         'C', 0, 1, 0, 6, 'Cayman Brac');

-- ===== Bermuda (4027) 9 教区, id 11831-11839 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11831, 4027, '汉密尔顿',           'H', 0, 1, 0, 1, 'Hamilton City'),
  (11832, 4027, '彭布罗克',           'P', 0, 1, 0, 2, 'Pembroke'),
  (11833, 4027, '德文郡',             'D', 0, 1, 0, 3, 'Devonshire'),
  (11834, 4027, '佩吉特',             'P', 0, 1, 0, 4, 'Paget'),
  (11835, 4027, '沃里克',             'W', 0, 1, 0, 5, 'Warwick'),
  (11836, 4027, '南安普敦',           'S', 0, 1, 0, 6, 'Southampton'),
  (11837, 4027, '桑迪斯',             'S', 0, 1, 0, 7, 'Sandys'),
  (11838, 4027, '史密斯',             'S', 0, 1, 0, 8, 'Smiths'),
  (11839, 4027, '圣乔治',             'S', 0, 1, 0, 9, 'St Georges');

-- ===== Aruba (4014) — 单个区, id 11841 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11841, 4014, '奥拉涅斯塔德',       'O', 0, 1, 0, 1, 'Oranjestad');

-- ===== Curacao (4053) 主要 4 区, id 11842-11845 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11842, 4053, '威廉斯塔德',         'W', 0, 1, 0, 1, 'Willemstad'),
  (11843, 4053, '蓬达',               'P', 0, 1, 0, 2, 'Punda'),
  (11844, 4053, '奥德班',             'O', 0, 1, 0, 3, 'Otrobanda'),
  (11845, 4053, '西庞达',             'W', 0, 1, 0, 4, 'Westpunt');

-- ===== Sint Maarten (4211) — 主要 1 区, id 11846 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11846, 4211, '菲利普斯堡',         'P', 0, 1, 0, 1, 'Philipsburg');

-- ===== Anguilla (4005) — 14 区,简化, id 11847-11849 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11847, 4005, '瓦利',               'V', 0, 1, 0, 1, 'The Valley'),
  (11848, 4005, '布卢厄因蒂安',       'B', 0, 1, 0, 2, 'Blowing Point'),
  (11849, 4005, '岛湾',               'I', 0, 1, 0, 3, 'Island Harbour');

-- ===== Montserrat (4152) — 3 教区, id 11851-11853 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11851, 4152, '布雷德',             'B', 0, 1, 0, 1, 'Brades'),
  (11852, 4152, '圣彼得',             'S', 0, 1, 0, 2, 'Saint Peter'),
  (11853, 4152, '圣安东尼',           'S', 0, 1, 0, 3, 'Saint Anthony');

-- ===== Bonaire (4030) — 主岛, id 11855-11856 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11855, 4030, '克拉伦代克',         'K', 0, 1, 0, 1, 'Kralendijk'),
  (11856, 4030, '林孔',               'R', 0, 1, 0, 2, 'Rincon');

-- ===== Turks and Caicos (4214) — 6 区, id 11861-11866 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11861, 4214, '科伯恩城',           'C', 0, 1, 0, 1, 'Cockburn Town'),
  (11862, 4214, '普罗维登西亚莱斯',   'P', 0, 1, 0, 2, 'Providenciales'),
  (11863, 4214, '大特克斯',           'G', 0, 1, 0, 3, 'Grand Turk'),
  (11864, 4214, '南凯科斯',           'S', 0, 1, 0, 4, 'South Caicos'),
  (11865, 4214, '中凯科斯',           'M', 0, 1, 0, 5, 'Middle Caicos'),
  (11866, 4214, '北凯科斯',           'N', 0, 1, 0, 6, 'North Caicos');

-- ===== Saint Barthelemy (4026) — 主区, id 11868 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11868, 4026, '古斯塔维亚',         'G', 0, 1, 0, 1, 'Gustavia');

-- ===== Saint Martin (FR) (4141) — 主区, id 11869 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11869, 4141, '马里戈',             'M', 0, 1, 0, 1, 'Marigot');

-- ===== Saint Pierre and Miquelon (4180) 2 区, id 11870-11871 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11870, 4180, '圣皮埃尔',           'S', 0, 1, 0, 1, 'Saint-Pierre'),
  (11871, 4180, '密克隆',             'M', 0, 1, 0, 2, 'Miquelon');

-- ===== American Samoa (4011) 5 区, id 11881-11885 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11881, 4011, '帕果帕果',           'P', 0, 1, 0, 1, 'Pago Pago'),
  (11882, 4011, '东区',               'E', 0, 1, 0, 2, 'Eastern'),
  (11883, 4011, '西区',               'W', 0, 1, 0, 3, 'Western'),
  (11884, 4011, '马努阿',             'M', 0, 1, 0, 4, 'Manua'),
  (11885, 4011, '罗斯环礁',           'R', 0, 1, 0, 5, 'Rose Atoll');

-- ===== Guam (4092) 19 村,简化 8, id 11891-11898 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11891, 4092, '阿加尼亚',           'H', 0, 1, 0, 1, 'Hagatna'),
  (11892, 4092, '德德多',             'D', 0, 1, 0, 2, 'Dededo'),
  (11893, 4092, '尤马塔克',           'Y', 0, 1, 0, 3, 'Yigo'),
  (11894, 4092, '塔穆宁',             'T', 0, 1, 0, 4, 'Tamuning'),
  (11895, 4092, '曼吉劳',             'M', 0, 1, 0, 5, 'Mangilao'),
  (11896, 4092, '巴里加达',           'B', 0, 1, 0, 6, 'Barrigada'),
  (11897, 4092, '塔拉福福',           'T', 0, 1, 0, 7, 'Talofofo'),
  (11898, 4092, '阿加特',             'A', 0, 1, 0, 8, 'Agat');

-- ===== Northern Mariana Islands (4149) 4 主岛, id 11901-11904 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11901, 4149, '塞班',               'S', 0, 1, 0, 1, 'Saipan'),
  (11902, 4149, '天宁',               'T', 0, 1, 0, 2, 'Tinian'),
  (11903, 4149, '罗塔',               'R', 0, 1, 0, 3, 'Rota'),
  (11904, 4149, '北部群岛',           'N', 0, 1, 0, 4, 'Northern Islands');

-- ===== US Outlying Islands (4232) 主要 8, id 11911-11918 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11911, 4232, '贝克岛',             'B', 0, 1, 0, 1, 'Baker Island'),
  (11912, 4232, '豪兰岛',             'H', 0, 1, 0, 2, 'Howland Island'),
  (11913, 4232, '贾维斯岛',           'J', 0, 1, 0, 3, 'Jarvis Island'),
  (11914, 4232, '约翰斯顿环礁',       'J', 0, 1, 0, 4, 'Johnston Atoll'),
  (11915, 4232, '金曼礁',             'K', 0, 1, 0, 5, 'Kingman Reef'),
  (11916, 4232, '中途岛',             'M', 0, 1, 0, 6, 'Midway Atoll'),
  (11917, 4232, '帕尔米拉环礁',       'P', 0, 1, 0, 7, 'Palmyra Atoll'),
  (11918, 4232, '威克岛',             'W', 0, 1, 0, 8, 'Wake Island');

-- ===== French Polynesia (4175) 5 群岛, id 11921-11925 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11921, 4175, '帕皮提',             'P', 0, 1, 0, 1, 'Papeete'),
  (11922, 4175, '社会群岛',           'S', 0, 1, 0, 2, 'Society Islands'),
  (11923, 4175, '土阿莫土',           'T', 0, 1, 0, 3, 'Tuamotu'),
  (11924, 4175, '马克萨斯',           'M', 0, 1, 0, 4, 'Marquesas'),
  (11925, 4175, '冈比尔',             'G', 0, 1, 0, 5, 'Gambier');

-- ===== New Caledonia (4161) 3 省, id 11931-11933 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11931, 4161, '南方省',             'S', 0, 1, 0, 1, 'Sud'),
  (11932, 4161, '北方省',             'N', 0, 1, 0, 2, 'Nord'),
  (11933, 4161, '罗亚蒂群岛',         'L', 0, 1, 0, 3, 'Loyalty Islands');

-- ===== Wallis and Futuna (4243) 3 区, id 11935-11937 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11935, 4243, '瓦利斯',             'W', 0, 1, 0, 1, 'Wallis'),
  (11936, 4243, '锡加韦',             'S', 0, 1, 0, 2, 'Sigave'),
  (11937, 4243, '阿洛',               'A', 0, 1, 0, 3, 'Alo');

-- ===== Cook Islands (4045) 主要 4 岛, id 11941-11944 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11941, 4045, '阿瓦鲁阿',           'A', 0, 1, 0, 1, 'Avarua'),
  (11942, 4045, '拉罗汤加',           'R', 0, 1, 0, 2, 'Rarotonga'),
  (11943, 4045, '艾图塔基',           'A', 0, 1, 0, 3, 'Aitutaki'),
  (11944, 4045, '阿蒂乌',             'A', 0, 1, 0, 4, 'Atiu');

-- ===== Niue (4170) 主岛, id 11945 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11945, 4170, '阿洛菲',             'A', 0, 1, 0, 1, 'Alofi');

-- ===== Pitcairn Islands (4181) 1 主岛, id 11946 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11946, 4181, '亚当斯敦',           'A', 0, 1, 0, 1, 'Adamstown');

-- ===== Tokelau (4220) 3 环礁, id 11947-11949 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11947, 4220, '阿塔富',             'A', 0, 1, 0, 1, 'Atafu'),
  (11948, 4220, '努库诺努',           'N', 0, 1, 0, 2, 'Nukunonu'),
  (11949, 4220, '法考福',             'F', 0, 1, 0, 3, 'Fakaofo');

-- ===== Guadeloupe (4087) 2 大区, id 11951-11952 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11951, 4087, '巴斯特尔',           'B', 0, 1, 0, 1, 'Basse-Terre'),
  (11952, 4087, '皮特尔角',           'P', 0, 1, 0, 2, 'Pointe-a-Pitre');

-- ===== Martinique (4150) 4 区, id 11953-11956 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11953, 4150, '法兰西堡',           'F', 0, 1, 0, 1, 'Fort-de-France'),
  (11954, 4150, '勒马林',             'L', 0, 1, 0, 2, 'Le Marin'),
  (11955, 4150, '拉特里尼泰',         'L', 0, 1, 0, 3, 'La Trinite'),
  (11956, 4150, '圣皮埃尔',           'S', 0, 1, 0, 4, 'Saint-Pierre');

-- ===== Reunion (4188) 4 区, id 11961-11964 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11961, 4188, '圣丹尼斯',           'S', 0, 1, 0, 1, 'Saint-Denis'),
  (11962, 4188, '圣保罗',             'S', 0, 1, 0, 2, 'Saint-Paul'),
  (11963, 4188, '圣皮埃尔',           'S', 0, 1, 0, 3, 'Saint-Pierre'),
  (11964, 4188, '圣贝努瓦',           'S', 0, 1, 0, 4, 'Saint-Benoit');

-- ===== Mayotte (4247) 2 区, id 11965-11966 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11965, 4247, '马穆楚',             'M', 0, 1, 0, 1, 'Mamoudzou'),
  (11966, 4247, '宗博利',             'D', 0, 1, 0, 2, 'Dzaoudzi');

-- ===== French Guiana (4080) 2 大区, id 11971-11972 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11971, 4080, '卡宴',               'C', 0, 1, 0, 1, 'Cayenne'),
  (11972, 4080, '圣洛朗',             'S', 0, 1, 0, 2, 'Saint-Laurent-du-Maroni');

-- ===== Saint Helena (4199) 3 岛, id 11975-11977 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11975, 4199, '詹姆斯敦',           'J', 0, 1, 0, 1, 'Jamestown'),
  (11976, 4199, '阿森松',             'A', 0, 1, 0, 2, 'Ascension'),
  (11977, 4199, '特里斯坦达库尼亚',   'T', 0, 1, 0, 3, 'Tristan da Cunha');

-- ===== Falkland Islands (4072) 2 主岛, id 11981-11982 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11981, 4072, '斯坦利港',           'S', 0, 1, 0, 1, 'Stanley'),
  (11982, 4072, '西福克兰',           'W', 0, 1, 0, 2, 'West Falkland');

-- ===== South Georgia (4090) 主区, id 11983 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11983, 4090, '格里特维肯',         'G', 0, 1, 0, 1, 'Grytviken');

-- ===== British Indian Ocean Territory (4106) 1, id 11985 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11985, 4106, '迪戈加西亚',         'D', 0, 1, 0, 1, 'Diego Garcia');

-- ===== Western Sahara (4066) 主要 4 区, id 11991-11994 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11991, 4066, '阿尤恩',             'L', 0, 1, 0, 1, 'Laayoune'),
  (11992, 4066, '达赫拉',             'D', 0, 1, 0, 2, 'Dakhla'),
  (11993, 4066, '布贾杜尔',           'B', 0, 1, 0, 3, 'Boujdour'),
  (11994, 4066, '塞马拉',             'S', 0, 1, 0, 4, 'Smara');

-- ===== Antarctica (4009) 主要科考站国家, id 12001-12005 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (12001, 4009, '南极半岛',           'A', 0, 1, 0, 1, 'Antarctic Peninsula'),
  (12002, 4009, '罗斯海',             'R', 0, 1, 0, 2, 'Ross Sea'),
  (12003, 4009, '威德尔海',           'W', 0, 1, 0, 3, 'Weddell Sea'),
  (12004, 4009, '东南极',             'E', 0, 1, 0, 4, 'East Antarctica'),
  (12005, 4009, '西南极',             'W', 0, 1, 0, 5, 'West Antarctica');

-- ===== Bouvet Island (4034) — 无人岛, id 12011 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (12011, 4034, '布维岛',             'B', 0, 1, 0, 1, 'Bouvet Island');

-- ===== Heard Island (4096) — 无人岛, id 12012 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (12012, 4096, '赫德岛',             'H', 0, 1, 0, 1, 'Heard Island');

-- ===== French Southern Territories (4216) 主要 5, id 12015-12019 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (12015, 4216, '凯尔盖朗',           'K', 0, 1, 0, 1, 'Kerguelen'),
  (12016, 4216, '克罗泽',             'C', 0, 1, 0, 2, 'Crozet'),
  (12017, 4216, '圣保罗',             'S', 0, 1, 0, 3, 'Saint Paul'),
  (12018, 4216, '阿姆斯特丹',         'A', 0, 1, 0, 4, 'Amsterdam'),
  (12019, 4216, '阿黛利地',           'A', 0, 1, 0, 5, 'Adelie Land');

-- ===== Cocos Islands (4039) — 主岛, id 12021 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (12021, 4039, '西岛',               'W', 0, 1, 0, 1, 'West Island');

-- ===== Christmas Island (4054) — 主区, id 12022 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (12022, 4054, '飞鱼湾',             'F', 0, 1, 0, 1, 'Flying Fish Cove');

-- ===== Norfolk Island (4163) — 主区, id 12023 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (12023, 4163, '金斯敦',             'K', 0, 1, 0, 1, 'Kingston');

-- ===== Svalbard (4201) 2 主区, id 12031-12032 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (12031, 4201, '朗伊尔城',           'L', 0, 1, 0, 1, 'Longyearbyen'),
  (12032, 4201, '巴伦支堡',           'B', 0, 1, 0, 2, 'Barentsburg');
