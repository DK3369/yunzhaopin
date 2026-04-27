# PHPYun WAP 接口规范（Rust 重写参考）

> 本文档用于把现有 PHP PHPYun 招聘系统的 WAP（移动网页）层重写为 Rust 后端。  
> 覆盖：**32 个 WAP 前台控制器（216 个 action）** + **WAP 会员中心**（个人 / 企业）。  
> 编写日期：2026-04-23。

---

## 目录

- [0. 架构与约定](#0-架构与约定)
- [1. 路由 & 入口](#1-路由--入口)
- [2. 鉴权 & 会话](#2-鉴权--会话)
- [3. WAP 前台控制器（32 个）](#3-wap-前台控制器)
  - [3.1 A–M](#31-a--m)
  - [3.2 N–Z](#32-n--z)
- [4. WAP 会员中心](#4-wap-会员中心)
  - [4.1 个人求职者 (usertype=1)](#41-个人求职者-usertype1)
  - [4.2 企业招聘 (usertype=2)](#42-企业招聘-usertype2)
- [5. Rust 实现建议](#5-rust-实现建议)

---

## 0. 架构与约定

### 技术栈映射

| PHP（现状） | Rust（目标） |
|---|---|
| Smarty 模板 | [maud](https://maud.lambda.xyz/) / [askama](https://github.com/djc/askama) / SSR by Leptos / 或仅提供 JSON 给 SPA |
| MySQL / mysqli | [sqlx](https://github.com/launchbadge/sqlx) (MySQL + 编译期 SQL 校验) |
| Session ($_SESSION) | Redis + [axum-sessions](https://crates.io/crates/axum-sessions) |
| $_COOKIE | HttpOnly / Secure / SameSite 规范 Cookie |
| $_POST / $_GET 混合 | 明确 `Query<T>` vs `Form<T>` vs `Json<T>` |
| md5(md5($pw)+salt) 密码 | argon2id（登录时惰性升级旧 md5 密码）|
| 返回 HTML | axum handlers，按路由组使用不同 Response Type |

### 响应约定

PHPYun 的响应有多种风格，Rust 重写时应**归一**：

| 原风格 | 建议统一成 |
|---|---|
| `layer_msg($msg, $st, $type, $url)` → JS-aware JSON | `{"code": 0, "msg": "", "data": {...}}` |
| `render_json($error, $msg, $data)` | 同上 |
| `echo 0/1/2/3` | 改为 JSON（业务码） |
| `echo 'document.write(xxx)'` | 改为 JSON |
| Template → HTML | Server-render 或前后端分离 |

---

## 1. 路由 & 入口

### 前台 WAP

```
入口：/wap/index.php
路由：c={controller} & a={action}
示例：/wap/index.php?c=job&a=show&id=123
```

伪静态规则（已在 nginx 中配置）：

```
/job/list/{params}.html  → /wap/index.php?c=job&a=list&...
/job/{id}.html           → /wap/index.php?c=job&a=show&id={id}
```

### WAP 会员中心

```
入口：/wap/member/index.php  (m=wap_member)
路由：c={模块}&a={action}
示例：/wap/member/index.php?c=index&a=resume
```

> 与 PC 会员中心 `/member/index.php?c=X&act=Y` 的差异：  
> - PC 用 `act`，WAP 用 `a`  
> - PC 模板在 `app/template/default`，WAP 在 `app/template/wap/member`

---

## 2. 鉴权 & 会话

### 前台 Cookie（PC/WAP 共用）

| Cookie | 含义 | Rust 对应 |
|---|---|---|
| `uid` | 会员 ID | JWT payload 中的 `sub` |
| `shell` | md5(username+password+salt) | JWT signed with server secret |
| `usertype` | 1=个人, 2=企业, 0=未选身份 | JWT payload `role` |
| `userdid` | 分站 ID | JWT payload `did` |
| `amtype` | 是否后台切换登录 | JWT payload `admin_impersonate` |

### Rust 鉴权中间件伪代码

```rust
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // 1. 从 Authorization: Bearer <jwt> 或 Cookie 提取 token
    let token = extract_token(&req)?;
    // 2. 校验 JWT 签名 + 过期时间
    let claims: Claims = jsonwebtoken::decode(&token, &state.jwt_key, &Validation::default())?
        .claims;
    // 3. 可选：对照 DB 校验 shell（兼容老 Cookie）
    //    shell == md5(username + password + salt)
    // 4. 注入到 request extensions
    req.extensions_mut().insert(AuthUser {
        uid: claims.uid,
        usertype: claims.usertype,
    });
    Ok(next.run(req).await)
}
```

### 入参校验（适用于全部 `/v1/*` 接口）

> **核心原则**：用户输入永远不能裸进数据库。无论是 Path / Query / Json / Form，都必须先过类型 + `validator::Validate`，再走业务逻辑、再到 DB。

#### 校验提取器（按 transport 选）

| 提取器 | 来源 | 适用 transport | 是否运行 `Validate` |
|---|---|---|---|
| `ValidatedJson<T>` | `phpyun_core::ValidatedJson` | `application/json` POST/PUT body | ✅ |
| `ValidatedQuery<T>` | `phpyun_core::ValidatedQuery` | URL query string | ✅ |
| `ValidatedForm<T>` | `phpyun_core::ValidatedForm` | `application/x-www-form-urlencoded` | ✅ |
| `Path<u64>` 等 | axum 内置 | 路径参数 | 类型层（u64 自然拒绝负数 / 非整数） |

#### 字段级约束清单（强制）

| 字段类型 | 默认必备约束 | 例 |
|---|---|---|
| 自由文本（`keyword`/`name`/`title`） | `length(min=N, max=M)` —— 至少给 max（建议 ≤ 100 chars） | `#[validate(length(max = 100))]` |
| ID / 枚举码 | `range(min, max)` —— 即便 `i32` 已类型隔离仍要给业务上限 | `#[validate(range(min = 0, max = 9_999_999))]` |
| 0/1 标志位 | `range(min = 0, max = 1)` | `is_rec`, `is_hot` 等 |
| 分页 limit | `range(min = 1, max = 100)`（或更小，按业务调整）| `#[validate(range(min = 1, max = 100))]` |
| 槽位 / 标识符 | 仅字母/数字/下划线/连字符 + 长度上限 | 见 `wap/ads.rs::validate_slot_charset` |
| 时间戳 | `range`（避免 `i64::MIN..=i64::MAX` 直进 SQL）| 视字段含义 |

#### 检查点

1. **Path 参数**：默认走 `Path<u64>`，sqlx bind 直接传整数；不要 `Path<String>` 后 format!() 拼 SQL。
2. **Query 参数**：所有可能直进 `LIKE`/`WHERE`/`IN` 的字段必须经 `ValidatedQuery`，不能裸 `Query<T>`。
3. **Json body**：所有变更类接口（POST/PATCH）必须 `ValidatedJson<T>` + `T: Validate`；禁止 `axum::Json<json::Value>` 收任意 JSON 后裸取字段（除非随后立即 `serde_json::from_value::<T>` 并 `.validate()`，见 `wap/once.rs::update` 的合理用法）。
4. **service 层守卫**：admin 类同时叠 `user.require_admin()?`；幅值类（如 `is_rec` ∈ {0,1}）在 service 入口再做一次 `range` 校验，handler 拒不下毒到达不了 service，但 service 也不轻信 handler。
5. **SQL bind**：永远用占位符 `?`，绝不 `format!("WHERE x = {}")`；用户字符串只能进 `LIKE %?%` 的参数位。

#### 已落地接口（截至本版本）

| 接口 | 校验项 |
|---|---|
| `GET /v1/wap/jobs` | `keyword` ≤ 100；`*_id` ∈ 0..=9_999_999；`min/max_salary` ∈ 0..=1_000_000；`exp/edu` ∈ 0..=99；`job_type` ∈ 0..=4 |
| `GET /v1/wap/jobs/{id}/similar`、`/same-company` | `limit` ∈ 1..=30 |
| `GET /v1/wap/companies` | `keyword` ≤ 100；id 字段 ∈ 0..=9_999_999 |
| `GET /v1/wap/companies/autocomplete` | `keyword` ∈ 1..=100；`limit` ∈ 1..=20 |
| `GET /v1/wap/companies/hot` | `order` ≤ 16；`limit` ∈ 1..=100 |
| `GET /v1/wap/resumes` | `keyword` ≤ 100；`education` ∈ 0..=99；`sex/marriage` ∈ 0..=9 |
| `GET /v1/wap/articles` | `category` ≤ 64；`keyword` ≤ 100 |
| `GET /v1/wap/qna/*` | `keyword` ≤ 100；`category_id` ∈ 0..=9_999_999；`days` ∈ 1..=365；`limit` 各处按业务上限 |
| `GET /v1/wap/search` | `kw` ∈ 1..=100；`scope` ∈ 1..=16 |
| `GET /v1/wap/ads` | `slot` 字符集 `[A-Za-z0-9_-]{1,64}`；`limit` ∈ 1..=100 |
| `GET /v1/mcenter/messages` | `category` ≤ 64 |
| `GET /v1/mcenter/search-history` | `scope` ≤ 16；`limit` ∈ 1..=200 |
| `GET /v1/mcenter/my-views` / `profile-views` | `kind` ∈ 1..=3 |
| `POST /v1/admin/rewards/{id}/flags` | `is_rec/is_hot` ∈ 0..=1（handler + service 双重校验）|

#### CI 强制项

- `crates/handlers/tests/new_endpoints.rs::high_risk_search_handlers_use_validated_query`：扫描源文件，任意搜索/列表 Query 结构体（如 `JobListQuery`、`SearchQuery`、`AdQuery`）若被裸 `Query<...>` 提取（而非 `ValidatedQuery<...>`），即 CI 失败。
- 同文件 `every_admin_route_declares_bearer_security`：admin 接口 OpenAPI 必须声明 bearer 安全方案。

#### 验证证据

```
$ curl /v1/wap/jobs?keyword=$(python3 -c 'print("x"*200)')
{"code":400,"msg":"param_invalid: validation.length"}

$ curl /v1/wap/ads?slot=abc%20def
{"code":400,"msg":"param_invalid: validation.slot_charset"}

$ curl /v1/wap/search?kw=
{"code":400,"msg":"param_invalid: validation.length"}

$ curl /v1/wap/qna/hotweek?limit=999
{"code":400,"msg":"param_invalid: validation.range"}
```

### 三层鉴权（仅适用于 `/v1/admin/*`）

> 管理后台是高权重攻击面，**必须**走三层独立鉴权；缺任一层都按事故处理。

| 层 | 位置 | 作用 |
|---|---|---|
| L1 路由层中间件 | `phpyun_core::admin_guard::layer` 挂在 `nest("/admin", ...)` 上 | 进入任意 `/v1/admin/*` 路由前先做：①Bearer 解析 + Cookie token 回退；②JWT 验签 / `exp` 校验；③`jti` 黑名单查询；④`pw_epoch` 失效检查；⑤`usertype != 3` 一律 403 RoleMismatch。失败请求 **不会接触** handler / DB / Redis 业务路径 |
| L2 服务层守卫 | 每个 `*_service::admin_*` 入口首行 `user.require_admin()?` | 即使 handler 直接被其它内部代码调用（绕过 router 时），仍在业务边界拦截。审计上保证只有 `usertype=3` 才能调用 admin 服务 |
| L3 Handler 层守卫 | 每个 admin handler 函数体首行 `user.require_admin()?` | 防御性写法 + 审计可读性。CI 静态测试 `every_admin_route_declares_bearer_security` 强制要求 `#[utoipa::path]` 声明 `security(("bearer" = []))`；漏写一处即测试失败 |

#### 失败响应矩阵

| 触发条件 | HTTP | `code` | `msg`（i18n key） |
|---|---|---|---|
| 没有 Bearer 也没有 cookie token | 401 | 401 | `unauthenticated` |
| JWT 签名错 / 过期 / `jti` 在黑名单 / `iat < pw_epoch` | 401 | 401 | `session_expired` |
| Token 合法但 `usertype != 3` | 403 | 403 | `role_mismatch` |
| Idempotency-Key 重放命中 | 200 + `x-idempotent-replay: 1` | 缓存原 code | 缓存原 msg |

#### Token 强制条件（admin）

- 仅接受 access token（`token_type = access`）；refresh token 仅供续签 access
- access TTL 默认 ≤ 15 分钟；超时自动续签需走 `/v1/wap/auth/refresh`
- `pw_epoch`：管理员密码变更 / 重置 / 强制下线 → 立即让所有旧 token 失效
- `jti` 黑名单：每次显式登出（`/v1/wap/logout`）写入 Redis，TTL 等于 token 剩余有效期
- `XFF` / `XRI` 头：仅当 `peer_addr` 是 loopback / 私网时才信任，否则审计日志按真实 peer IP 记录（`phpyun_core::extractors::ClientIp` 在 `extractors.rs` 中已带单元测试覆盖各种伪造场景）

#### 高权重接口的额外护栏（除上述三层外）

| 接口 | 额外护栏 |
|---|---|
| `POST /v1/admin/account-merge` | service 层 `require_admin` + 写入 `audit_log`（`admin.account_merge`）+ 操作前后做 `phpyun_member` 双向幂等校验，避免重复合并 |
| `POST /v1/admin/site-settings` | 写入即触发 `phpyun_admin_config` 缓存失效；记录 actor_uid + ip + 旧值/新值 diff 到 `audit_log` |
| `GET /v1/admin/audit-log` | service 层强制 `require_admin`；不暴露给任何其他角色（即使是站长本人）|
| `POST /v1/admin/account-logouts/{id}/{approve\|reject}` | 通过则触发 `pw_epoch` 跳级 + 全 token 失效 + 异步审计 |
| `POST /v1/admin/usertype-changes/{id}/{approve\|reject}` | 同上，并落审计日志 |

#### CI 强制项

- `crates/handlers/tests/new_endpoints.rs::every_admin_route_declares_bearer_security` — 任意新加的 `/v1/admin/*` 路由必须在 `#[utoipa::path]` 写 `security(("bearer" = []))`，否则 CI 红
- `scripts/check-architecture.sh`（`pre-commit`）— 拦截 `crates/services/src/*_service.rs` 中的 admin_* 函数缺 `require_admin` 调用

### 路由守卫

| PHPYun 行为 | Rust 守卫 |
|---|---|
| `if ($this->uid=='') redirect to login` | `require_auth` middleware |
| `if ($this->usertype!=1) die` | `require_usertype(1)` |
| 未绑定身份时跳 `register/?c=ident` | 自定义错误码 `NEED_IDENT` |

---

## 3. WAP 前台控制器

## 3.1 A–M

---

### wap/advice.class.php — 意见反馈

#### `index_action` — 意见反馈页面
- **URL**: `GET /wap/index.php?c=advice&a=index`
- **Auth**: no
- **Returns**: Template `wap/advice`
- **Purpose**: 显示意见反馈表单页面

#### `saveadd_action` — 提交反馈
- **URL**: `POST /wap/index.php?c=advice&a=saveadd`
- **Auth**: no
- **Params**: `username`, `infotype`, `content`, `moblie`, `authcode`, `advice_code`
- **Model**: `advice.model.php::addInfo()`
- **Returns**: JSON
- **Purpose**: 提交意见反馈

#### `sendmsg_action` — 发送验证码
- **URL**: `POST /wap/index.php?c=advice&a=sendmsg`
- **Auth**: no
- **Params**: `authcode`, `moblie`
- **Model**: `notice.model.php::jycheck()`, `sendCode()`
- **Returns**: JSON
- **Purpose**: 发送短信验证码

---

### wap/ajax.class.php — AJAX API（48 个 action）

> 这是最复杂的控制器，几乎承载所有小交互。Rust 重写时建议拆成多个模块：`ajax_cert`（认证类）/`ajax_collect`（收藏类）/`ajax_data_viz`（数据可视化类）等。

#### 分类字典类
- `wap_job_action` — `POST`：根据一级分类返回二级职位 option。Params: `id`。Returns: HTML `<option>`
- `wap_city_action` — `POST`：根据一级城市返回二级。Params: `id`
- `getjob_action` — `POST`：职位分类树。Params: `id`, `type`。Returns: HTML list
- `getcity_action` — `POST`：城市分类树。Params: `id`, `type`, `kzq`
- `qclass_action` — `POST`：问答分类。Params: `id`。Returns: JSON
- `getredeem_action` — `POST`：积分商城分类。Params: `id`, `type`

#### 求职 / 招聘交互
- `sava_ajaxresume_action` — `POST` (usertype=2)：简历邀请面试。Model: `job.model.php::addYqmsInfo()`
- `forlink_action` — `POST`：查看简历联系方式（扣积分）。Params: `eid`。Model: `downresume.model.php::downResume()`
- `talentpool_action` — `POST`：加入人才库。Params: `eid`, `uid`
- `indexajaxresume_action` — `POST`：邀请面试（首页入口）
- `pl_action` — `POST` (usertype=1)：职位咨询留言。Params: `jobid`, `job_uid`, `content`, `com_name`, `job_name`, `authcode`
- `atn_action` — `POST`：收藏职位/简历。Params: `id`, `type`
- `atncompany_action` — `POST`：关注企业。Params: `id`

#### 验证码 / 认证
- `emailcert_action` — `POST`：发邮箱认证邮件。Params: `authcode`, `email`
- `mobliecert_action` — `POST`：发手机认证短信。Params: `code`, `str`
- `regcode_action` — `POST`：注册验证码。Params: `code`, `moblie`, `noblur`
- `regcodeks_action` — `POST`：快速投递验证码
- `temporaryresume_action` — `POST`：快速申请职位。Model: `userinfo.model.php::fastToudi()`

#### 账号操作
- `sign_action` — `POST`：每日签到赚积分。Params: `rand`
- `setpwd_action` — `POST`：改密码。Params: `password`, `passwordnew`, `passwordconfirm`
- `applytype_action` — `POST`：申请身份变更。Params: `applyusertype`, `applybody`
- `notuserout_action` — `POST`：切换账号。Params: `jobid`。Returns: plain text (redirect URL)

#### 招聘会
- `ajaxzphjob_action` — `POST` (usertype=2)：企业报名招聘会。Params: `jobid`, `id`, `zid`
- `ajaxComjob_action` — `POST` (usertype=2)：检查企业职位报名状态

#### 其他
- `getbusiness_action` — `POST`：天眼查企业工商信息。Params: `name`
- `msgNum_action` — `GET`：消息数统计 (需登录)
- `ajax_url_action` — `POST`：URL 参数构造。Params: `url`, `type`, `id`
- `ajax_day_action_check_action` — `POST` (usertype=2)：每日操作次数限制。Params: `type`
- `pubqrcode_action` — `GET`：公共二维码跳转。Params: `toc`, `toa`, `toid`, `totype`, `touid`。Returns: PNG
- `gzhqrcode_action` — `GET`：公众号关注二维码。Params: `token`。Returns: PNG
- `getJobHb_action` — `GET`：职位分享海报。Params: `hb`, `id`。Returns: HTML/Image
- `getComHb_action` — `GET`：企业分享海报。Params: `uid`, `hb`, `style`, `jobids`
- `getInviteRegHbList_action` — `GET`：邀请注册海报列表。Returns: JSON
- `getInviteRegHb_action` — `GET`：邀请注册海报。Params: `hb`, `uid`
- `addJobTelLog_action` — `POST`：记录查看联系方式。Params: `jobid`, `comid`

#### 数据可视化（实时统计）
- `dataShowIndex_action` — `GET`：数据展示首页
- `cityData_action` / `ageData_action` / `expData_action` / `sexData_action` / `eduData_action` — `GET`：城市/年龄/经验/性别/学历分布数据
- `userHyChart_action` / `userRegChart_action` — `GET`：用户活跃/注册趋势
- `comcityData_action` / `comgmData_action` / `comxzData_action` — `GET`：公司城市/规模/性质
- `comLogChart_action` / `comJobChart_action` — `GET`：企业登录/岗位趋势
- `lastYearReport_action` — `GET`：HR 年度报告海报。Params: `uid`

> **Rust 建议**：所有 ajax handler 返回统一 JSON `{code, msg, data}`，不用再区分 HTML/plain/JSON。

---

### wap/announcement.class.php — 公告

#### `index_action`
- **URL**: `GET /wap/index.php?c=announcement&a=index[&id={id}]`
- **Auth**: no
- **Returns**: Template `wap/announcement`（列表）/ `wap/announcements`（详情）
- **Model**: `announcement.model.php::getInfo()`, `upViewNum()`
- **Purpose**: 公告列表或详情

---

### wap/article.class.php — 职场资讯

#### `index_action` — 列表
- **URL**: `GET /wap/index.php?c=article&a=index`
- **Returns**: Template `wap/article`

#### `show_action` — 详情
- **URL**: `GET /wap/index.php?c=article&a=show&id={id}`
- **Model**: `article.model.php::getInfo/getGroup/getList`
- **Returns**: Template `wap/article_show`

#### `channels_action` — 频道管理
- **URL**: `GET /wap/index.php?c=article&a=channels`
- **Returns**: Template `wap/article_channels`

#### `editchannels_action`
- **URL**: `POST /wap/index.php?c=article&a=editchannels`
- **Params**: `newc`, `oldc`
- **Returns**: plain text redirect URL

#### `GetHits_action`
- **URL**: `GET /wap/index.php?c=article&a=GetHits&id={id}`
- **Returns**: JS `document.write(n)`（Rust 重写建议换成 JSON）

---

### wap/ask.class.php — 职场问答

| Action | URL | 功能 |
|---|---|---|
| `index` | GET `?c=ask&a=index` | 问答首页 |
| `list` | GET `?c=ask&a=list&cid&keyword` | 问答列表 |
| `content` | GET `?c=ask&a=content&id&orderby&page` | 问题详情+回答 |
| `answer` | POST `?c=ask&a=answer` | 回答问题（需 auth），参数 `id`, `content`, `authcode` |
| `topic` | GET `?c=ask&a=topic&pid` | 话题页 |
| `myquestion` | GET `?c=ask&a=myquestion&uid&page` | 我的提问 |
| `delask` | GET `?c=ask&a=delask&id&type` | 删提问/回答（type 0=问题 1=回答） |
| `attention` | GET/POST `?c=ask&a=attention` | 关注问题 |
| `myanswer` | GET `?c=ask&a=myanswer&uid&page` | 我的回答 |
| `attenquestion` | GET `?c=ask&a=attenquestion&uid&page` | 我关注的问题 |
| `hotweek` | GET `?c=ask&a=hotweek` | 一周热点 |
| `addquestion` | GET `?c=ask&a=addquestion` | 发布问题页 |
| `addquestions` | POST `?c=ask&a=addquestions` | 提交问题 `cid,title,content,authcode` |
| `qclass` | POST `?c=ask&a=qclass` | 分类 AJAX |
| `qrepost` | POST `?c=ask&a=qrepost` | 举报问题 `eid,reason[]` |
| `getcomment` | POST `?c=ask&a=getcomment` | 回答评论列表 `aid` |
| `forcomment` | POST `?c=ask&a=forcomment` | 添加评论 `aid,qid,content` |
| `forsupport` | POST `?c=ask&a=forsupport` | 点赞回答 `aid` |

> Model: `ask.model.php`, `msg.model.php`, `report.model.php`, `log.model.php`

---

### wap/claim.class.php — 会员认领

#### `index_action`
- **URL**: `GET /wap/index.php?c=claim&a=index&uid={uid}&code={code}`
- **Model**: `userinfo.model.php::getInfo()`, `company.model.php::getCertInfo()`
- **Returns**: Template `wap/claim`

#### `save_action`
- **URL**: `POST /wap/index.php?c=claim&a=save`
- **Params**: `uid`, `code`, `username`, `password`
- **Model**: `userinfo.model.php::upInfo()`
- **Returns**: JSON

---

### wap/company.class.php — 企业

#### `index_action` — 企业搜索列表
- **URL**: `GET /wap/index.php?c=company&a=index`
- **Params**（任意组合）: `provinceid`, `cityid`, `three_cityid`, `hy`, `pr`, `mun`, `rec`, `ecity`, `ehy`, `keyword`
- **Model**: `cache.model.php::GetCache()`, `pinYin()`
- **Returns**: Template `wap/company`

#### `show_action` — 企业详情
- **URL**: `GET /wap/index.php?c=company&a=show&id={company_uid}`
- **Model**: `company.model.php::getInfo/getCompanyShowList`, `statis.model.php`, `rating.model.php`
- **Returns**: Template（动态）

---

### wap/evaluate.class.php — 职业测评

| Action | URL | 功能 |
|---|---|---|
| `index` | GET `?c=evaluate&a=index&page` | 测评列表 → Rust: `GET /v1/wap/eval-papers` |
| `show` | GET `?c=evaluate&a=show&id` | 测评概览 → Rust: `GET /v1/wap/eval-papers/{id}` |
| `exampaper` | GET `?c=evaluate&a=exampaper&id` | 测评试卷 → 同上（题目随详情返回） |
| `grade` | POST `?c=evaluate&a=grade` | 提交答案 → Rust: `POST /v1/mcenter/eval-papers/{id}/submit` 🔒 |
| `gradeshow` | GET `?c=evaluate&a=gradeshow&id&page` | 成绩详情 → Rust: `GET /v1/mcenter/eval-logs/{id}` 🔒（Round 11）；`GET /v1/wap/eval-papers/{id}/recent-examinees`（Round 11） |
| `messages` | POST `?c=evaluate&a=messages` | 成绩页留言 `examid,message` → Rust: `POST /v1/mcenter/eval-papers/{id}/messages` 🔒，`GET /v1/wap/eval-papers/{id}/messages` |

---

### wap/forgetpw.class.php — 找回密码

| Action | URL | 功能 |
|---|---|---|
| `index` | GET `?c=forgetpw&a=index` | 找回密码页 |
| `sendcode` | POST `?c=forgetpw&a=sendcode` | 发送重置码 `authcode,sendtype=moblie/email,moblie,email` |
| `checksendcode` | POST `?c=forgetpw&a=checksendcode` | 验证重置码 `sendtype,moblie,email,code` |
| `checklink` | POST `?c=forgetpw&a=checklink` | 账号申诉 `username,linkman,linkphone,linkemail` |
| `editpw` | POST `?c=forgetpw&a=editpw` | 设置新密码 `username,uid,mobile,email,code,password` |

---

### wap/geetest.class.php — 极验验证

#### `index_action`
- **URL**: `GET /wap/index.php?c=geetest&a=index`
- **Returns**: JSON（极验 SDK 初始化格式）

---

### wap/gongzhao.class.php — 公招

| Action | URL |
|---|---|
| `index` | GET `?c=gongzhao&a=index` |
| `show` | GET `?c=gongzhao&a=show&id` |

---

### wap/index.class.php — 首页

| Action | URL | 功能 |
|---|---|---|
| `index` | GET `/wap/index.php` | WAP 首页 |
| `loginout` | GET `?c=index&a=loginout` | 退出登录 |
| `about` | GET `?c=index&a=about&fr` | 关于我们 |
| `contact` | GET `?c=index&a=contact&fr` | 联系我们 |
| `appDown` | GET `?c=index&a=appDown` | 下载 App |
| `privacy` | GET `?c=index&a=privacy&fr` | 隐私政策 |
| `protocol` | GET `?c=index&a=protocol&fr` | 用户协议 |
| `getmq` | POST `?c=index&a=getmq` | 首页名企 `limit` |
| `getCityDomain` | POST `?c=index&a=getCityDomain` | 坐标→城市 `x,y` |

---

### wap/job.class.php — 职位（19 actions）

| Action | URL | 功能 |
|---|---|---|
| `index` | GET `?c=job&a=index[&筛选]` | 职位搜索列表 |
| `search` | GET `?c=job&a=search` | 别名→index |
| `comapply` | GET `?c=job&a=comapply&id[&type=sq/fav&eid]` | 职位详情（支持申请/收藏） |
| `getreport` | GET `?c=job&a=getreport` | 获取举报原因列表 |
| `view` | GET `?c=job&a=view&id` | 兼容旧链接，重定向 comapply |
| `report` | POST `?c=job&a=report` | 提交举报（usertype=1）`id,reason,authcode` |
| `applyjobuid` | GET `?c=job&a=applyjobuid&jobid` | 快速申请职位页 |
| `share` | GET `?c=job&a=share&id` | 分享页 |
| `GetHits` | GET `?c=job&a=GetHits&id` | 浏览数 `document.write(n)` → Rust: `POST /v1/wap/jobs/{id}/hits`（Round 7） |
| `msg` | POST `?c=job&a=msg` | 职位咨询 |
| `jobmap` | GET `?c=job&a=jobmap[&uid=\|id]` | 地图显示 |
| `history` | POST `?c=job&a=history` | 浏览历史 `id`（usertype=1） |
| `ajaxLoad` | POST `?c=job&a=ajaxLoad` | 下拉加载 `searchurl,page,x,y` |
| `telQrcode` | GET `?c=job&a=telQrcode&id` | 联系方式二维码（PNG） |
| `getLink` | POST `?c=job&a=getLink` | 获取联系方式 `jobid` |
| `cancelJobFav` | POST `?c=job&a=cancelJobFav` | 取消收藏 `id` |
| `getJobWb` | POST `?c=job&a=getJobWb` | 职位文案 `jobid` |
| `getHbList` | POST `?c=job&a=getHbList` | 职位海报列表 |
| `getJobLink` | POST `?c=job&a=getJobLink` | 职位关联信息 `id` → Rust: `GET /v1/wap/jobs/{id}/contact`（Round 13） |

---

### wap/login.class.php — 登录（10 actions）

| Action | URL | 功能 |
|---|---|---|
| `index` | GET `?c=login&a=index[&bind&wxid&wxloginid]` | 登录页 |
| `mlogin` | POST `?c=login&a=mlogin` | 登录提交（见下方详细） |
| `sendmsg` | POST `?c=login&a=sendmsg` | 发送短信码 `moblie,authcode,is_yuliu` |
| `loginlock` | GET `?c=login&a=loginlock` | 账号锁定提示 |
| `utype` | GET `?c=login&a=utype` | 选身份 |
| `setutype` | GET `?c=login&a=setutype&usertype=1/2` | 激活身份 |
| `wxlogin` | GET `?c=login&a=wxlogin` | 微信扫码二维码 |
| `getwxloginstatus` | GET `?c=login&a=getwxloginstatus` | 轮询扫码状态 |
| `baloginsave` | POST `?c=login&a=baloginsave` | 第三方绑已有账号 `provider,username,password,authcode` |
| `balogin` | POST `?c=login&a=balogin` | 第三方快速注册 `provider` |

#### `mlogin_action` 详细参数（登录接口）
```
POST /wap/index.php?c=login&a=mlogin
Content-Type: application/x-www-form-urlencoded

username            用户名/手机/邮箱
password            密码（act_login=0）或 短信码（act_login=1）
dynamiccode         动态码（备用）
act_login           1=短信登录，0=密码
authcode            图形验证码
moblie              手机号（可选）
job                 职位 ID（可选，带投递场景）
checkurl            登录后 redirect URL
qfyuid              邀请人 UID
is_yuliu            1=预留信息模式
age/sex/realname/jid（is_yuliu=1 时）
```

Response（非 is_yuliu）:
```json
// layer_msg 格式，Rust 建议归一
{"msg": "...", "st": 8/9/2, "url": "跳转地址", "error": 0/1/2, "tm": 2}
```

Response（is_yuliu=1）：直接 JSON 的 userLogin() 返回值。

Cookie 下发：成功后 Set-Cookie `uid`, `shell`, `usertype`, `userdid`。

---

### wap/map.class.php — 地图职位

| Action | URL | 功能 |
|---|---|---|
| `index` | GET `?c=map&a=index[&x&y]` | 地图首页 |
| `maplist` | GET `?c=map&a=maplist` | 地图列表变体 |
| `joblist` | POST `?c=map&a=joblist` | 附近职位 `x,y,page,limit,depower` JSON |
| `comlist` | POST `?c=map&a=comlist` | 附近公司 `x,y,page` JSON |
| `jobmap` | GET `?c=map&a=jobmap&id` | 公司地址地图 |

## 3.2 N–Z

---

### wap/once.class.php — 店铺招聘（9 actions）

| Action | URL | 功能 |
|---|---|---|
| `index` | GET `?c=once&a=index` | 店铺招聘列表 |
| `sendmsg` | POST `?c=once&a=sendmsg` | 验证码 `moblie,authcode` |
| `add` | GET/POST `?c=once&a=add[&id]` | 添加/编辑店铺招聘（见下） |
| `show` | GET `?c=once&a=show&id[&pay=1]` | 详情 |
| `pay` | GET `?c=once&a=pay&id&oncepricegear` | 付款页 |
| `getOrder` | POST `?c=once&a=getOrder` | 创建订单 `id,paytype,once_price,oncepricegear` |
| `paylog` | GET `?c=once&a=paylog` | 待付款列表 |
| `delpaylog` | GET `?c=once&a=delpaylog&id` | 取消订单 |
| `ajax` | POST `?c=once&a=ajax` | 密码验证 `checkcode,id,password,operation_type` |

#### `add_action` POST 参数
```
title             招聘标题
companyname       公司名
linkman/phone     联系人/电话
provinceid/cityid/three_cityid
address           地址
require           招聘要求
yyzzpreview       营业执照（base64）
preview           店铺图片（base64）
edate             有效期（天）
salary            薪资范围
password          访问密码
oncepricegear     价格档位 ID
authcode/moblie_code/verify_token  校验
```

---

### wap/part.class.php — 兼职（5 actions）

| Action | URL |
|---|---|
| `index` | GET `?c=part&a=index` |
| `show` | GET `?c=part&a=show&id` |
| `collect` | POST `?c=part&a=collect` 收藏 `jobid,comid` |
| `apply` | POST `?c=part&a=apply` 报名 `jobid` |
| `telQrcode` | GET `?c=part&a=telQrcode&id` 二维码 |

---

### wap/qqconnect.class.php — QQ 登录（3 actions）

| Action | URL | 功能 |
|---|---|---|
| `qqlogin` | GET `?c=qqconnect&a=qqlogin[&code&state&login=1]` | OAuth 入口 |
| `qqbind` | GET/POST `?c=qqconnect&a=qqbind` | 绑定页，POST `moblie,moblie_code,authcode` |
| `sendmsg` | POST `?c=qqconnect&a=sendmsg` | 短信码 |

**OAuth Flow**（Rust 重写要点）：
1. 无 `code`：生成 `state`，redirect 到 `https://graph.qq.com/oauth2.0/authorize?...`
2. 有 `code`：
   - 调用 `get_token`，再 `oauth2.0/me` 拿 openid/unionid
   - 查数据库：已存在→签发 JWT，设置 cookie；否则 → qqbind 流程
3. state 建议存 Redis 5 分钟，防 CSRF

---

### wap/redeem.class.php — 积分商城（5 actions）

| Action | URL | 功能 |
|---|---|---|
| `index` | GET `?c=redeem&a=index` | 商城首页（需登录） |
| `list` | GET `?c=redeem&a=list` | 商品列表 |
| `show` | GET `?c=redeem&a=show&id&page` | 商品详情 |
| `dh` | GET `?c=redeem&a=dh&id&num` | 兑换确认 |
| `savedh` | POST `?c=redeem&a=savedh` | 提交兑换（见下） |

#### `savedh_action` 参数
```
id          商品 ID
num         数量
linkman     收货人
linktel     电话
provinceid/cityid/three_cityid
address     详细地址
body/other  备注
password    账户密码验证
```

---

### wap/register.class.php — 注册（9 actions）

| Action | URL | 功能 |
|---|---|---|
| `index` | GET/POST `?c=register&a=index[&type=1/2/3&bind=1/2/3&uid]` | 注册页 |
| `checkComName` | POST `?c=register&a=checkComName` | 查重公司名 `c_name` |
| `ident` | GET `?c=register&a=ident[&usertype=1/2]` | 身份激活 |
| `regok` | GET `?c=register&a=regok` | 成功页 |
| `ajaxreg` | POST `?c=register&a=ajaxreg` | 验用户名+邮箱+密码 |
| `regmoblie` | POST `?c=register&a=regmoblie` | 验手机 |
| `regemail` | POST `?c=register&a=regemail` | 验邮箱 |
| `writtenoff` | POST `?c=register&a=writtenoff` | 账号注销 |
| `jobregsave` | POST `?c=register&a=jobregsave` | 投递时快速注册 |

#### `index_action` (POST) 参数

```
usertype          1=个人,2=企业
username          用户名
email             邮箱
password          密码
moblie            手机
moblie_code       短信验证码
checkcode         图形验证码
regway            注册方式
qfyuid            邀请人
reg_name          企业名称（sy_reg_type=2 时）
reg_link          联系方式
reg_type          注册类型
```

---

### wap/reportlist.class.php — 举报（2 actions）

| Action | URL |
|---|---|
| `index` | GET `?c=reportlist&a=index` |
| `saveReport` | POST `?c=reportlist&a=saveReport` `uid,eid,r_name,reason[]` |

---

### wap/resume.class.php — 简历（8 actions）

| Action | URL | 功能 |
|---|---|---|
| `index` | GET `?c=resume&a=index` | 简历搜索列表 |
| `search` | GET `?c=resume&a=search` | 别名 |
| `show` | GET `?c=resume&a=show&uid\|id[&down=1]` | 详情（有隐私检查） |
| `share` | GET `?c=resume&a=share&id` | 分享页 |
| `invite` | GET `?c=resume&a=invite&uid[&jobid&invite=1]` | 邀请面试（usertype=2） |
| `history` | POST `?c=resume&a=history` | 浏览历史 `eid` (usertype=2) |
| `report` | POST `?c=resume&a=report` | 举报检查 `eid` 返回 1/2/3 |
| `ajaxLoad` | POST `?c=resume&a=ajaxLoad` | 下拉加载 `searchurl,page` |

---

### wap/search.class.php — 搜索历史（3 actions）

| Action | URL | 功能 |
|---|---|---|
| `history` | POST `?c=search&a=history` | 历史 `type=resume/job` |
| `del` | POST `?c=search&a=del` | 清空 `type=3/5` |
| `getComData` | POST `?c=search&a=getComData` | 搜索辅助数据 JSON |

---

### wap/services.class.php — 服务协议（1 action）

| Action | URL |
|---|---|
| `index` | GET `?c=services&a=index` |

---

### wap/sinaconnect.class.php — 微博登录（3 actions）

| Action | URL |
|---|---|
| `index` | GET `?c=sinaconnect[&code&login=1&wxoauth=1]` |
| `sinabind` | GET/POST `?c=sinaconnect&a=sinabind` |
| `sendmsg` | POST `?c=sinaconnect&a=sendmsg` |

同 qqconnect 的 OAuth 流程。

---

### wap/site.class.php — 分站（3 actions）

| Action | URL | 功能 |
|---|---|---|
| `index` | GET `?c=site&a=index` | 分站列表 |
| `cache` | POST `?c=site&a=cache` | 分站缓存 JSON |
| `domain` | POST `?c=site&a=domain` | 切换分站 `id` |

---

### wap/special.class.php — 专题招聘（6 actions）

| Action | URL | 功能 |
|---|---|---|
| `index` | GET `?c=special&a=index` | 专题列表 |
| `show` | GET `?c=special&a=show&id` | 专题详情 |
| `apply` | POST `?c=special&a=apply` | 企业报名 `id` (usertype=2) |
| `getComList` | POST `?c=special&a=getComList` | 企业 list `sid,hy,page,numb` |
| `getJobList` | POST `?c=special&a=getJobList` | 职位 list `sid` |
| `hotjobclass` | POST `?c=special&a=hotjobclass` | 热门职位分类 → Rust: `GET /v1/wap/categories/{kind}/recommended`（Round 12） |

---

### wap/tiny.class.php — 普工简历（5 actions）

| Action | URL | 功能 |
|---|---|---|
| `index` | GET `?c=tiny&a=index` | 普工列表 |
| `sendmsg` | POST `?c=tiny&a=sendmsg` | 验证码 |
| `add` | GET/POST `?c=tiny&a=add[&id]` | 添加/编辑（参数同 once 类似） |
| `show` | GET `?c=tiny&a=show&id` | 详情 |
| `ajax` | POST `?c=tiny&a=ajax` | 密码验证 |

---

### wap/upload.class.php — 文件上传（4 actions）

| Action | URL | 功能 |
|---|---|---|
| `qrcode` | GET `?c=upload&a=qrcode&type=1/2/3/4` | 生成扫码二维码（1=营业执照,2=身份证,3=头像,4=logo） |
| `p` | GET `?c=upload&a=p&t={token}` | 手机端扫码上传页 |
| `uploadimg_save` | POST `?c=upload&a=uploadimg_save` | 保存图片（base64） |
| `upCertPic` | POST `?c=upload&a=upCertPic` | 旧版证件上传 |

**安全要点**（Rust 重写强化）：

```
白名单后缀：.jpg .jpeg .png .gif .bmp
拒绝：.php .phtml .php5 .php7 .pht .phar .asp .aspx .jsp .exe .do .cgi .htaccess .user.ini
magic bytes 校验：image/jpeg (FFD8FF), image/png (89504E47), ...
大小限制：单图 < 5MB
生成的文件名：日期目录 + 时间戳 + 随机，不沿用用户传的 name
所有 OSS 上传走服务器中转，不要直连前端→OSS
```

---

### wap/wxconnect.class.php — 微信登录（3 actions）

| Action | URL |
|---|---|
| `index` | GET `?c=wxconnect[&code&state&login=1&wxoauth=1]` |
| `wxbind` | GET/POST `?c=wxconnect&a=wxbind` |
| `sendmsg` | POST `?c=wxconnect&a=sendmsg` |

Flow：scope=snsapi_userinfo → access_token → userinfo → openid/unionid。

---

### wap/wxoauth.class.php — 微信简化 OAuth（1 action）

| Action | URL |
|---|---|
| `index` | GET `?c=wxoauth[&code&state]` |

Flow：scope=snsapi_base（无感知），仅拿 openid；已绑定→登录，未绑定→跳 wxconnect。

---

### wap/zph.class.php — 招聘会（6 actions）

| Action | URL | 功能 |
|---|---|---|
| `index` | GET `?c=zph&a=index` | 列表 |
| `show` | GET `?c=zph&a=show&id` | 详情 |
| `com` | GET `?c=zph&a=com&id&page` | 参会企业 |
| `reserve` | GET `?c=zph&a=reserve&id[&zph=1]` | 预订页 |
| `getComList` | POST `?c=zph&a=getComList` `zid,page,limit` → Rust: `GET /v1/wap/zph/{id}/companies` |
| `getJobList` | POST `?c=zph&a=getJobList` `zid,page,limit` → Rust: `GET /v1/wap/zph/{id}/jobs`（Round 15） |

---

## 4. WAP 会员中心

### 入口

```
URL: /wap/member/index.php?m=wap_member&c={模块}&a={action}
Auth: 必须 — 未登录 302 到 /wap/index.php?c=login
```

会员中心的业务逻辑**和 PC 会员中心共享 model 代码**，只是模板换成 `app/template/wap/member/*`。Rust 重写建议：

- 一套业务 service，两套 handler（WAP / PC）或只保留一套（SPA 前端做适配）

### 4.1 个人求职者 (usertype=1)

入口 `c=index`（个人）的 action：

| Action | URL 段 | 功能 |
|---|---|---|
| `index` | `c=index&a=index` | 会员中心首页 |
| `isgzh` | POST `a=isgzh` | 检查关注公众号状态 |
| `otherservice` | `a=otherservice` | 其他服务 |
| `photo` | `a=photo` | 上传形象照 |
| `sq` | `a=sq` | 我申请的职位 |
| `partapply` | `a=partapply` | 兼职管理 |
| `collect` | `a=collect` | 收藏/关注 |
| `password` | `a=password` | 密码设置 |
| `invite` | `a=invite` | 面试通知列表 |
| `invitecont` | `a=invitecont` | 面试详情 |
| `look` | `a=look` | 谁看了我 |
| `addresume` | `a=addresume` | 创建简历 |
| `addresumeson` | `a=addresumeson` | 添加简历附表（工作/教育/项目等） |
| `info` | `a=info` | 基本信息 |
| `addexpect` | `a=addexpect` | 修改意向职位 |
| `rcomplete` | `a=rcomplete&id` | 简历发布成功 |
| `resume` | `a=resume` | 我的简历列表 |
| `optimize` | `a=optimize` | 优化简历 |
| `setPrivacyCookie` | `a=setPrivacyCookie` | 设置隐私 cookie |
| `binding` | `a=binding` | 社交账号绑定 |
| `idcard` | `a=idcard` | 身份证认证 |
| `bindingbox` | `a=bindingbox&type=moblie/email` | 手机/邮箱认证 |
| `setname` | `a=setname` | 修改用户名 |
| `reward_list` | `a=reward_list` | 兑换记录 |
| `privacy` | `a=privacy` | 隐私设置 |
| `getOrder` | POST `a=getOrder` | 生成订单 |
| `pay` | `a=pay` | 充值页 |
| `payment` | `a=payment&id` | 支付确认 |
| `dingdan` | POST `a=dingdan` | 创建订单 |
| `paylog` | `a=paylog` | 账单明细 |
| `likejob` | `a=likejob` | 职位速配 |
| `set` | `a=set` | 账户设置 |
| `sysnews` | `a=sysnews` | 消息通知 |
| `sxnews` | `a=sxnews` | 系统消息 |
| `commsg` | `a=commsg` | 求职咨询 |
| `finance` | `a=finance` | 财务管理 |
| `integral` | `a=integral` | 积分任务中心 |
| `blacklist` | `a=blacklist` | 屏蔽企业列表 |
| `blacklistadd` | `a=blacklistadd` | 添加屏蔽企业 |
| `transfer` | `a=transfer` | 账户分离 |
| `logout` | `a=logout` | 账号注销 |
| `ident` | `a=ident` | 认证与绑定 |
| `safe` | `a=safe` | 账号安全 |

### 4.2 企业招聘 (usertype=2)

入口 `c=com`（企业）的 action：

| Action | URL 段 | 功能 |
|---|---|---|
| `index` | `c=com&a=index` | 企业首页 |
| `isgzh` | POST `a=isgzh` | 关注公众号状态 |
| `zhaopin` | `a=zhaopin` | 招聘数据 |
| `zhaopinzhou` | `a=zhaopinzhou` | 周数据 |
| `com` | `a=com` | 我的服务 |
| `reportlist` | `a=reportlist` | 举报简历 |
| `info` | `a=info` | 基本信息 |
| `jobadd` | `a=jobadd` | 发布职位 |
| `saveJobSuccess` | `a=saveJobSuccess` | 发布成功 |
| `job` | `a=job` | 职位管理 |
| `partapply` | `a=partapply` | 兼职报名 |
| `hr` | `a=hr` | 应聘简历 |
| `password` | `a=password` | 改密码 |
| `pay` | `a=pay` | 充值 |
| `payment` | `a=payment&id` | 订单确认 |
| `getOrder` | POST `a=getOrder` | 生成订单 |
| `dingdan` | POST `a=dingdan` | 创建订单 |
| `look_job` | `a=look_job` | 谁看过我 |
| `invite` | `a=invite` | 面试邀请 |
| `part` | `a=part` | 兼职管理 |
| `partadd` | `a=partadd` | 发布兼职 |
| `photo` | `a=photo` | 企业 Logo |
| `comcert` | `a=comcert` | 企业资质 |
| `binding` | `a=binding` | 社交绑定 |
| `bindingbox` | `a=bindingbox` | 账户绑定 |
| `setname` | `a=setname` | 改用户名 |
| `reward_list` / `delreward` | 兑换记录 / 删除 |
| `paylog` | `a=paylog` | 账单 |
| `special` | `a=special` | 专题招聘 |
| `zhaopinhui` | `a=zhaopinhui` | 招聘会记录 |
| `set` | `a=set` | 账户设置 |
| `sysnews` / `sxnews` | 消息 / 系统消息 |
| `msg` | `a=msg` | 求职咨询 |
| `attention_me` | `a=attention_me` | 对我感兴趣 → Rust: `GET /v1/mcenter/fans`（Round 13） |
| `finance` | `a=finance` | 财务 |
| `integral` | `a=integral` | 任务中心 |
| `resumecolumn` | `a=resumecolumn` | 简历管理 |
| `jobcolumn` | `a=jobcolumn` | 其他服务 |
| `integral_reduce` | `a=integral_reduce` | 消费规则 |
| `banner` | `a=banner` | 企业 banner |
| `show` | `a=show` | 企业环境展示 |
| `server` | `a=server` | 优选服务 |
| `yqmb` / `yqmbedit` | 邀请模板管理 / 编辑 |
| `reserveUp` | POST `a=reserveUp` | 预约刷新 |
| `logout` | `a=logout` | 账号注销 |
| `address` / `newAddress` | 地址管理 / 新增 |
| `ident` / `safe` | 认证 / 安全 |
| `poi` | POST `a=poi` | POI 地点查询 |

---

## 5. Rust 实现建议

### 5.1 技术栈

```toml
[package]
name = "phpyun-rs"
version = "0.1.0"
edition = "2024"

[dependencies]
axum = "0.8"
tokio = { version = "1", features = ["full"] }
sqlx = { version = "0.8", features = ["mysql", "runtime-tokio-rustls", "chrono"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
jsonwebtoken = "9"
argon2 = "0.5"
tower = "0.5"
tower-http = { version = "0.6", features = ["trace", "cors", "limit", "compression-gzip"] }
tracing = "0.1"
tracing-subscriber = "0.3"
redis = { version = "0.26", features = ["tokio-comp"] }
chrono = { version = "0.4", features = ["serde"] }
once_cell = "1"
```

### 5.2 工程结构

```
phpyun-rs/
├─ src/
│  ├─ main.rs
│  ├─ config.rs                 # 配置加载
│  ├─ error.rs                  # 错误类型 + IntoResponse
│  ├─ auth/
│  │  ├─ mod.rs
│  │  ├─ jwt.rs                 # JWT 签发/校验
│  │  └─ middleware.rs          # require_auth / require_usertype
│  ├─ models/                   # 业务对象（ORM row 类型）
│  │  ├─ user.rs
│  │  ├─ company.rs
│  │  ├─ job.rs
│  │  ├─ resume.rs
│  │  └─ ...
│  ├─ services/                 # 业务逻辑
│  │  ├─ user_service.rs        # 对应 userinfo.model.php
│  │  ├─ job_service.rs         # 对应 job.model.php
│  │  └─ ...
│  ├─ handlers/                 # HTTP handler
│  │  ├─ wap/                   # WAP 前台
│  │  │  ├─ mod.rs
│  │  │  ├─ login.rs            # 对应 wap/login.class.php
│  │  │  ├─ register.rs
│  │  │  ├─ job.rs
│  │  │  ├─ resume.rs
│  │  │  ├─ company.rs
│  │  │  ├─ ajax.rs             # 所有 ajax action
│  │  │  └─ ...
│  │  └─ member/                # 会员中心
│  │     ├─ user/
│  │     └─ com/
│  ├─ oauth/                    # QQ/微信/微博 登录
│  └─ db/
│     └─ pool.rs
├─ migrations/                  # sqlx migrations（保留原表结构）
└─ Cargo.toml
```

### 5.3 路由组织样板

```rust
use axum::{Router, routing::{get, post}};

pub fn wap_routes(state: AppState) -> Router {
    Router::new()
        // 公开路由
        .route("/wap/index.php", get(handlers::wap::index::index))
        .nest("/api/wap", wap_api_routes())
        // 登录 / 注册
        .route("/wap/login", get(handlers::wap::login::index_page))
        .route("/wap/login/submit", post(handlers::wap::login::mlogin))
        .route("/wap/login/sms", post(handlers::wap::login::send_sms))
        // ...
        .with_state(state)
}

fn wap_api_routes() -> Router<AppState> {
    Router::new()
        // ajax
        .route("/ajax/sign", post(handlers::wap::ajax::sign))
        .route("/ajax/atn", post(handlers::wap::ajax::atn))
        .route("/ajax/msg-num", get(handlers::wap::ajax::msg_num))
        // job
        .route("/job/list", get(handlers::wap::job::list))
        .route("/job/:id", get(handlers::wap::job::show))
        .route("/job/:id/apply", post(handlers::wap::job::apply))
        // ...
}
```

### 5.4 登录接口 Rust 样板

```rust
use axum::{extract::{State, Json}, response::IntoResponse, Form};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
    pub authcode: Option<String>,
    pub act_login: Option<u8>,
    pub moblie: Option<String>,
    pub checkurl: Option<String>,
    pub is_yuliu: Option<u8>,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub code: i32,         // 0=success, 非零=error
    pub msg: String,
    pub data: Option<LoginData>,
}

#[derive(Serialize)]
pub struct LoginData {
    pub token: String,     // JWT，替代 cookie shell
    pub uid: u64,
    pub usertype: u8,
    pub redirect: Option<String>,
}

pub async fn mlogin(
    State(state): State<AppState>,
    Form(form): Form<LoginForm>,
) -> Result<Json<LoginResponse>, AppError> {
    // 1. 验证码校验
    if form.act_login != Some(1) {
        services::verify::check_captcha(&state, &form.authcode).await?;
    }
    
    // 2. 查用户
    let user = sqlx::query_as!(
        User,
        "SELECT uid, username, password, salt, usertype, status 
         FROM phpyun_member 
         WHERE username = ? OR moblie = ? OR email = ? 
         LIMIT 1",
        form.username, form.username, form.username
    ).fetch_optional(&state.db).await?
    .ok_or(AppError::InvalidCredentials)?;
    
    if user.status == 2 { return Err(AppError::AccountLocked); }
    
    // 3. 密码校验（兼容老 md5，登录时升级为 argon2）
    let password = form.password;
    let valid = if user.password.starts_with("$argon2") {
        argon2::verify_password(&user.password, &password)
    } else {
        // 旧算法：md5(md5($pw) . $salt)
        let legacy = format!("{:x}", md5::compute(
            format!("{:x}{}", md5::compute(&password), user.salt)
        ));
        if legacy == user.password {
            let new_hash = argon2_hash(&password)?;
            sqlx::query!("UPDATE phpyun_member SET password=? WHERE uid=?",
                new_hash, user.uid).execute(&state.db).await?;
            true
        } else {
            false
        }
    };
    if !valid { return Err(AppError::InvalidCredentials); }
    
    // 4. 签发 JWT
    let token = issue_jwt(&state.jwt_secret, user.uid, user.usertype)?;
    
    // 5. 写登录日志
    services::log_service::add_login_log(&state, user.uid, get_client_ip(&req)).await;
    
    Ok(Json(LoginResponse {
        code: 0,
        msg: "登录成功".into(),
        data: Some(LoginData {
            token,
            uid: user.uid,
            usertype: user.usertype,
            redirect: form.checkurl,
        }),
    }))
}
```

### 5.5 兼容策略（渐进式迁移）

Rust 服务上线后，**一开始不用把所有接口都写完**。用 Nginx 层面做路由分流：

```nginx
# 已 Rust 化的路由
location ~ ^/wap/(login|register|job/list|resume/list)(/|$) {
    proxy_pass http://127.0.0.1:3000;
}

# 其他还跑 PHP-FPM
location ~ \.php$ {
    fastcgi_pass unix:/run/php/php-fpm.sock;
    ...
}
```

按以下优先级迁移（从高到低）：

1. **登录 / 注册**（[wap/login](#waploginclassphp--登录10-actions) / [register](#wapregisterclassphp--注册9-actions)）— 用 argon2 替代 md5，消除弱密码风险
2. **文件上传**（[wap/upload](#wapuploadclassphp--文件上传4-actions)）— 严格 magic-bytes 校验，消除上传木马风险
3. **搜索接口**（[wap/job](#wapjobclassphp--职位19-actions) / resume list / ajax）— 读多写少，性能受益大
4. **第三方登录**（[qqconnect](#wapqqconnectclassphp--qq-登录3-actions) / [wxconnect](#wapwxconnectclassphp--微信登录3-actions) / [sinaconnect](#wapsinaconnectclassphp--微博登录3-actions) / [wxoauth](#wapwxoauthclassphp--微信简化-oauth1-action)）— OAuth 签名和 state 存储统一到 Redis
5. **支付回调**（在 `api/alipay/`、`api/tenpay/`，本文档未覆盖）— 签名校验是最高危点
6. **会员中心**（[WAP 会员中心](#4-wap-会员中心)）— 最后迁，业务最复杂

### 5.6 数据库共享策略

Rust 服务和 PHP 服务**共享同一个 MySQL 数据库**，不做数据迁移：

- PHP 继续用 `/www/wwwroot/zzzz.com/uploads/config/db.config.php` 的凭据
- Rust 用相同凭据连接，读写相同表
- 表名保留 `phpyun_` 前缀
- 不新建迁移，遵循现有 schema

这样 Rust 可以逐步接管，PHP 老代码还能继续跑。MySQL 的事务隔离保证一致性。

### 5.7 明确要改掉的 PHPYun 设计

| PHPYun 原设计 | Rust 改进 |
|---|---|
| `error_reporting(0)` 吞错 | tracing 结构化日志 + Sentry 集成 |
| `md5(md5($pw)+salt)` | argon2id |
| Cookie 无 `HttpOnly/Secure` | `Set-Cookie` 统一强制 |
| Token = md5 派生，永不过期 | JWT + exp + refresh token |
| `shell` Cookie 在 HTML 里暴露 | JWT 仅在 HttpOnly cookie |
| 无前台 CSRF | SameSite=Strict + 双提交 cookie pattern |
| SQL 拼接（即使有 addslashes） | 100% sqlx 编译期校验参数绑定 |
| `eval('<?php $config_vars = ...' )`（Smarty 编译）| 不用，直接 SSR 或 JSON |
| 硬编码 `init.ov6.com` 信标 | 彻底移除 |
| PHP 黑名单 WAF 正则 | 不做 WAF，用强类型 + prepared statements 从根源避免 |

---

## 附录 A：术语 Glossary

| PHP / 中文 | 英文 / 规范 |
|---|---|
| 简历 eid | resume_id |
| 职位 jid | job_id |
| 会员 uid | member_id / user_id |
| 企业 uid（也叫 comid） | company_id |
| 分站 did | site_id |
| usertype=1 | role=jobseeker |
| usertype=2 | role=employer |
| 招聘会 zph | career_fair |
| 兼职 part | part_time_job |
| 店铺招聘 once | shop_recruit |
| 普工 tiny | general_worker |
| 悬赏 reward | bounty |
| 猎头 lietou | headhunter |

## 附录 B：响应码建议

```
0        success
1xxx     参数错误类
  1001   缺少必要参数
  1002   参数格式错误
  1003   验证码错误
2xxx     认证/授权类
  2001   未登录
  2002   会话过期 / token 失效
  2003   无权访问此资源
  2004   用户身份不匹配
3xxx     业务错误类
  3001   用户不存在
  3002   用户名/密码错误
  3003   账号被锁
  3004   手机号已注册
  3005   积分不足
  3006   套餐已用完
  3007   职位已下架
...
4xxx     资源限制类
  4001   日操作次数超限
  4002   IP 频繁请求
5xxx     第三方服务失败类
9xxx     系统错误类
```

---

**文档版本**: 1.7（2026-04-27 全栈入参校验加固 + admin 三层鉴权 + Round 11–15 增量接口；详见附录 C）  
**生成时间**: 2026-04-23  
**覆盖范围**: 32 个 WAP 前台控制器 / 216 个 action / WAP 会员中心 100+ actions  
**Rust 已实现**: 371 个 utoipa 注册接口（截至 Round 15）；详见附录 C 的增量列表与 OpenAPI `/openapi.json`。  
**安全基线**:
- §2「入参校验」全栈强制（Path/Query/Json/Form 必经 `validator::Validate`）；CI 测试保证搜索/列表接口走 `ValidatedQuery`。
- §2「鉴权 & 会话」三层鉴权矩阵已对所有 `/v1/admin/*` 强制生效；CI 测试保证不会被新增路由破坏。

---

## 附录 C：Rust 实施增量（Round 11–15）

> 该附录记录 PHP→Rust 重写过程中各 Round 完成的接口，便于和 §3 / §4 的 PHP 原始 action 对照。每条接口均同时登记到 `crates/handlers/tests/new_endpoints.rs` 的静态路由测试中，可通过 `cargo test -p phpyun-handlers --test new_endpoints` 反向校验。所有接口默认走统一响应封装 `{code, msg, data}`，Auth 类标注 `🔒` 表示需 Bearer Token。

### Round 11 — 推荐配额、测评日志、最近答题用户

| 方法 | 路径 | 来源 PHP action | 备注 |
|---|---|---|---|
| `GET` 🔒 | `/v1/mcenter/recommend/email/quota` | `wap/ajax::ajax_recommend_interval_action` | 返回 `{status, msg, used_today, day_cap, interval_secs, seconds_remaining}`；feature-flag 由 `sy_recommend_day_num` 控制（0 = 关闭） |
| `GET` 🔒 | `/v1/mcenter/eval-logs/{id}` | `app/evaluate/exampaper::gradeshow_action` | 单条作答记录详情；只能查本人或他人公开记录 |
| `GET` | `/v1/wap/eval-papers/{id}/recent-examinees` | `evaluate.model.php::getEvaluateLogList(groupby:uid)` | 按 uid 分组的最近答题人，最多 12 条 |

### Round 12 — 推荐分类

| 方法 | 路径 | 来源 PHP action | 备注 |
|---|---|---|---|
| `GET` | `/v1/wap/categories/{kind}/recommended?limit=` | `category::getHotJobClass(rec=1)` | `kind ∈ {job, company/industry/com, city, part, question/qa}`；只有 `phpyun_job_class` 有 `rec` 列，其他表退回到 `ORDER BY sort DESC` 的 top-N；未识别 kind 返回空数组 |

### Round 13 — 公司端粉丝 + 职位联系方式

| 方法 | 路径 | 来源 PHP action | 备注 |
|---|---|---|---|
| `GET` 🔒 | `/v1/mcenter/fans` | `wap/com::attention_me_action` | 公司端"对我感兴趣"列表；返回 `{uid, username, fav_count, last_datetime}`，按最近收藏倒序；`usertype != 2` 时返回空 |
| `GET` | `/v1/wap/jobs/{id}/contact` | `app/job/comapply::getJobLink_action` (+ `getContactNew`) | 按 `phpyun_company_job.is_link∈{1,2,3}` 解析 `phpyun_company` 默认 / `phpyun_company_job_link` 备选联系方式；附 `city_name` 字典翻译；404 当职位不存在 |

### Round 14 — 简历访问统计 + 默认期望跳转

| 方法 | 路径 | 来源 PHP action | 备注 |
|---|---|---|---|
| `POST` | `/v1/wap/resumes/expects/{eid}/hits` | `app/resume/show::GetHits_action` | 自增 `phpyun_resume_expect.hits +1`，返回 `{eid, hits}`；PHP 原版可按 `sy_job_hits` 配置随机膨胀，Rust 版固定 +1（可后续接入配置） |
| `GET` | `/v1/wap/resumes/by-uid/{uid}/default-expect` | `wap/resume::showuid_action` | 取 `phpyun_resume.def_job`（公开求职意向 id）；用于替换 PHP 旧 `?uid=` 跳转链接；隐藏/草稿简历 → `default_eid: 0` |

### Round 15 — 招聘会职位列表

| 方法 | 路径 | 来源 PHP action | 备注 |
|---|---|---|---|
| `GET` | `/v1/wap/zph/{id}/jobs` | `app/zph/index::getJobList_action` | 把所有签到企业的 `phpyun_zhaopinhui_com.jobid` CSV 展开后批量加载 `phpyun_company_job`；过滤 `state=1, r_status=1, status=0`；保持 CSV 顺序，最多 40 条；返回与 `/v1/wap/jobs` 同结构的 `JobSummary` 卡片，前端可复用同一组件 |

### Round 17 — 全栈入参校验加固（无新增路由）

> 不增加路由，但把"参数校验"从"个别接口手写"升级为"`Validate` 派生 + 提取器自动跑 + CI 静态校验"的统一流水线。重点拦截：1) 自由文本字段长度炸弹（10MB 关键字 → LIKE OOM）；2) 数值字段越界（`is_rec=99` 直进 DB）；3) 枚举/槽位串非法字符。

**变更：**
- 新增 [`phpyun_core::ValidatedQuery<T>`](phpyun-rs/crates/core/src/extractors.rs)：query string 版本的 `ValidatedJson`，extractor 在 deserialize 后自动跑 `validator::Validate`，失败返回 `param_invalid: validation.<key>`。
- 新增 [`LocalizedDicts::find_id_by_name`](phpyun-rs/crates/services/src/dict_service.rs)：写入路径接收 `*_classname` 文本时反查字典 id（解锁 `mcenter/resume_expect` 此前已存在但缺方法的代码路径）。
- 升级 13 个高风险列表/搜索 handler 到 `ValidatedQuery`，并补齐字段约束（`keyword ≤ 100`、id ∈ 0..=9_999_999、limit 上限等）：
  - `wap/jobs.rs`（`JobListQuery`/`RecQuery`）
  - `wap/companies.rs`（`CompanyListQuery`/`CompanyAutoQuery`/`HotCompaniesQuery`）
  - `wap/resumes.rs`（`ResumeListQuery`）
  - `wap/articles.rs`（`ArticleListQuery`）
  - `wap/qna.rs`（`QListQuery`/`HotweekQuery`/`TopAnswerersQuery`）
  - `wap/search.rs`（`SearchQuery`）
  - `wap/ads.rs`（`AdQuery`，含字符集自定义校验）
  - `mcenter/messages.rs`（`MessageListQuery`）
  - `mcenter/search_history.rs`（`ListQuery`/`ClearQuery`）
  - `mcenter/views.rs`（`KindQuery`，`kind ∈ 1..=3`）
- `admin/redeem.rs::set_reward_flags`：`FlagsForm` 改为 `Validate` 派生 + `is_rec/is_hot ∈ 0..=1`；service 入口同样加 0/1 防御 + `require_admin`。
- 新增 CI 静态测试 [`high_risk_search_handlers_use_validated_query`](phpyun-rs/crates/handlers/tests/new_endpoints.rs)：扫描源文件，搜索/列表 Query 结构体若被裸 `Query<...>` 提取即 CI 失败。
- §2「入参校验」章节补全字段级约束清单、检查点、已落地接口、CI 强制项、验证证据。

**已验证：**
- `cargo test -p phpyun-handlers --test new_endpoints` → `47 passed`
- 200-char keyword → `400 validation.length`
- 含空格的 slot → `400 validation.slot_charset`
- 空 `kw` → `400 validation.length`
- `limit=99999` → `400 validation.range`
- 合法请求 200 OK，业务行为不受影响

### Round 16 — admin 三层鉴权加固（无新增路由）

> 不增加路由，但把 `/v1/admin/*` 的鉴权从单层 `require_admin()` 升级为 L1 路由中间件 + L2 服务层 + L3 handler + CI 静态校验四道防线。

**变更：**
- 新增 [`crates/core/src/admin_guard.rs`](phpyun-rs/crates/core/src/admin_guard.rs)：路由级中间件，挂在 `nest("/admin", ...)` 上。任意 `/v1/admin/*` 请求先解析 + 验签 + 黑名单 + `pw_epoch`，再判 `usertype == 3`，否则 401/403 直接返回，不进 handler / DB / Redis 业务路径。
- 新增 [`crates/handlers/src/routes.rs::build_router_with_state`](phpyun-rs/crates/handlers/src/routes.rs)：production main 入口换成 state-aware 版本，把 admin guard layer 通过 `from_fn_with_state` 装到 admin nest 上。
- 补齐 12 个 admin handler 的 L3 守卫（`account_logout` / `ads` / `app_versions` / `audit_log` / `broadcasts` / `categories` / `dashboard` / `merge` / `nav` / `site_settings` / `usertype_change` / `warnings::issue` / `company_cert::review`）。
- 新增 CI 静态测试 `every_admin_route_declares_bearer_security`（`crates/handlers/tests/new_endpoints.rs`）：遍历 OpenAPI 文档，任一 `/v1/admin/*` 路由的 `#[utoipa::path]` 漏写 `security(("bearer" = []))` 即 CI 失败。
- §2「三层鉴权」章节补全详细说明、失败响应矩阵、token 强制条件、高权重接口护栏、CI 强制项。

**已验证：**
- `curl /v1/admin/dashboard/overview` 无 token → `401 unauthenticated`
- `curl -H 'Authorization: Bearer bogus' /v1/admin/dashboard/overview` → `401 session_expired`
- `cargo test -p phpyun-handlers --test new_endpoints every_admin_route_declares_bearer_security` → `ok. 1 passed`
- 所有 admin 接口 `routes ≤ guards`（部分 `route(...).get(...).post(...)` 是 1 路由 N 处理函数，guards > routes 是正常）

### 字段命名约定（§0 响应约定的延伸）

Rust 重写过程中沉淀的几条命名约定，方便阅读已交付接口时对号入座：

- `*_n` 后缀：服务端把原始字段渲染好的"展示版"。
  - `cover` → `cover_n`：CDN 全路径
  - `created_at` → `created_at_n`：`YYYY-MM-DD HH:MM` 字符串
  - `hy` (industry id) → `hy_n`：字典翻译后的中文名
- `is_*` 字段：`bool`（不再使用 PHP 习惯的 `0/1` 字符串）
- 列表分页统一返回 `Paged<T> = {list, total, page, page_size}`
- 计数/状态切换接口返回 `{status, msg, data}` 而不是 PHP 的 `echo 0/1/2/3`

### 已知偏离 PHP 的点

| 接口 | 偏离 | 理由 |
|---|---|---|
| `/v1/wap/resumes/expects/{eid}/hits` | 固定 `+1`，未读取 `sy_job_hits` 随机膨胀 | 反作弊视角：随机膨胀只是给 PHP 数据"好看"用的，不应内置；如需保留可在 Rust 服务层加配置 |
| `/v1/wap/categories/{kind}/recommended` | 仅 `job` kind 走 `rec=1` 过滤 | 其他四个真实表（comclass / city_class / partclass / q_class）没有 `rec` 列；fallback 到 sort 排序而非报错 |
| `/v1/mcenter/fans` | `usertype != 2` 返回空数组而非 403 | 前端已按身份隔离入口；改为静默空列表降低误用噪声 |
| `/v1/wap/jobs/{id}/contact` | 未鉴权（PHP 行为相同） | 站点要按 VIP/积分扣费的话，应在路由前再加一层 service 守卫；当前与 PHP 默认配置对齐 |

### 测试覆盖

| Round | 静态路由测试函数（`crates/handlers/tests/new_endpoints.rs`） |
|---|---|
| 11 | `recommend_quota_route_registered`, `eval_log_detail_route_registered`, `eval_recent_examinees_route_registered` |
| 12 | `recommended_categories_route_registered` |
| 13 | `fans_route_registered`, `job_contact_route_registered` |
| 14 | `resume_expect_hits_route_registered`, `resume_default_expect_route_registered` |
| 15 | `zph_jobs_route_registered` |
| 16 | `every_admin_route_declares_bearer_security` —— 全 admin 接口 OpenAPI 安全声明强制项 |
| 17 | `high_risk_search_handlers_use_validated_query` —— 搜索/列表 Query 结构体强制走 `ValidatedQuery` |

> 这批静态测试只走 `V1Doc::openapi()` 内省、不连数据库，便于 CI 快速反向校验"路由 + Schema 是否在 OpenAPI 中正确登记"。完整 E2E 烟雾测试见 `scripts/`（人工触发）。

---

**最近一次更新**: 2026-04-27（Round 17 全栈入参校验加固 —— `ValidatedQuery<T>` extractor + 13 个搜索/列表 handler 全部上 `Validate` + 字段级 length/range/charset 约束 + CI 静态校验 `high_risk_search_handlers_use_validated_query`；详见附录 C）  
**下一步**: Round 18+ 对齐 `safe` 安全面板汇总 / `paylog` 账单流水 / mcenter 路由层细分（jobseeker-only / employer-only 中间件，对照 admin guard 模式扩展）
