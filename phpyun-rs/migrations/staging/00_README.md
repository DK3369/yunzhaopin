# 全球国家+省级数据 — Staging SQL(`phpyun_city_class` 表)

📂 **位置:** `/www/wwwroot/zzzz.com/phpyun-rs/migrations/staging/`
🗂️ **状态:** **未写入数据库**,等你审过再决定执行

## 表结构(沿用现有 `phpyun_city_class`,**不改表结构**)

```sql
CREATE TABLE `phpyun_city_class` (
  `id` int(11) NOT NULL,                            -- 手动分配
  `keyid` int(11) NOT NULL,                         -- 父级 id,0=国家
  `name` varchar(100) NOT NULL,                     -- 中文名
  `letter` varchar(1) NOT NULL,                     -- 英文首字母
  `code` int(3) NOT NULL COMMENT '手机号国家编码',  -- 国际区号(int(3) 限制最大 999)
  `display` int(1) NOT NULL,                        -- 1=显示,0=隐藏
  `sitetype` int(2) NOT NULL,                       -- 沿用现有惯例填 0
  `sort` int(11) NOT NULL DEFAULT '0',
  `e_name` varchar(255) DEFAULT NULL                -- 英文名
) ENGINE=MyISAM DEFAULT CHARSET=utf8;
```

## 文件清单

| 文件 | 内容 | id 区间 | 数据量 |
|---|---|---|---|
| `01_countries.sql` | 250 个国家(全部 ISO 3166-1) | 4001-4250 | 250 |
| `02_provinces_cn_us_jp_kr.sql` | 中国 34 + 美国 51 + 日本 47 + 韩国 17 | 5001-5317 | 149 |
| `03_provinces_europe.sql` | 英国 4 + 德国 16 + 法国 13 + 意大利 20 + 西班牙 19 + 波兰 16 + 荷兰 12 | 5401-5552 | 100 |
| `04_provinces_russia_americas.sql` | 俄罗斯 85 + 巴西 27 + 墨西哥 32 + 阿根廷 24 + 加拿大 13 | 5901-6413 | 181 |
| `05_provinces_seasia_oceania.sql` | 印度 36 + 印尼 38 + 马来 16 + 泰国 36(主要府) + 越南 30(主要省) + 菲律宾 17 + 巴基斯坦 8 + 新加坡 5 + 澳大利亚 8 | 6501-7308 | 194 |
| `06_provinces_mideast_africa.sql` | 土耳其 30(主要省) + 沙特 13 + 南非 9 + 埃及 27 + 尼日利亚 37 | 7401-7837 | 116 |
| `07_provinces_seasia_more.sql` | **东南亚补全** 文莱 4 + 柬埔寨 25 + 老挝 18 + 缅甸 15 + 东帝汶 13;**泰国补 39 完成 76 府**;**越南补 33 完成 63 省** | 7901-8133 | 147 |
| `08_provinces_southasia_mideast.sql` | **南亚** 孟加拉 8 + 斯里兰卡 9 + 尼泊尔 7 + 不丹 20;**中东** 阿富汗 34 + 伊朗 31 + 伊拉克 19 + 阿联酋 7 + 以色列 6 + 约旦 12 + 黎巴嫩 9 + 阿曼 11 + 科威特 6 + 卡塔尔 8 + 巴林 4 + 也门 22 + 叙利亚 14;**新西兰** 16 | 8201-8616 | 233 |
| `09_provinces_africa_more.sql` | 肯尼亚 47 + 摩洛哥 12 + 阿尔及利亚 30 + 埃塞俄比亚 11 + 坦桑尼亚 31 + 加纳 16 + 乌干达 4 + 安哥拉 18 + 莫桑比克 11 | 8701-8941 | 180 |
| `10_provinces_latam_more.sql` | 哥伦比亚 33 + 智利 16 + 秘鲁 26 + 委内瑞拉 24 + 厄瓜多尔 24 + 玻利维亚 9 + 古巴 16 + 多米尼加 32 + 巴拿马 13 + 哥斯达黎加 7 + 危地马拉 22 | 9001-9292 | 222 |
| `11_provinces_europe_more.sql` | 瑞士 26 + 奥地利 9 + 比利时 3 + 葡萄牙 20 + 希腊 13 + 捷克 14 + 匈牙利 20 + 罗马尼亚 42 + 乌克兰 27 + 瑞典 21 + 挪威 11 + 芬兰 19 + 丹麦 5 + 保加利亚 14 + 白俄罗斯 7 + 爱尔兰 12 | 9301-9662 | 282 |
| `12_provinces_greater_china_centralasia.sql` | 中国香港 18 + 中国澳门 8 + 中国台湾 22 + 朝鲜 11 + 蒙古 22 + 哈萨克 17 + 乌兹别克 14 + 吉尔吉斯 9 + 塔吉克 5 + 土库曼 6 + 格鲁吉亚 12 + 亚美尼亚 11 + 阿塞拜疆 12 + 爱沙尼亚 15 + 拉脱维亚 13 + 立陶宛 10 + 克罗地亚 21 + 阿尔巴尼亚 12 + 北马其顿 8 + 摩尔多瓦 8 + 塞尔维亚 6 + 波黑 3 + 斯洛文尼亚 12 + 斯洛伐克 8 | 9701-10118 | 281 |
| `13_provinces_caribbean_pacific_africa_more.sql` | 牙买加 14 + 特立尼达 9 + 海地 10 + 洪都拉斯 18 + 尼加拉瓜 17 + 萨尔瓦多 14 + 巴拉圭 18 + 乌拉圭 19 + 巴新 22 + 斐济 4 + 突尼斯 24 + 喀麦隆 10 + 科特迪瓦 14 + 塞内加尔 14 + 马达加斯加 11 + 刚果(金)14 + 津巴布韦 10 + 赞比亚 10 + 博茨瓦纳 10 + 纳米比亚 14 | 10201-10594 | 276 |
| `14_provinces_europe_micro.sql` | **欧洲彻底完成** — 冰岛 8 + 卢森堡 12 + 塞浦路斯 6 + 马耳他 5 + 安道尔 7 + 摩纳哥 4 + 列支敦士登 11 + 圣马力诺 9 + 梵蒂冈 1 + 科索沃 7 + 黑山 8 + 法罗群岛 6 + 格陵兰 5 + 直布罗陀 7 + 马恩岛 6 + 泽西 12 + 根西 10 + 奥兰 6 | 10601-10805 | 130 |

**总计:**
- 国家级 250 行
- 省级 2361 行
- **合计 2611 行新增,横跨 13 个 SQL 文件**

**地理覆盖:**
- 🌏 亚洲: 东亚 6(含港澳台/朝/蒙) + 东南亚 11(全部 ASEAN+TL) + 南亚 4 + 中东 15 + 中亚 5 + 高加索 3 = **44 国**
- 🌍 欧洲: 西/北/中/东欧 + 巴尔干 + 波罗的海 = **31 国**
- 🌎 美洲: 北美 3 + 中美洲 7 + 加勒比 3 + 南美 11 = **24 国**
- 🌍 非洲: 北非 + 东非 + 南非 + 西非 + 中非 = **20 国**
- 🌏 大洋洲: 澳/新/巴新/斐济 = **4 国**

**总计 123 国家有省/州级数据。**

## 字段填法(按 PHP 端 `app/include/cache.class.php` 用法)

| 字段 | 国家级 | 省级 |
|---|---|---|
| `id` | 4001-4250 | 5000+ 按国家区间 |
| `keyid` | **0** | 父国家的 id (例:中国 4048) |
| `name` | 中文名 | 中文名 |
| `letter` | 英文首字母 | 英文首字母 |
| `code` | 国际区号(int(3) 仅主码) | **0**(省级不填) |
| `display` | 1 | 1 |
| `sitetype` | 0 | 0 |
| `sort` | 1-250 按 ISO 字母序 | 1- 按重要度/字母序 |
| `e_name` | 英文名 | 英文名 |

## ⚠️ 重要注意

### `code` 字段是 int(3) 的限制

PHPYun 原表 `code` 字段是 `int(3)`,**最大只能存 999**。所以:
- 中国 86、英国 44、日本 81 等三位数没问题
- 北美统一 1 (美国/加拿大/巴哈马/百慕大/开曼/牙买加/多米尼克...全部存 `1`)
- 不能存 `1264`(巴巴多斯实际 +1-264)、`1284`(英属维京 +1-284)等带分机的小区段

如果你需要精确到分机,**需要 `ALTER TABLE phpyun_city_class MODIFY code VARCHAR(8)`**。我可以补一个 alter SQL。

### 现有 2 行(id 3409、3410)

数据库里现在有 `id=3409 美国 USA` + 1 条。你 **是否要清理掉**?三种处理:
1. **保留** —— 我新加的从 4001 起,与现有不冲突,但有 2 条孤儿数据
2. **DELETE 旧 2 条** —— `DELETE FROM phpyun_city_class WHERE id IN (3409,3410);`
3. **TRUNCATE 整表** —— 干净开始(确认这表没被其他数据引用)

### MyISAM 引擎

表是 MyISAM,不支持事务。数据有问题的话只能靠 `DELETE WHERE id BETWEEN 4001 AND 7999` 回滚。

## 已有的省级(我没动)

```
US 51 ✅  CN 34 ✅  JP 47 ✅  KR 17 ✅  DE 16 ✅
MY 16 ✅  CA 13 ✅  FR 13 ✅  AU 8 ✅   GB 4 ✅   SG 5 ✅
```
**注意:** 我重新写的版本走 id 4001+,跟旧 `phpyun_region` 完全独立(那个表是 Rust 重构用的,跟 `phpyun_city_class` 不冲突)。

## 这一批数据没写全的(后续可补)

```
🔸 TH 泰国    77 府,只写了 36 个常用
🔸 VN 越南    63 省,只写了 30 个常用
🔸 TR 土耳其  81 省,只写了 30 个常用
🔸 RU 一些自治区如 Komi-Permyak 已并入,有 2-3 个边角案例
```
其余地区如非洲剩下 50 国、加勒比、太平洋小岛等,**省级一般无意义**(国家本身就很小),按需求再加。

## 执行顺序(等你确认后)

```bash
# 进入 mysql,从配置读凭据
cd /www/wwwroot/zzzz.com/phpyun-rs
mysql phpyun < migrations/staging/01_countries.sql
mysql phpyun < migrations/staging/02_provinces_cn_us_jp_kr.sql
mysql phpyun < migrations/staging/03_provinces_europe.sql
mysql phpyun < migrations/staging/04_provinces_russia_americas.sql
mysql phpyun < migrations/staging/05_provinces_seasia_oceania.sql
mysql phpyun < migrations/staging/06_provinces_mideast_africa.sql

# 验证总行数
mysql phpyun -e "SELECT keyid=0 AS is_country, COUNT(*) FROM phpyun_city_class GROUP BY keyid=0;"
# 期望: keyid=0 251 (250 + 旧的 2 条减 1 重复? 实际看现有数据);  其它 ~740
```

## 验证 SQL(执行后用)

```sql
-- 看国家列表(按字母分组)
SELECT letter, COUNT(*) FROM phpyun_city_class WHERE keyid=0 GROUP BY letter ORDER BY letter;

-- 抽中国所有省
SELECT id, name, letter, e_name, sort FROM phpyun_city_class WHERE keyid=4048 ORDER BY sort;

-- 抽美国所有州
SELECT id, name, letter, e_name, sort FROM phpyun_city_class WHERE keyid=4233 ORDER BY sort;

-- 看 phone code 是否都对
SELECT name, code FROM phpyun_city_class WHERE keyid=0 AND code IN (1,86,44,81,49,33) ORDER BY code, name;
```

## 你需要决定的事

1. ✅ **是否要清理旧的 2 条**(id 3409/3410)?
2. ✅ **`code` 字段要不要改 varchar(8)**?(为了支持 +1-264 这种带分机)
3. ✅ **泰国/越南/土耳其的剩余省份**要不要继续补?
4. ✅ **letter 用英文首字母** OK 吗?(如要中文拼音首字母,中国 = Z,需另算)
5. ✅ **格式 OK** → 我可以把 6 个文件按顺序灌入数据库

---

## 旧版本(phpyun_region 双表方案)

我之前生成的 phpyun_region + phpyun_dict_i18n 双表方案放在 `_archive_phpyun_region/` 目录,如果你只是想审视/对比可以参考,本次确认用 `phpyun_city_class` 单表方案。
