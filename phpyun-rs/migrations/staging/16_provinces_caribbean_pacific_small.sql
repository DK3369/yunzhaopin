-- ============================================================================
-- 16_provinces_caribbean_pacific_small.sql — 加勒比小国 + 太平洋小岛 + 苏里南/圭亚那
-- id 范围 11501-11800
-- ----------------------------------------------------------------------------
-- 父 id (来自 01_countries.sql):
--   BS 巴哈马   = 4032   BB 巴巴多斯 = 4018   LC 圣卢西亚 = 4128
--   VC 圣文森特 = 4237   GD 格林纳达 = 4078   AG 安提瓜   = 4004
--   KN 圣基茨   = 4120   DM 多米尼克 = 4060
--   SR 苏里南   = 4207   GY 圭亚那   = 4094
--   MV 马尔代夫 = 4155   MH 马绍尔   = 4143   FM 密克罗尼西亚 = 4073
--   PW 帕劳     = 4185   NR 瑙鲁     = 4169   TV 图瓦卢   = 4227
--   KI 基里巴斯 = 4118   SB 所罗门   = 4194   VU 瓦努阿图 = 4242
--   WS 萨摩亚   = 4244   TO 汤加     = 4224
-- ============================================================================

-- ===== Bahamas (4032) 主要 10 岛区, id 11501-11510 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11501, 4032, '拿骚',               'N', 0, 1, 0, 1,  'Nassau'),
  (11502, 4032, '新普罗维登斯',       'N', 0, 1, 0, 2,  'New Providence'),
  (11503, 4032, '大巴哈马',           'G', 0, 1, 0, 3,  'Grand Bahama'),
  (11504, 4032, '阿巴科',             'A', 0, 1, 0, 4,  'Abaco'),
  (11505, 4032, '安德罗斯',           'A', 0, 1, 0, 5,  'Andros'),
  (11506, 4032, '伊柳塞拉',           'E', 0, 1, 0, 6,  'Eleuthera'),
  (11507, 4032, '艾克苏马',           'E', 0, 1, 0, 7,  'Exuma'),
  (11508, 4032, '比米尼',             'B', 0, 1, 0, 8,  'Bimini'),
  (11509, 4032, '伊纳瓜',             'I', 0, 1, 0, 9,  'Inagua'),
  (11510, 4032, '长岛',               'L', 0, 1, 0, 10, 'Long Island');

-- ===== Barbados (4018) 11 教区, id 11511-11521 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11511, 4018, '基督教堂',           'C', 0, 1, 0, 1,  'Christ Church'),
  (11512, 4018, '圣迈克尔',           'S', 0, 1, 0, 2,  'Saint Michael'),
  (11513, 4018, '圣詹姆斯',           'S', 0, 1, 0, 3,  'Saint James'),
  (11514, 4018, '圣彼得',             'S', 0, 1, 0, 4,  'Saint Peter'),
  (11515, 4018, '圣约翰',             'S', 0, 1, 0, 5,  'Saint John'),
  (11516, 4018, '圣约瑟',             'S', 0, 1, 0, 6,  'Saint Joseph'),
  (11517, 4018, '圣安德鲁',           'S', 0, 1, 0, 7,  'Saint Andrew'),
  (11518, 4018, '圣菲利普',           'S', 0, 1, 0, 8,  'Saint Philip'),
  (11519, 4018, '圣乔治',             'S', 0, 1, 0, 9,  'Saint George'),
  (11520, 4018, '圣托马斯',           'S', 0, 1, 0, 10, 'Saint Thomas'),
  (11521, 4018, '圣露西',             'S', 0, 1, 0, 11, 'Saint Lucy');

-- ===== Saint Lucia (4128) 11 区, id 11531-11541 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11531, 4128, '卡斯特里',           'C', 0, 1, 0, 1,  'Castries'),
  (11532, 4128, '阿努兹卡奈',         'A', 0, 1, 0, 2,  'Anse-la-Raye'),
  (11533, 4128, '康拉斯',             'C', 0, 1, 0, 3,  'Canaries'),
  (11534, 4128, '舒瓦瑟尔',           'C', 0, 1, 0, 4,  'Choiseul'),
  (11535, 4128, '丹尼里',             'D', 0, 1, 0, 5,  'Dennery'),
  (11536, 4128, '格罗斯岛',           'G', 0, 1, 0, 6,  'Gros Islet'),
  (11537, 4128, '拉博里',             'L', 0, 1, 0, 7,  'Laborie'),
  (11538, 4128, '米库德',             'M', 0, 1, 0, 8,  'Micoud'),
  (11539, 4128, '苏弗里耶尔',         'S', 0, 1, 0, 9,  'Soufriere'),
  (11540, 4128, '维约堡',             'V', 0, 1, 0, 10, 'Vieux Fort'),
  (11541, 4128, '北橘',               'N', 0, 1, 0, 11, 'North');

-- ===== Saint Vincent (4237) 6 教区, id 11551-11556 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11551, 4237, '金斯敦',             'K', 0, 1, 0, 1, 'Kingstown'),
  (11552, 4237, '夏洛特',             'C', 0, 1, 0, 2, 'Charlotte'),
  (11553, 4237, '圣安德鲁',           'S', 0, 1, 0, 3, 'Saint Andrew'),
  (11554, 4237, '圣戴维',             'S', 0, 1, 0, 4, 'Saint David'),
  (11555, 4237, '圣乔治',             'S', 0, 1, 0, 5, 'Saint George'),
  (11556, 4237, '格林纳丁斯',         'G', 0, 1, 0, 6, 'Grenadines');

-- ===== Grenada (4078) 6 教区, id 11561-11566 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11561, 4078, '圣乔治',             'S', 0, 1, 0, 1, 'Saint George'),
  (11562, 4078, '圣安德鲁',           'S', 0, 1, 0, 2, 'Saint Andrew'),
  (11563, 4078, '圣大卫',             'S', 0, 1, 0, 3, 'Saint David'),
  (11564, 4078, '圣约翰',             'S', 0, 1, 0, 4, 'Saint John'),
  (11565, 4078, '圣马克',             'S', 0, 1, 0, 5, 'Saint Mark'),
  (11566, 4078, '圣帕特里克',         'S', 0, 1, 0, 6, 'Saint Patrick');

-- ===== Antigua and Barbuda (4004) 7 教区/岛, id 11571-11577 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11571, 4004, '圣约翰',             'S', 0, 1, 0, 1, 'Saint John'),
  (11572, 4004, '圣保罗',             'S', 0, 1, 0, 2, 'Saint Paul'),
  (11573, 4004, '圣彼得',             'S', 0, 1, 0, 3, 'Saint Peter'),
  (11574, 4004, '圣腓力',             'S', 0, 1, 0, 4, 'Saint Philip'),
  (11575, 4004, '圣乔治',             'S', 0, 1, 0, 5, 'Saint George'),
  (11576, 4004, '圣玛丽',             'S', 0, 1, 0, 6, 'Saint Mary'),
  (11577, 4004, '巴布达',             'B', 0, 1, 0, 7, 'Barbuda');

-- ===== Saint Kitts and Nevis (4120) 14 教区, id 11581-11594 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11581, 4120, '巴斯特尔',           'B', 0, 1, 0, 1,  'Basseterre'),
  (11582, 4120, '圣乔治-巴斯特尔',   'S', 0, 1, 0, 2,  'Saint George Basseterre'),
  (11583, 4120, '圣彼得',             'S', 0, 1, 0, 3,  'Saint Peter'),
  (11584, 4120, '基督教堂',           'C', 0, 1, 0, 4,  'Christ Church'),
  (11585, 4120, '圣保罗',             'S', 0, 1, 0, 5,  'Saint Paul Capisterre'),
  (11586, 4120, '圣安',               'S', 0, 1, 0, 6,  'Saint Anne Sandy Point'),
  (11587, 4120, '圣约翰',             'S', 0, 1, 0, 7,  'Saint John Capisterre'),
  (11588, 4120, '三一',               'T', 0, 1, 0, 8,  'Trinity Palmetto Point'),
  (11589, 4120, '圣詹姆斯',           'S', 0, 1, 0, 9,  'Saint James Windward'),
  (11590, 4120, '圣玛丽',             'S', 0, 1, 0, 10, 'Saint Mary Cayon'),
  (11591, 4120, '圣托马斯-米德岛',   'S', 0, 1, 0, 11, 'Saint Thomas Middle Island'),
  (11592, 4120, '尼维斯-查尔斯',     'C', 0, 1, 0, 12, 'Charlestown Nevis'),
  (11593, 4120, '尼维斯-圣保罗',     'S', 0, 1, 0, 13, 'Saint Paul Charlestown'),
  (11594, 4120, '尼维斯-圣托马斯',   'S', 0, 1, 0, 14, 'Saint Thomas Lowland');

-- ===== Dominica (4060) 10 教区, id 11601-11610 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11601, 4060, '罗索',               'R', 0, 1, 0, 1,  'Roseau'),
  (11602, 4060, '圣约翰',             'S', 0, 1, 0, 2,  'Saint John'),
  (11603, 4060, '圣安德鲁',           'S', 0, 1, 0, 3,  'Saint Andrew'),
  (11604, 4060, '圣大卫',             'S', 0, 1, 0, 4,  'Saint David'),
  (11605, 4060, '圣乔治',             'S', 0, 1, 0, 5,  'Saint George'),
  (11606, 4060, '圣约瑟',             'S', 0, 1, 0, 6,  'Saint Joseph'),
  (11607, 4060, '圣路加',             'S', 0, 1, 0, 7,  'Saint Luke'),
  (11608, 4060, '圣马克',             'S', 0, 1, 0, 8,  'Saint Mark'),
  (11609, 4060, '圣帕特里克',         'S', 0, 1, 0, 9,  'Saint Patrick'),
  (11610, 4060, '圣保罗',             'S', 0, 1, 0, 10, 'Saint Paul');

-- ===== Suriname (4207) 10 区, id 11621-11630 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11621, 4207, '帕拉马里博',         'P', 0, 1, 0, 1,  'Paramaribo'),
  (11622, 4207, '瓦尼卡',             'W', 0, 1, 0, 2,  'Wanica'),
  (11623, 4207, '尼克里',             'N', 0, 1, 0, 3,  'Nickerie'),
  (11624, 4207, '科罗尼',             'C', 0, 1, 0, 4,  'Coronie'),
  (11625, 4207, '萨拉马卡',           'S', 0, 1, 0, 5,  'Saramacca'),
  (11626, 4207, '康门维内',           'C', 0, 1, 0, 6,  'Commewijne'),
  (11627, 4207, '马罗韦讷',           'M', 0, 1, 0, 7,  'Marowijne'),
  (11628, 4207, '帕拉',               'P', 0, 1, 0, 8,  'Para'),
  (11629, 4207, '布罗科蓬多',         'B', 0, 1, 0, 9,  'Brokopondo'),
  (11630, 4207, '锡帕利威尼',         'S', 0, 1, 0, 10, 'Sipaliwini');

-- ===== Guyana (4094) 10 区, id 11631-11640 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11631, 4094, '乔治敦',             'G', 0, 1, 0, 1,  'Georgetown'),
  (11632, 4094, '巴里马-瓦伊尼',     'B', 0, 1, 0, 2,  'Barima-Waini'),
  (11633, 4094, '波默龙-苏佩纳姆',   'P', 0, 1, 0, 3,  'Pomeroon-Supenaam'),
  (11634, 4094, '埃塞奎博群岛',       'E', 0, 1, 0, 4,  'Essequibo Islands-West Demerara'),
  (11635, 4094, '德梅拉拉-马哈卡',   'D', 0, 1, 0, 5,  'Demerara-Mahaica'),
  (11636, 4094, '马哈卡-伯比斯',     'M', 0, 1, 0, 6,  'Mahaica-Berbice'),
  (11637, 4094, '东伯比斯-科伦太因','E', 0, 1, 0, 7,  'East Berbice-Corentyne'),
  (11638, 4094, '上塔库图',           'U', 0, 1, 0, 8,  'Upper Takutu-Upper Essequibo'),
  (11639, 4094, '库尤尼-马扎鲁尼',   'C', 0, 1, 0, 9,  'Cuyuni-Mazaruni'),
  (11640, 4094, '波塔罗-锡帕鲁尼',   'P', 0, 1, 0, 10, 'Potaro-Siparuni');

-- ===== Maldives (4155) 7 行政区, id 11651-11657 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11651, 4155, '马累',               'M', 0, 1, 0, 1, 'Male'),
  (11652, 4155, '北部环礁',           'N', 0, 1, 0, 2, 'Northern'),
  (11653, 4155, '北中环礁',           'N', 0, 1, 0, 3, 'North Central'),
  (11654, 4155, '中环礁',             'C', 0, 1, 0, 4, 'Central'),
  (11655, 4155, '南中环礁',           'S', 0, 1, 0, 5, 'South Central'),
  (11656, 4155, '上南环礁',           'U', 0, 1, 0, 6, 'Upper South'),
  (11657, 4155, '南部环礁',           'S', 0, 1, 0, 7, 'Southern');

-- ===== Solomon Islands (4194) 9 省, id 11661-11669 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11661, 4194, '霍尼亚拉',           'H', 0, 1, 0, 1, 'Honiara'),
  (11662, 4194, '中央',               'C', 0, 1, 0, 2, 'Central'),
  (11663, 4194, '舒瓦瑟尔',           'C', 0, 1, 0, 3, 'Choiseul'),
  (11664, 4194, '瓜达尔卡纳尔',       'G', 0, 1, 0, 4, 'Guadalcanal'),
  (11665, 4194, '伊莎贝尔',           'I', 0, 1, 0, 5, 'Isabel'),
  (11666, 4194, '马基拉',             'M', 0, 1, 0, 6, 'Makira-Ulawa'),
  (11667, 4194, '马莱塔',             'M', 0, 1, 0, 7, 'Malaita'),
  (11668, 4194, '伦内尔-贝洛纳',     'R', 0, 1, 0, 8, 'Rennell and Bellona'),
  (11669, 4194, '泰莫图',             'T', 0, 1, 0, 9, 'Temotu');

-- ===== Vanuatu (4242) 6 省, id 11671-11676 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11671, 4242, '维拉港',             'P', 0, 1, 0, 1, 'Port Vila'),
  (11672, 4242, '马朗帕',             'M', 0, 1, 0, 2, 'Malampa'),
  (11673, 4242, '彭纳玛',             'P', 0, 1, 0, 3, 'Penama'),
  (11674, 4242, '萨纳玛',             'S', 0, 1, 0, 4, 'Sanma'),
  (11675, 4242, '谢法',               'S', 0, 1, 0, 5, 'Shefa'),
  (11676, 4242, '塔费阿',             'T', 0, 1, 0, 6, 'Tafea');

-- ===== Samoa (4244) 11 区, id 11681-11691 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11681, 4244, '阿皮亚',             'A', 0, 1, 0, 1,  'Apia'),
  (11682, 4244, '阿图阿',             'A', 0, 1, 0, 2,  'Atua'),
  (11683, 4244, '艾纳阿凯',           'A', 0, 1, 0, 3,  'Aiga-i-le-Tai'),
  (11684, 4244, '法萨莱莱阿加',       'F', 0, 1, 0, 4,  'Faasaleleaga'),
  (11685, 4244, '加加埃姆迦',         'G', 0, 1, 0, 5,  'Gagaemauga'),
  (11686, 4244, '加加伊福',           'G', 0, 1, 0, 6,  'Gagaifomauga'),
  (11687, 4244, '帕劳里',             'P', 0, 1, 0, 7,  'Palauli'),
  (11688, 4244, '萨蒂帕伊塔',         'S', 0, 1, 0, 8,  'Satupa\'itea'),
  (11689, 4244, '图阿马萨加',         'T', 0, 1, 0, 9,  'Tuamasaga'),
  (11690, 4244, '瓦伊瓦罗',           'V', 0, 1, 0, 10, 'Vaa-o-Fonoti'),
  (11691, 4244, '瓦伊西甘',           'V', 0, 1, 0, 11, 'Vaisigano');

-- ===== Tonga (4224) 5 区, id 11701-11705 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11701, 4224, '汤加塔布',           'T', 0, 1, 0, 1, 'Tongatapu'),
  (11702, 4224, '哈派',               'H', 0, 1, 0, 2, 'Haapai'),
  (11703, 4224, '瓦瓦乌',             'V', 0, 1, 0, 3, 'Vavau'),
  (11704, 4224, '埃瓦',               'E', 0, 1, 0, 4, 'Eua'),
  (11705, 4224, '尼瓦斯',             'N', 0, 1, 0, 5, 'Niuas');

-- ===== Kiribati (4118) 3 群岛, id 11711-11713 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11711, 4118, '吉尔伯特群岛',       'G', 0, 1, 0, 1, 'Gilbert Islands'),
  (11712, 4118, '凤凰群岛',           'P', 0, 1, 0, 2, 'Phoenix Islands'),
  (11713, 4118, '莱恩群岛',           'L', 0, 1, 0, 3, 'Line Islands');

-- ===== Marshall Islands (4143) 5 主要环礁, id 11721-11725 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11721, 4143, '马朱罗',             'M', 0, 1, 0, 1, 'Majuro'),
  (11722, 4143, '夸贾林',             'K', 0, 1, 0, 2, 'Kwajalein'),
  (11723, 4143, '埃贝',               'E', 0, 1, 0, 3, 'Ebeye'),
  (11724, 4143, '阿尔诺',             'A', 0, 1, 0, 4, 'Arno'),
  (11725, 4143, '贾卢伊特',           'J', 0, 1, 0, 5, 'Jaluit');

-- ===== Micronesia (4073) 4 州, id 11731-11734 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11731, 4073, '雅浦',               'Y', 0, 1, 0, 1, 'Yap'),
  (11732, 4073, '丘克',               'C', 0, 1, 0, 2, 'Chuuk'),
  (11733, 4073, '波纳佩',             'P', 0, 1, 0, 3, 'Pohnpei'),
  (11734, 4073, '科斯雷',             'K', 0, 1, 0, 4, 'Kosrae');

-- ===== Palau (4185) 16 州 (主要 6), id 11741-11746 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11741, 4185, '梅莱凯奥克',         'M', 0, 1, 0, 1, 'Melekeok'),
  (11742, 4185, '科罗尔',             'K', 0, 1, 0, 2, 'Koror'),
  (11743, 4185, '艾梅里克',           'A', 0, 1, 0, 3, 'Aimeliik'),
  (11744, 4185, '艾拉伊',             'A', 0, 1, 0, 4, 'Airai'),
  (11745, 4185, '佩莱里乌',           'P', 0, 1, 0, 5, 'Peleliu'),
  (11746, 4185, '安加尔',             'A', 0, 1, 0, 6, 'Angaur');

-- ===== Nauru (4169) 主要 3 区, id 11751-11753 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11751, 4169, '亚伦',               'Y', 0, 1, 0, 1, 'Yaren'),
  (11752, 4169, '梅嫩',               'M', 0, 1, 0, 2, 'Meneng'),
  (11753, 4169, '安纳',               'A', 0, 1, 0, 3, 'Anabar');

-- ===== Tuvalu (4227) 9 岛, id 11761-11769 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (11761, 4227, '富纳富提',           'F', 0, 1, 0, 1, 'Funafuti'),
  (11762, 4227, '南岛',               'N', 0, 1, 0, 2, 'Nanumea'),
  (11763, 4227, '努伊',               'N', 0, 1, 0, 3, 'Nui'),
  (11764, 4227, '努库费陶',           'N', 0, 1, 0, 4, 'Nukufetau'),
  (11765, 4227, '努库拉拉埃',         'N', 0, 1, 0, 5, 'Nukulaelae'),
  (11766, 4227, '尼塔基奥',           'N', 0, 1, 0, 6, 'Niutao'),
  (11767, 4227, '瓦伊图普',           'V', 0, 1, 0, 7, 'Vaitupu'),
  (11768, 4227, '南摩努',             'N', 0, 1, 0, 8, 'Nanumaga'),
  (11769, 4227, '努伊',               'N', 0, 1, 0, 9, 'Niulakita');
