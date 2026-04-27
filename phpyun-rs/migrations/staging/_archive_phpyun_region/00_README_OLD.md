# 区域数据扩充 — Staging SQL 文件清单

**位置:** `/www/wwwroot/zzzz.com/phpyun-rs/migrations/staging/`
**状态:** 待审核,**未写入数据库**。审核完成后由你决定执行顺序与时机。

## 文件用途

| 文件 | 内容 | 行数 | 影响表 |
|---|---|---|---|
| `01_alter_phpyun_region.sql` | 给 phpyun_region 加 `letter` (英文首字母) + `phone_code` (国际区号) 两列 + 索引 | 7 | DDL |
| `02_countries_letter_and_phone.sql` | 250 个国家全量 UPDATE 填充 letter + phone_code | 250 | phpyun_region |
| `03_countries_zh_i18n.sql` | 250 个国家中文翻译 INSERT(`kind=region`,`lang=zh-CN`) | 1 大 SQL,250 条数据行 | phpyun_dict_i18n |
| `04_provinces_russia.sql` | 俄罗斯 85 联邦主体(双语+letter)| ~170 | phpyun_region + phpyun_dict_i18n |
| `05_provinces_americas.sql` | 巴西 27 + 墨西哥 32 + 阿根廷 24(双语+letter)| ~170 | 同上 |
| `06_provinces_europe.sql` | 意大利 20 + 西班牙 19 + 波兰 16 + 荷兰 12(双语+letter)| ~140 | 同上 |
| `07_provinces_mideast_africa.sql` | 土耳其 81 + 沙特 13 + 南非 9 + 埃及 27 + 尼日利亚 37(双语+letter)| ~330 | 同上 |

**共计追加:**
- 国家级 letter + phone_code: 250 行 UPDATE
- 国家级中文翻译: 250 条 INSERT
- 省级新增: ~410 行 INSERT(分布在俄/巴/墨/阿/意/西/波/荷/土/沙/南非/埃及/尼日利亚)
- 省级中文翻译: ~410 条 INSERT

## 已经存在的省级数据(无需补)

```
US 51 ✅  JP 47 ✅  CN 34 ✅  KR 17 ✅
DE 16 ✅  MY 16 ✅  CA 13 ✅  FR 13(法国本土,海外大区可后补)
AU 8 ✅   GB 4 ✅   SG 5 ✅
```

## 仍未补全(下一阶段需要)

```
IN 已 16 / 完整 36 — 缺 20 个邦/中央直辖区
TH 已  8 / 完整 77 — 缺 69 个府(数量大)
VN 已  5 / 完整 63 — 缺 58 个省
ID 已  7 / 完整 38 — 缺 31 个省
PH 缺      / 完整 17 — 整套缺
PK 缺      / 完整 8  — 整套缺
```

如果你审过这一批后认可格式,我下一批补这 6 个国家。

## 执行顺序(等你确认后)

```bash
# 务必按 01 → 07 顺序,01 必须先跑(加列)否则后面 UPDATE 会失败
mysql phpyun < migrations/staging/01_alter_phpyun_region.sql
mysql phpyun < migrations/staging/02_countries_letter_and_phone.sql
mysql phpyun < migrations/staging/03_countries_zh_i18n.sql
mysql phpyun < migrations/staging/04_provinces_russia.sql
mysql phpyun < migrations/staging/05_provinces_americas.sql
mysql phpyun < migrations/staging/06_provinces_europe.sql
mysql phpyun < migrations/staging/07_provinces_mideast_africa.sql
```

## 数据格式说明

### 国家级 (level=0)

| 字段 | 例 | 说明 |
|---|---|---|
| code | `CN` | ISO 3166-1 alpha-2 |
| name | `China` | 默认英文名 |
| letter | `C` | 英文首字母,前端按字母分组用 |
| phone_code | `86` | 国际区号(不含 + 号),北美统一为 1 但加州市码如 `1264` 表示 +1-264 |
| continent | `AS` | AF/AN/AS/EU/NA/OC/SA |
| 中文 | `中国` | 走 phpyun_dict_i18n,kind=region/lang=zh-CN |

### 省级 (level=1)

| 字段 | 例 | 说明 |
|---|---|---|
| parent_id | (CN 的 region.id) | 通过 SELECT 子查询解析,不写死 |
| code | `CN-BJ` | ISO 3166-2 |
| name | `北京` 或 `California` | 中国行直接用中文,其他用英文 |
| letter | `B` 或 `C` | 英文首字母(中国行也用英文首字母方便统一排序) |
| phone_code | `''` | 省级不填 |

### 重要约定

1. **幂等**: 所有 INSERT 用 `ON DUPLICATE KEY UPDATE`,任意脚本可重跑
2. **parent_id 不硬编码**: 用 `JOIN phpyun_region p ON p.code='CN' AND p.level=0` 动态解析
3. **letter 计算**: `UPPER(LEFT(name_en, 1))` —— 但有些国家名是固定写法(如 'United Kingdom' 取 U,'Saint Kitts' 取 S)
4. **港澳台**: 已在原迁移里 `level=0` 国家级,中文名为"中国香港/中国澳门/中国台湾"

## 你需要决定的事

1. ✅ **DDL 是否通过**(添加 letter + phone_code 列)
2. ✅ **letter 字母分组规则**是否合适(我用的是 ISO 英文名首字母,这是国际通用做法;如要按中文拼音首字母需另做)
3. ✅ **phone_code 格式**是否合适(`86` 不带 +;北美 `1` 共享,但加勒比小岛用 `1264`/`1284` 这类含区段的格式)
4. ✅ **中文名翻译**是否符合你站点习惯(如 "韩国" vs "南韩" vs "大韩民国")
5. ⏳ 其余 IN/TH/VN/ID/PH/PK 等需要补的国家,确认格式后我继续生成

## 如何快速核对

```sql
-- 看 250 个国家的 letter+phone_code 填充情况
SELECT letter, phone_code, code, name FROM phpyun_region WHERE level=0 ORDER BY letter, name LIMIT 30;

-- 看俄罗斯 85 个省
SELECT r.code, r.name AS en, dz.text AS zh FROM phpyun_region r
LEFT JOIN phpyun_dict_i18n dz ON dz.kind='region' AND dz.item_id=r.id AND dz.lang='zh-CN'
WHERE r.country_code='RU' AND r.level=1 ORDER BY r.sort;

-- 总数统计
SELECT level, COUNT(*) FROM phpyun_region WHERE status!=2 GROUP BY level;
SELECT lang, COUNT(*) FROM phpyun_dict_i18n WHERE kind='region' GROUP BY lang;
```
