-- ============================================================================
-- 14_provinces_europe_micro.sql — 欧洲剩余小国 + 海外属地
-- id 范围 10601-10810
-- ----------------------------------------------------------------------------
-- 父 id (来自 01_countries.sql):
--   IS 冰岛       = 4109  (8 区)
--   LU 卢森堡     = 4134  (12 县)
--   CY 塞浦路斯   = 4055  (6 区)
--   MT 马耳他     = 4153  (5 大区)
--   AD 安道尔     = 4001  (7 教区)
--   MC 摩纳哥     = 4138  (4 区)
--   LI 列支敦士登 = 4129  (11 镇)
--   SM 圣马力诺   = 4204  (9 镇)
--   VA 梵蒂冈     = 4236  (1)
--   XK 科索沃     = 4245  (7 区)
--   ME 黑山       = 4140  (24 市,主要 8)
--   FO 法罗群岛   = 4074  (6 区)
--   GL 格陵兰     = 4084  (5 市)
--   GI 直布罗陀   = 4083  (7 区)
--   IM 马恩岛     = 4104  (6 史丁)
--   JE 泽西       = 4111  (12 教区)
--   GG 根西岛     = 4081  (10 教区)
--   AX 奥兰群岛   = 4015  (6 主要市)
-- ============================================================================

-- ===== Iceland (4109) 8 区, id 10601-10608 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (10601, 4109, '首都区',             'C', 0, 1, 0, 1, 'Capital Region'),
  (10602, 4109, '南部半岛',           'S', 0, 1, 0, 2, 'Southern Peninsula'),
  (10603, 4109, '西部',               'W', 0, 1, 0, 3, 'Western Region'),
  (10604, 4109, '西峡湾',             'W', 0, 1, 0, 4, 'Westfjords'),
  (10605, 4109, '西北部',             'N', 0, 1, 0, 5, 'Northwestern Region'),
  (10606, 4109, '东北部',             'N', 0, 1, 0, 6, 'Northeastern Region'),
  (10607, 4109, '东部',               'E', 0, 1, 0, 7, 'Eastern Region'),
  (10608, 4109, '南部',               'S', 0, 1, 0, 8, 'Southern Region');

-- ===== Luxembourg (4134) 12 县, id 10611-10622 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (10611, 4134, '卢森堡市',           'L', 0, 1, 0, 1,  'Luxembourg City'),
  (10612, 4134, '埃施',               'E', 0, 1, 0, 2,  'Esch-sur-Alzette'),
  (10613, 4134, '卡佩伦',             'C', 0, 1, 0, 3,  'Capellen'),
  (10614, 4134, '梅尔施',             'M', 0, 1, 0, 4,  'Mersch'),
  (10615, 4134, '雷丹日',             'R', 0, 1, 0, 5,  'Redange'),
  (10616, 4134, '迪基希',             'D', 0, 1, 0, 6,  'Diekirch'),
  (10617, 4134, '克莱沃',             'C', 0, 1, 0, 7,  'Clervaux'),
  (10618, 4134, '维登',               'V', 0, 1, 0, 8,  'Vianden'),
  (10619, 4134, '维尔茨',             'W', 0, 1, 0, 9,  'Wiltz'),
  (10620, 4134, '埃希特纳赫',         'E', 0, 1, 0, 10, 'Echternach'),
  (10621, 4134, '格雷文马赫',         'G', 0, 1, 0, 11, 'Grevenmacher'),
  (10622, 4134, '雷米希',             'R', 0, 1, 0, 12, 'Remich');

-- ===== Cyprus (4055) 6 区, id 10631-10636 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (10631, 4055, '尼科西亚',           'N', 0, 1, 0, 1, 'Nicosia'),
  (10632, 4055, '利马索尔',           'L', 0, 1, 0, 2, 'Limassol'),
  (10633, 4055, '拉纳卡',             'L', 0, 1, 0, 3, 'Larnaca'),
  (10634, 4055, '法马古斯塔',         'F', 0, 1, 0, 4, 'Famagusta'),
  (10635, 4055, '帕福斯',             'P', 0, 1, 0, 5, 'Paphos'),
  (10636, 4055, '凯里尼亚',           'K', 0, 1, 0, 6, 'Kyrenia');

-- ===== Malta (4153) 5 大区, id 10641-10645 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (10641, 4153, '北部',               'N', 0, 1, 0, 1, 'Northern'),
  (10642, 4153, '中部',               'C', 0, 1, 0, 2, 'Central'),
  (10643, 4153, '南部',               'S', 0, 1, 0, 3, 'Southern'),
  (10644, 4153, '东南部',             'S', 0, 1, 0, 4, 'South Eastern'),
  (10645, 4153, '戈佐和科米诺',       'G', 0, 1, 0, 5, 'Gozo and Comino');

-- ===== Andorra (4001) 7 教区, id 10651-10657 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (10651, 4001, '老安道尔',           'A', 0, 1, 0, 1, 'Andorra la Vella'),
  (10652, 4001, '埃斯卡尔德斯',       'E', 0, 1, 0, 2, 'Escaldes-Engordany'),
  (10653, 4001, '恩坎普',             'E', 0, 1, 0, 3, 'Encamp'),
  (10654, 4001, '拉马萨纳',           'L', 0, 1, 0, 4, 'La Massana'),
  (10655, 4001, '奥尔迪诺',           'O', 0, 1, 0, 5, 'Ordino'),
  (10656, 4001, '卡尼略',             'C', 0, 1, 0, 6, 'Canillo'),
  (10657, 4001, '圣胡利娅',           'S', 0, 1, 0, 7, 'Sant Julia de Loria');

-- ===== Monaco (4138) 4 区, id 10661-10664 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (10661, 4138, '蒙特卡罗',           'M', 0, 1, 0, 1, 'Monte Carlo'),
  (10662, 4138, '摩纳哥城',           'M', 0, 1, 0, 2, 'Monaco-Ville'),
  (10663, 4138, '孔达米讷',           'C', 0, 1, 0, 3, 'La Condamine'),
  (10664, 4138, '丰特维耶',           'F', 0, 1, 0, 4, 'Fontvieille');

-- ===== Liechtenstein (4129) 11 镇, id 10671-10681 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (10671, 4129, '瓦杜兹',             'V', 0, 1, 0, 1,  'Vaduz'),
  (10672, 4129, '沙恩',               'S', 0, 1, 0, 2,  'Schaan'),
  (10673, 4129, '巴尔扎斯',           'B', 0, 1, 0, 3,  'Balzers'),
  (10674, 4129, '特里森',             'T', 0, 1, 0, 4,  'Triesen'),
  (10675, 4129, '埃申',               'E', 0, 1, 0, 5,  'Eschen'),
  (10676, 4129, '毛伦',               'M', 0, 1, 0, 6,  'Mauren'),
  (10677, 4129, '甘普林',             'G', 0, 1, 0, 7,  'Gamprin'),
  (10678, 4129, '鲁格尔',             'R', 0, 1, 0, 8,  'Ruggell'),
  (10679, 4129, '申贝格',             'S', 0, 1, 0, 9,  'Schellenberg'),
  (10680, 4129, '特里森贝格',         'T', 0, 1, 0, 10, 'Triesenberg'),
  (10681, 4129, '普兰肯',             'P', 0, 1, 0, 11, 'Planken');

-- ===== San Marino (4204) 9 镇, id 10691-10699 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (10691, 4204, '圣马力诺市',         'S', 0, 1, 0, 1, 'San Marino City'),
  (10692, 4204, '塞拉瓦莱',           'S', 0, 1, 0, 2, 'Serravalle'),
  (10693, 4204, '博尔戈马焦雷',       'B', 0, 1, 0, 3, 'Borgo Maggiore'),
  (10694, 4204, '多马尼亚诺',         'D', 0, 1, 0, 4, 'Domagnano'),
  (10695, 4204, '法埃塔诺',           'F', 0, 1, 0, 5, 'Faetano'),
  (10696, 4204, '菲奥伦蒂诺',         'F', 0, 1, 0, 6, 'Fiorentino'),
  (10697, 4204, '蒙泰贾尔迪诺',       'M', 0, 1, 0, 7, 'Montegiardino'),
  (10698, 4204, '阿夸维瓦',           'A', 0, 1, 0, 8, 'Acquaviva'),
  (10699, 4204, '基耶萨努奥瓦',       'C', 0, 1, 0, 9, 'Chiesanuova');

-- ===== Kosovo (4245) 7 区, id 10701-10707 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (10701, 4245, '普里什蒂纳',         'P', 0, 1, 0, 1, 'Prishtina'),
  (10702, 4245, '普里兹伦',           'P', 0, 1, 0, 2, 'Prizren'),
  (10703, 4245, '佩奇',               'P', 0, 1, 0, 3, 'Peja'),
  (10704, 4245, '米特罗维察',         'M', 0, 1, 0, 4, 'Mitrovica'),
  (10705, 4245, '贾科维察',           'G', 0, 1, 0, 5, 'Gjakova'),
  (10706, 4245, '吉拉内',             'G', 0, 1, 0, 6, 'Gjilan'),
  (10707, 4245, '费里扎伊',           'F', 0, 1, 0, 7, 'Ferizaj');

-- ===== Montenegro (4140) 8 主要市, id 10711-10718 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (10711, 4140, '波德戈里察',         'P', 0, 1, 0, 1, 'Podgorica'),
  (10712, 4140, '采蒂涅',             'C', 0, 1, 0, 2, 'Cetinje'),
  (10713, 4140, '尼克希奇',           'N', 0, 1, 0, 3, 'Niksic'),
  (10714, 4140, '普列夫利亚',         'P', 0, 1, 0, 4, 'Pljevlja'),
  (10715, 4140, '比耶洛波列',         'B', 0, 1, 0, 5, 'Bijelo Polje'),
  (10716, 4140, '巴尔',               'B', 0, 1, 0, 6, 'Bar'),
  (10717, 4140, '布德瓦',             'B', 0, 1, 0, 7, 'Budva'),
  (10718, 4140, '科托尔',             'K', 0, 1, 0, 8, 'Kotor');

-- ===== Faroe Islands (4074) 6 区, id 10721-10726 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (10721, 4074, '托尔斯港',           'T', 0, 1, 0, 1, 'Torshavn'),
  (10722, 4074, '克拉克斯维克',       'K', 0, 1, 0, 2, 'Klaksvik'),
  (10723, 4074, '霍伊维克',           'H', 0, 1, 0, 3, 'Hoyvik'),
  (10724, 4074, '阿尔加迪',           'A', 0, 1, 0, 4, 'Argir'),
  (10725, 4074, '武格',               'V', 0, 1, 0, 5, 'Vagar'),
  (10726, 4074, '埃斯图罗',           'E', 0, 1, 0, 6, 'Eysturoy');

-- ===== Greenland (4084) 5 市, id 10731-10735 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (10731, 4084, '努克',               'N', 0, 1, 0, 1, 'Nuuk'),
  (10732, 4084, '瑟米索克',           'S', 0, 1, 0, 2, 'Sermersooq'),
  (10733, 4084, '库雅雷克',           'K', 0, 1, 0, 3, 'Kujalleq'),
  (10734, 4084, '凯凯塔',             'Q', 0, 1, 0, 4, 'Qeqqata'),
  (10735, 4084, '阿瓦那',             'A', 0, 1, 0, 5, 'Avannaata');

-- ===== Gibraltar (4083) 7 区, id 10741-10747 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (10741, 4083, '直布罗陀城',         'G', 0, 1, 0, 1, 'Gibraltar Town'),
  (10742, 4083, '南区',               'S', 0, 1, 0, 2, 'South District'),
  (10743, 4083, '西区',               'W', 0, 1, 0, 3, 'West Side'),
  (10744, 4083, '东区',               'E', 0, 1, 0, 4, 'East Side'),
  (10745, 4083, '北区',               'N', 0, 1, 0, 5, 'North District'),
  (10746, 4083, '上岩石',             'U', 0, 1, 0, 6, 'Upper Rock'),
  (10747, 4083, '欧洲岬',             'E', 0, 1, 0, 7, 'Europa Point');

-- ===== Isle of Man (4104) 6 主要史丁, id 10751-10756 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (10751, 4104, '道格拉斯',           'D', 0, 1, 0, 1, 'Douglas'),
  (10752, 4104, '艾尔',               'A', 0, 1, 0, 2, 'Ayre'),
  (10753, 4104, '加尔夫',             'G', 0, 1, 0, 3, 'Garff'),
  (10754, 4104, '迈克尔',             'M', 0, 1, 0, 4, 'Michael'),
  (10755, 4104, '格拉巴',             'G', 0, 1, 0, 5, 'Glenfaba'),
  (10756, 4104, '米德尔',             'M', 0, 1, 0, 6, 'Middle');

-- ===== Jersey (4111) 12 教区, id 10761-10772 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (10761, 4111, '圣赫利尔',           'S', 0, 1, 0, 1,  'Saint Helier'),
  (10762, 4111, '圣布雷拉德',         'S', 0, 1, 0, 2,  'Saint Brelade'),
  (10763, 4111, '圣劳伦斯',           'S', 0, 1, 0, 3,  'Saint Lawrence'),
  (10764, 4111, '圣彼得',             'S', 0, 1, 0, 4,  'Saint Peter'),
  (10765, 4111, '圣乌恩',             'S', 0, 1, 0, 5,  'Saint Ouen'),
  (10766, 4111, '圣玛丽',             'S', 0, 1, 0, 6,  'Saint Mary'),
  (10767, 4111, '圣约翰',             'S', 0, 1, 0, 7,  'Saint John'),
  (10768, 4111, '圣三一',             'T', 0, 1, 0, 8,  'Trinity'),
  (10769, 4111, '圣马丁',             'S', 0, 1, 0, 9,  'Saint Martin'),
  (10770, 4111, '格鲁维尔',           'G', 0, 1, 0, 10, 'Grouville'),
  (10771, 4111, '圣塞维耶',           'S', 0, 1, 0, 11, 'Saint Saviour'),
  (10772, 4111, '圣克莱门特',         'S', 0, 1, 0, 12, 'Saint Clement');

-- ===== Guernsey (4081) 10 教区, id 10781-10790 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (10781, 4081, '圣彼得港',           'S', 0, 1, 0, 1,  'Saint Peter Port'),
  (10782, 4081, '圣桑普森',           'S', 0, 1, 0, 2,  'Saint Sampson'),
  (10783, 4081, '瓦尔',               'V', 0, 1, 0, 3,  'Vale'),
  (10784, 4081, '卡斯特尔',           'C', 0, 1, 0, 4,  'Castel'),
  (10785, 4081, '圣萨维奥',           'S', 0, 1, 0, 5,  'Saint Saviour'),
  (10786, 4081, '圣彼得',             'S', 0, 1, 0, 6,  'Saint Peter'),
  (10787, 4081, '托尔特瓦尔',         'T', 0, 1, 0, 7,  'Torteval'),
  (10788, 4081, '森林',               'F', 0, 1, 0, 8,  'Forest'),
  (10789, 4081, '圣马丁',             'S', 0, 1, 0, 9,  'Saint Martin'),
  (10790, 4081, '圣安德鲁',           'S', 0, 1, 0, 10, 'Saint Andrew');

-- ===== Aland Islands (4015) 6 主要市, id 10791-10796 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (10791, 4015, '玛丽港',             'M', 0, 1, 0, 1, 'Mariehamn'),
  (10792, 4015, '延德',               'J', 0, 1, 0, 2, 'Jomala'),
  (10793, 4015, '芬斯特伦',           'F', 0, 1, 0, 3, 'Finstrom'),
  (10794, 4015, '萨尔特维克',         'S', 0, 1, 0, 4, 'Saltvik'),
  (10795, 4015, '亨马兰',             'H', 0, 1, 0, 5, 'Hammarland'),
  (10796, 4015, '伦帕兰',             'L', 0, 1, 0, 6, 'Lemland');

-- ===== Vatican City (4236) 1 (整体), id 10805 =====
INSERT INTO `phpyun_city_class` (`id`,`keyid`,`name`,`letter`,`code`,`display`,`sitetype`,`sort`,`e_name`) VALUES
  (10805, 4236, '梵蒂冈城',           'V', 0, 1, 0, 1, 'Vatican City');
