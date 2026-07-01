# 货币符号与 lcCoin 多语言设计计划

## 1. 目标

货币体系与默认语言是一体的：项目默认 locale 使用 `en_US`，默认货币使用 `USD`。没有显式 locale 或 currency 时，统一按英文美元格式输出，例如 `$1,234.56`。`zh`、`ph` 作为业务 locale key 支持，分别绑定 `CNY`、`PHP`。

为项目增加统一货币配置，按 locale 驱动金额展示。金额在数据库中统一存整数最小单位，例如 `price_cent = 123456` 表示 `1234.56`。数据库仍存货币 code，例如 `currency_code = USD`，不存 locale。页面展示不直接拼接符号，而是通过多语言货币函数 `lcCoin` 统一处理。

## 2. 配置文件建议

新增 JSON 配置文件，建议路径：

```text
config/currency.json
```

初始 JSON 示例按 locale 组织。字段名按项目约定保留 `defaul` 和 `loc`：

```json
{
  "defaul": "en_US",
  "loc": {
    "en_US": {
      "currency": "USD",
      "symbol": "$",
      "nameKey": "coin.usd.name",
      "minorUnit": 2,
      "symbolPosition": "before",
      "space": false,
      "decimal": ".",
      "thousands": ","
    },
    "zh": {
      "currency": "CNY",
      "symbol": "￥",
      "nameKey": "coin.cny.name",
      "minorUnit": 2,
      "symbolPosition": "before",
      "space": false,
      "decimal": ".",
      "thousands": ","
    },
    "ph": {
      "currency": "PHP",
      "symbol": "₱",
      "nameKey": "coin.php.name",
      "minorUnit": 2,
      "symbolPosition": "before",
      "space": false,
      "decimal": ".",
      "thousands": ","
    }
  }
}
```

字段说明：

- `defaul`：默认 locale key，固定为 `en_US`。
- `loc`：locale 配置集合，key 使用项目业务 locale，例如 `en_US`、`zh`、`ph`。
- `loc.{locale}.currency`：该 locale 默认货币 code，例如 `USD`、`CNY`、`PHP`。
- `loc.{locale}.symbol`：该 locale 默认展示符号，例如 `$`、`￥`、`₱`。
- `loc.{locale}.nameKey`：货币名称语言包 key，例如 `coin.usd.name`。
- `loc.{locale}.minorUnit`：该货币小数位，USD/CNY/PHP 为 `2`。
- `loc.{locale}.symbolPosition`：符号位置，`before` 或 `after`。
- `loc.{locale}.space`：符号和金额之间是否加空格。
- `loc.{locale}.decimal` / `loc.{locale}.thousands`：该 locale 的小数点和千分位展示符号。

## 3. 数据库存储规则

所有金额字段统一新增或迁移为整数最小单位：

```sql
price_cent INT NOT NULL DEFAULT 0,
currency_code CHAR(3) NOT NULL DEFAULT 'USD'
```

规则：

- 禁止用 float/double 存钱。
- 禁止把 `$123.00`、`￥123.00`、`₱123.00` 这类展示字符串写入数据库。
- 老字段如 `price` 如果是主单位 decimal，迁移时转换为 `price_cent = ROUND(price * 100)`。
- 数据库仍存 `currency_code`，不存 locale。
- `currency_code` 为空时回退到当前 locale 的 `loc.{locale}.currency`；locale 也为空时回退 `loc[defaul].currency = USD`。
- 接口输出金额时同时给出 `price_cent`、`currency_code`，必要时给出后端格式化好的 `price_text`。

## 4. 内存加载方案

启动或首次调用时加载 `config/currency.json` 到内存缓存，避免每次格式化都读文件。

建议后端提供：

```php
function yun_coin_config($locale = '') {}
function lcCoin($amountCent, $currency = '', $options = array()) {}
```

加载规则：

1. 第一次调用 `yun_coin_config()` 时读取 JSON。
2. 解析后保存在静态变量或全局 `$coinConfig` 中。
3. 同一请求内重复调用直接读内存。
4. 后台清缓存时清理货币配置缓存。
5. JSON 解析失败时使用内置安全默认值：`defaul=en_US`，`currency=USD`，`symbol=$`，`minorUnit=2`。

## 5. lcCoin 输出规则

`lcCoin` 是统一展示入口，负责金额、货币符号和多语言环境差异。

推荐调用：

```php
echo lcCoin($row['price_cent'], $row['currency_code']);
echo lcCoin($row['price_cent']); // locale 和 currency 为空时默认 en_US/USD
```

解析规则：

- locale 为空：使用 `defaul`，即 `en_US`。
- currency 为空：使用 `loc[locale].currency`。
- 找不到 locale：回退 `loc[defaul]`。
- 找不到 currency 或 currency 与 locale 不匹配：仍按目标 locale 的配置输出。
- 默认显示符号，不显示 code。
- 管理后台关键财务页面可以显示 code，例如 `$1,234.56 USD`，避免 `$` 歧义。
- `minorUnit=0` 的货币不显示小数。
- 负数格式由 `lcCoin` 统一处理，例如 `-$1,234.56`。
- 不允许页面自行拼接 `$`、`￥`、`₱`。

期望输出：

```text
lcCoin(123456)                              => $1,234.56
lcCoin(123456, '', array('locale' => 'zh')) => ￥1,234.56
lcCoin(123456, '', array('locale' => 'ph')) => ₱1,234.56
```

可选参数：

```php
lcCoin(123456, '', array(
    'locale' => 'en_US',
    'showCode' => false,
    'showSymbol' => true,
    'trimZero' => false
));
```

## 6. 多语言语言包

货币名称进入语言包，不把名称写死在 JSON 中。默认展示语言为 `en_us`，必须先保证英文 key 完整，再补齐中文。

示例：

```php
// data/lang/en_us.php
'coin' => array(
    'usd' => array('name' => 'US Dollar'),
    'cny' => array('name' => 'Chinese Yuan'),
    'php' => array('name' => 'Philippine Peso')
);

// data/lang/zh_cn.php
'coin' => array(
    'usd' => array('name' => '美元'),
    'cny' => array('name' => '人民币'),
    'php' => array('name' => '菲律宾比索')
);
```

前端语言包可暴露：

```js
window.yunI18n.coin = {
  defaul: 'en_US',
  loc: {
    en_US: { currency: 'USD', symbol: '$', nameKey: 'coin.usd.name', minorUnit: 2 },
    zh: { currency: 'CNY', symbol: '￥', nameKey: 'coin.cny.name', minorUnit: 2 },
    ph: { currency: 'PHP', symbol: '₱', nameKey: 'coin.php.name', minorUnit: 2 }
  }
};
```

## 7. 前端函数设计

建议在 `js/yun-i18n.js` 增加：

```js
function lcCoin(amountCent, currency, options) {}
```

前端只格式化展示，不参与财务计算。接口返回金额时应返回：

```json
{
  "price_cent": 123456,
  "currency_code": "USD",
  "price_text": "$1,234.56"
}
```

页面优先使用后端返回的 `price_text`。只有纯前端计算预览时才调用 JS `lcCoin()`。

## 8. 迁移步骤

1. 新建 `config/currency.json`，默认 `defaul=en_US`，`loc.en_US.currency=USD`。
2. 新增后端 `yun_coin_config()` 和 `lcCoin()`。
3. 新增前端 `lcCoin()`，并在语言包接口中下发 `coin.defaul` 和 `coin.loc`。
4. 财务相关表逐步新增 `*_cent` 和 `currency_code DEFAULT 'USD'` 字段。
5. 老字段迁移为整数分，并保留回滚脚本。
6. 替换页面中的 `$`、`￥`、`₱`、`元` 等硬编码展示。
7. 管理后台财务页优先改造并人工核对金额。
8. 跑扫描脚本，确保无新增硬编码货币符号。

## 9. 验收规则

- 配置文件使用 `defaul` 和 `loc`，不再以顶层 `currencies` 作为主入口。
- 默认值一致：`defaul = en_US`，`loc.en_US.currency = USD`。
- 数据库金额字段不得新增 float/double。
- 数据库默认货币字段为 `currency_code DEFAULT 'USD'`。
- 数据库仍存货币 code，不存 locale。
- 页面不得直接拼接货币符号。
- 示例输出正确：`lcCoin(123456)` 为 `$1,234.56`，locale `zh` 为 `￥1,234.56`，locale `ph` 为 `₱1,234.56`。
- 负数、0、小数位、千分位格式正确。
- 后台财务、套餐、支付、订单页面人工抽查。
- 多语言切换后货币名称变更，符号和金额不丢失。
