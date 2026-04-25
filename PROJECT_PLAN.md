# PHPYun → Rust 重写项目计划书

> **目标**：用 Rust + axum 实现 [WAP_API_SPEC.md](WAP_API_SPEC.md) 列出的全部接口，替换现有 PHP 后端。  
> **覆盖范围**：32 个 WAP 前台控制器（216 actions）+ 个人会员中心（45）+ 企业会员中心（53）≈ **314 个 HTTP endpoints**。  
> **策略**：**渐进式替换**，不做 Big-Bang 切换；Rust 与 PHP 共用 MySQL，逐 endpoint 迁移。  
> **文档版本**：v1 · 2026-04-23

---

## 目录

- [1. 总览](#1-总览)
- [2. 技术栈](#2-技术栈)
- [3. 项目结构（cargo workspace）](#3-项目结构cargo-workspace)
- [4. 里程碑（Milestones）](#4-里程碑milestones)
- [5. 数据库与数据兼容](#5-数据库与数据兼容)
- [6. 鉴权迁移策略](#6-鉴权迁移策略)
- [7. Nginx 分流与灰度](#7-nginx-分流与灰度)
- [8. 测试策略](#8-测试策略)
- [9. 部署策略](#9-部署策略)
- [10. 监控与运维](#10-监控与运维)
- [11. 风险清单与缓解](#11-风险清单与缓解)
- [12. 工作量估算与排期](#12-工作量估算与排期)
- [13. 交付物清单](#13-交付物清单)
- [14. 团队与协作](#14-团队与协作)

---

## 1. 总览

### 1.1 为什么要重写

| 问题 | 重写后解决 |
|---|---|
| SQL 字符串拼接（见前期审计）| sqlx 编译期 prepared statement |
| md5 密码 + 6 位 salt | argon2id + 自动升级 |
| 前台零 CSRF | SameSite=Strict + 双提交 cookie |
| Cookie 无 HttpOnly/Secure | axum 全局约定 |
| 信标 `init.ov6.com` 泄漏 coding 密钥 | 新系统彻底剥离 |
| 一请求数十个 PHP Notice 刷屏 | 类型安全，编译期消除 |
| PHP 单进程阻塞 | tokio 异步，单节点 ~50k req/s |
| 难以做接口级限流 | tower middleware 可插拔 |

### 1.2 不重写的

- **MySQL schema** — 表结构 100% 保留，Rust 用 sqlx 读写相同表
- **PHP 后台管理面板** — `/admin/*` 路径保留（工作量最大且业务复杂，用户少，不值得先重写）
- **支付回调** — `/api/alipay/*`, `/api/tenpay/*`, `/api/wapalipay/*` 保留 PHP（签名校验兼容性复杂，银行/支付宝主动 POST URL 不能改）
- **UCenter / PWind 集成** — `api/uc_*`, `api/pw_api` 保留，用户基本不用
- **文件/图片** — 继续放 `data/upload/` 目录，Rust 只负责写入前校验

### 1.3 目标性能指标

| 指标 | 原 PHP | Rust 目标 |
|---|---|---|
| QPS（单核）| ~100 req/s | ~10,000 req/s |
| P99 延迟 | 200~500ms | < 50ms |
| 内存占用（常驻）| 每 php-fpm worker ~50MB × N | 总 ~100MB |
| 冷启动 | php-fpm 3 秒 | < 500ms |
| SQL 漏洞面 | 手动防护 | 100% prepared |

---

## 2. 技术栈

### 2.1 核心依赖

```toml
[workspace.dependencies]
# Web 框架
axum = "0.8"
tower = { version = "0.5", features = ["limit", "timeout", "util"] }
tower-http = { version = "0.6", features = [
    "trace", "cors", "limit", "compression-gzip", "fs", "set-header", "request-id"
] }

# 异步运行时
tokio = { version = "1", features = ["full"] }
futures = "0.3"

# 数据库
sqlx = { version = "0.8", features = [
    "mysql", "runtime-tokio-rustls", "chrono", "json", "migrate"
] }

# Redis (会话、限流、缓存)
redis = { version = "0.26", features = ["tokio-comp", "connection-manager"] }
deadpool-redis = "0.18"

# 鉴权
jsonwebtoken = "9"
argon2 = "0.5"
md-5 = "0.10"  # 仅用于兼容旧密码

# 序列化
serde = { version = "1", features = ["derive"] }
serde_json = "1"
serde_urlencoded = "0.7"

# HTTP 客户端（OAuth、短信 API）
reqwest = { version = "0.12", features = ["json", "rustls-tls"] }

# 日志/Tracing
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
tracing-appender = "0.2"
sentry = { version = "0.34", features = ["tower", "tracing"] }

# 错误处理
thiserror = "1"
anyhow = "1"

# 工具
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1", features = ["v4", "v7"] }
once_cell = "1"
dotenvy = "0.15"
regex = "1"
base64 = "0.22"
rand = "0.8"

# 文件 / 图片
image = "0.25"
infer = "0.16"  # MIME magic bytes

# 配置
config = "0.14"

# 限流
tower_governor = "0.4"

# 测试
[dev-dependencies]
axum-test = "16"
mockito = "1"
insta = "1"  # snapshot testing
```

### 2.2 技术选型理由

| 选型 | 理由 |
|---|---|
| **axum** | Rust 生态最主流的 Web 框架，tokio 原生，中间件模型简洁 |
| **sqlx** | 编译期校验 SQL（`query!` 宏），无 ORM 魔法，写 SQL 像写 SQL |
| **tokio** | 异步标杆，连接池、runtime 成熟 |
| **argon2** | 现代密码哈希，PHC 标准，抵抗 GPU 破解 |
| **jsonwebtoken** | 无状态 JWT，签发快速，支持 RS256/HS256 |
| **Redis** | Session / 限流 / 短信码 / state / 分布式锁 |
| **reqwest** | rustls，零 OpenSSL 依赖，async first |
| **tracing** | 结构化日志，和 Sentry/Jaeger 打通 |

### 2.3 不使用的

- ❌ **actix-web**（学习曲线比 axum 陡）
- ❌ **Rocket**（同步为主，性能和生态弱于 axum）
- ❌ **Diesel**（同步，和 tokio 不契合）
- ❌ **SeaORM**（ORM 魔法太重，此项目场景用 sqlx 更合适）
- ❌ **OpenSSL**（选 rustls，避开本地 libssl 依赖）

---

## 3. 项目结构（cargo workspace）

```
phpyun-rs/
├─ Cargo.toml                        # Workspace 根
├─ Cargo.lock
├─ .env.example                      # 环境变量模板
├─ .gitignore
├─ README.md
├─ deploy/
│  ├─ Dockerfile
│  ├─ docker-compose.yml
│  ├─ systemd/
│  │  └─ phpyun-rs.service
│  └─ nginx/
│     └─ route-map.conf              # 逐 endpoint 分流规则
├─ migrations/                       # sqlx migrations（不建表，只建索引/触发器兼容）
│  └─ 20260501_add_jwt_exp_index.sql
├─ sqlx-data.json                    # 离线模式编译所需
├─ docs/
│  ├─ api-spec.md -> ../WAP_API_SPEC.md
│  └─ adr/                           # 架构决策记录
│
├─ crates/
│  │
│  ├─ core/                          # 基础设施（不依赖业务）
│  │  ├─ Cargo.toml
│  │  └─ src/
│  │     ├─ lib.rs
│  │     ├─ config.rs                # Config 加载
│  │     ├─ error.rs                 # AppError / IntoResponse
│  │     ├─ db.rs                    # DB pool + migration runner
│  │     ├─ redis.rs                 # Redis pool
│  │     ├─ telemetry.rs             # tracing init + Sentry
│  │     └─ middleware/
│  │        ├─ request_id.rs
│  │        ├─ logging.rs
│  │        └─ rate_limit.rs
│  │
│  ├─ auth/                          # 鉴权
│  │  ├─ Cargo.toml
│  │  └─ src/
│  │     ├─ lib.rs
│  │     ├─ jwt.rs                   # issue/verify JWT
│  │     ├─ password.rs              # argon2 + md5 兼容
│  │     ├─ middleware.rs            # require_auth / require_usertype
│  │     └─ claims.rs
│  │
│  ├─ models/                        # 数据库实体 + 仓储
│  │  ├─ Cargo.toml
│  │  └─ src/
│  │     ├─ lib.rs
│  │     ├─ user/                    # phpyun_member + 关联表
│  │     │  ├─ mod.rs
│  │     │  ├─ entity.rs             # struct Member
│  │     │  └─ repo.rs               # CRUD
│  │     ├─ company/
│  │     ├─ job/
│  │     ├─ resume/
│  │     ├─ ask/
│  │     ├─ part/
│  │     ├─ once/
│  │     ├─ tiny/
│  │     ├─ zph/
│  │     ├─ special/
│  │     ├─ article/
│  │     ├─ announcement/
│  │     ├─ evaluate/
│  │     ├─ redeem/
│  │     ├─ msg/
│  │     ├─ notice/                  # 短信/邮件模板
│  │     ├─ order/
│  │     ├─ admin_config/
│  │     └─ common/                  # cache, page, site, domain
│  │
│  ├─ services/                      # 业务逻辑（跨 model）
│  │  ├─ Cargo.toml
│  │  └─ src/
│  │     ├─ lib.rs
│  │     ├─ user_service.rs          # 对应 userinfo.model.php
│  │     ├─ job_service.rs
│  │     ├─ resume_service.rs
│  │     ├─ company_service.rs
│  │     ├─ ask_service.rs
│  │     ├─ part_service.rs
│  │     ├─ once_service.rs
│  │     ├─ notice_service.rs        # 短信/邮件发送
│  │     ├─ integral_service.rs      # 积分结算
│  │     ├─ oauth_service.rs         # QQ/微信/微博
│  │     ├─ captcha_service.rs       # 图形+短信验证码
│  │     ├─ upload_service.rs        # 文件上传
│  │     ├─ log_service.rs
│  │     └─ warning_service.rs       # 风控预警
│  │
│  ├─ handlers/                      # HTTP handler
│  │  ├─ Cargo.toml
│  │  └─ src/
│  │     ├─ lib.rs
│  │     ├─ routes.rs                # 总路由拼接
│  │     ├─ wap/                     # WAP 前台（32 个 controllers）
│  │     │  ├─ mod.rs
│  │     │  ├─ index.rs
│  │     │  ├─ login.rs
│  │     │  ├─ register.rs
│  │     │  ├─ forgetpw.rs
│  │     │  ├─ job.rs
│  │     │  ├─ resume.rs
│  │     │  ├─ company.rs
│  │     │  ├─ ask.rs
│  │     │  ├─ article.rs
│  │     │  ├─ announcement.rs
│  │     │  ├─ part.rs
│  │     │  ├─ tiny.rs
│  │     │  ├─ once.rs
│  │     │  ├─ zph.rs
│  │     │  ├─ special.rs
│  │     │  ├─ gongzhao.rs
│  │     │  ├─ evaluate.rs
│  │     │  ├─ redeem.rs
│  │     │  ├─ map.rs
│  │     │  ├─ search.rs
│  │     │  ├─ site.rs
│  │     │  ├─ claim.rs
│  │     │  ├─ advice.rs
│  │     │  ├─ reportlist.rs
│  │     │  ├─ services.rs
│  │     │  ├─ geetest.rs
│  │     │  ├─ upload.rs
│  │     │  ├─ qqconnect.rs
│  │     │  ├─ sinaconnect.rs
│  │     │  ├─ wxconnect.rs
│  │     │  ├─ wxoauth.rs
│  │     │  └─ ajax/                 # ajax 控制器（48 actions）
│  │     │     ├─ mod.rs
│  │     │     ├─ cert.rs
│  │     │     ├─ category.rs
│  │     │     ├─ collect.rs
│  │     │     ├─ captcha.rs
│  │     │     ├─ account.rs
│  │     │     ├─ zph.rs
│  │     │     ├─ qrcode.rs
│  │     │     ├─ share_card.rs      # 海报生成
│  │     │     └─ dataviz.rs         # 数据可视化
│  │     └─ member/                  # 会员中心
│  │        ├─ mod.rs
│  │        ├─ user/                 # 个人（usertype=1）
│  │        └─ com/                  # 企业（usertype=2）
│  │
│  ├─ templates/                     # Smarty → askama/maud
│  │  ├─ Cargo.toml
│  │  └─ src/
│  │     └─ ... （仅当需要 SSR HTML 时使用；前后端分离时可跳过）
│  │
│  └─ app/                           # 主 binary
│     ├─ Cargo.toml
│     └─ src/
│        └─ main.rs                  # 启动、路由组装
│
└─ tests/                            # 集成测试
   ├─ auth_test.rs
   ├─ job_test.rs
   └─ ...
```

### 3.1 crate 划分原则

- `core` — 只含基础设施，不含业务
- `auth` — 纯鉴权，不碰业务表
- `models` — Data Access Layer（每个表一个子模块）
- `services` — Business Logic Layer，跨 model 聚合，对应原 PHP 的 `app/model/*.model.php`
- `handlers` — HTTP 适配层，薄；每个 WAP controller 一个 rs
- `app` — 只有 `main.rs`，负责 wire 各层

**分层原则**：依赖单向，`app → handlers → services → models → core`。

---

## 4. 里程碑（Milestones）

> 假设：**1 名 Senior Rust + 1 名 Senior PHP（讲解业务）+ 0.5 名 DBA/DevOps**。  
> 如果是单人全栈，工期 × 1.6。

---

### **M0 — 启动准备**（1 周）

**目标**：团队能开始写第一行业务代码。

#### 任务

- [ ] 搭建 cargo workspace（见 §3 结构）
- [ ] Cargo.toml、`.env.example`、`.gitignore`、README
- [ ] Dockerfile + docker-compose（含 MySQL/Redis 本地副本）
- [ ] `make dev` / `make test` / `make build` 开发命令
- [ ] CI 基础：cargo fmt/clippy/test，MR 必过
- [ ] 技术决策记录（ADR）：JWT 算法、密码哈希、日志格式
- [ ] 开发分支策略：`main` 受保护，PR review required
- [ ] 生产数据库的只读副本（别直连生产）

#### 验收标准

- `cargo run -p app` 能起 HTTP 服务，`curl localhost:3000/health` → `{"ok": true}`
- CI 绿灯
- 新同事 `git clone && docker compose up` 10 分钟内能跑起来

#### 交付物

- 初始化好的 workspace（已有脚手架）
- CI 配置（.github/workflows/ci.yml 或 .gitlab-ci.yml）

---

### **M1 — 核心基础设施**（2 周）

**目标**：把跨业务的"横切关注点"一次性建好。

#### 任务

- [ ] `core::config` — 从 env/toml 加载配置（DB / Redis / JWT key / OAuth secrets / SMS API keys）
- [ ] `core::db` — sqlx MySqlPool，兼容现有 `phpyun_` 前缀
- [ ] `core::redis` — deadpool-redis，Session / 限流 / 短信码存储
- [ ] `core::error::AppError` — `thiserror` 定义，实现 `IntoResponse`；统一 `{code, msg, data}` 响应
- [ ] `core::telemetry` — tracing-subscriber 输出 JSON；Sentry 接入
- [ ] `core::middleware::request_id` — 每请求 UUID，写入响应 header
- [ ] `core::middleware::rate_limit` — IP + uid 粒度，tower_governor
- [ ] `core::middleware::cors` — 配置化 origin
- [ ] 健康检查路由 `/health`、`/ready`
- [ ] Graceful shutdown（`tokio::signal::ctrl_c`）
- [ ] 业务响应类型：`ApiResponse<T>`、分页 `Paged<T>`

#### 验收标准

- 压测 `/health` 单核 > 50k QPS
- 人为触发 panic → 进 Sentry + JSON log 有 request_id
- 限流生效：连续 100 req/s 从同 IP，第 101 个被拒
- `AppError::InvalidCredentials` 自动返回 `{code: 3002, msg: "用户名或密码错误"}`

#### 交付物

- `crates/core/` 完整实现 + 单测
- 基础中间件覆盖率 > 80%

---

### **M2 — 鉴权与用户系统**（3 周）

**目标**：能登录、能注册、能找密码。这是所有其他功能的前提。

#### 任务

- [ ] `auth::password` — argon2id hash/verify；md5 兼容函数
- [ ] `auth::jwt` — `Claims { sub: uid, usertype, did, exp, jti }`，HS256 + `sy_safekey`
- [ ] `auth::middleware::require_auth` — 从 `Authorization: Bearer` 提取
- [ ] `auth::middleware::legacy_cookie` — **关键兼容**：老用户带 `uid/shell` cookie 过来时，校验 shell，现场签发 JWT 写 cookie
- [ ] `auth::middleware::require_usertype(u8)` — RBAC
- [ ] `models::user` — Member 实体 + 仓储（按 uid/username/moblie/email 查）
- [ ] `services::user_service`
  - `user_login(account, pw, captcha)` — 含 md5 → argon2 惰性升级
  - `user_register(form)` — 短信码 / 邮箱码校验
  - `activate_email(code)`
  - `reset_password(mobile_or_email, code, new_pw)`
  - `change_password(uid, old, new)`
  - `bind_mobile(uid, mobile, code)`
  - `bind_email(uid, email, code)`
- [ ] `services::captcha_service`
  - `generate_graph_captcha()` → base64 PNG，key 存 Redis 10min
  - `verify_graph(id, code)`
  - `send_sms_code(mobile, scene)` — Redis 防刷：60s / 5 条每小时
  - `verify_sms_code(mobile, code, scene)`
- [ ] `services::notice_service::send_sms` / `send_email` — 对接 `u.ov6.com` 或新 SMS 提供商（阿里云/腾讯云）
- [ ] `handlers::wap::login`
  - `GET /wap/index.php?c=login&a=index` — **此阶段保留 PHP 渲染 HTML 表单**；Rust 只接 POST
  - `POST /wap/login/mlogin` — 登录提交
  - `POST /wap/login/sendmsg`
  - `GET /wap/login/logout`
- [ ] `handlers::wap::register`
  - 8 个 action 全量实现
- [ ] `handlers::wap::forgetpw`
  - 5 个 action 全量实现
- [ ] `handlers::wap::ajax::account`
  - `sign_action` / `setpwd_action` / `emailcert_action` / `mobliecert_action` / `applytype_action` / `notuserout_action`

#### 验收标准

- **真实用户能在 Rust handler 登录**（POST /wap/login/mlogin 返回 JWT）
- md5 旧密码登录成功，DB 里 `password` 字段自动升级为 `$argon2id$...`
- 短信码不能重用、不能暴力（30 次 /IP /天限制）
- 集成测试 ≥ 50 条：正常登录、密码错、账号锁、短信过期、注册冲突…

#### 交付物

- `crates/auth/`、`crates/models/user/`、`crates/services/{user,captcha,notice}`
- `crates/handlers/wap/{login,register,forgetpw}.rs`
- 测试覆盖率 > 75%

---

### **M3 — 公开内容接口**（2 周）

**目标**：前端所有"不登录就能看"的页面可以走 Rust。

#### 任务

- [ ] `models::{job, company, resume, article, announcement, ask, category, cache}`
- [ ] `services::job_service::search` — 支持所有搜索参数（city/job/salary/exp/edu…）
- [ ] `services::company_service::search`、`get_info`、`get_hot_list`
- [ ] `services::resume_service::search`（公开部分，privacy check）
- [ ] `handlers::wap::`
  - `index` — 首页 + 9 actions（getmq / getCityDomain / about / contact / protocol / privacy / appDown）
  - `job` — 19 actions
  - `resume` — 8 actions（含 privacy 字段过滤）
  - `company` — 2 actions
  - `ask` — 18 actions
  - `article` — 5 actions
  - `announcement` — 1 action
  - `search` — 3 actions
  - `site` — 3 actions
  - `services` — 1 action
  - `ajax::category`、`ajax::qrcode`、`ajax::dataviz`
- [ ] SEO：`<title>` / `<meta keywords>` 等生成逻辑与 PHP 对齐（后台 `sy_webkeyword` 等字段）

#### 验收标准

- 所有列表/详情页接口**响应数据**与 PHP 侧完全一致（diff 工具对比）
- 职位列表分页、排序、筛选参数全覆盖
- P99 延迟 < 80ms（含 DB 查询）

#### 交付物

- `crates/handlers/wap/{index,job,resume,company,ask,article,announcement,search,site,services}.rs`
- 对照测试脚本：100 组查询参数，Rust 响应 == PHP 响应

---

### **M4 — OAuth 第三方登录**（2 周）

**目标**：QQ/微信/微博/极验 全部可用。

#### 任务

- [ ] `services::oauth_service::qq` — 换 token、取 openid/unionid、绑定/注册
- [ ] `services::oauth_service::weixin` — snsapi_userinfo + snsapi_base
- [ ] `services::oauth_service::sina`
- [ ] `services::captcha_service::geetest` — 极验 SDK 调用
- [ ] **state 存 Redis 5 分钟 TTL**（防 CSRF）
- [ ] `handlers::wap::qqconnect` — 3 actions
- [ ] `handlers::wap::wxconnect` — 3 actions
- [ ] `handlers::wap::sinaconnect` — 3 actions
- [ ] `handlers::wap::wxoauth` — 1 action
- [ ] `handlers::wap::geetest` — 1 action

#### 验收标准

- 真实账号完整跑通 QQ、微信、微博登录
- 第三方 openid 绑定到现有用户后，下次直接免密登录
- state 重放攻击被拦

#### 交付物

- OAuth 4 套 + 极验
- 集成测试（mock 第三方 API）

---

### **M5 — 业务模块：招聘相关**（3 周）

**目标**：除简历 / 职位（已完成）外的所有招聘业务都能跑。

#### 任务

- [ ] `models::{part, once, tiny, zph, special, gongzhao, redeem, evaluate, map}`
- [ ] `services::part_service` — 列表、详情、收藏、报名、联系方式二维码
- [ ] `services::once_service` — 店铺招聘（含支付流程，**支付回调仍在 PHP**）
- [ ] `services::tiny_service` — 普工简历（含密码访问）
- [ ] `services::zph_service` — 招聘会：列表、详情、参会企业、职位、预订、企业报名
- [ ] `services::special_service` — 专题招聘
- [ ] `services::gongzhao_service` — 公招
- [ ] `services::redeem_service` — 积分商城：列表、详情、兑换
- [ ] `services::evaluate_service` — 测评：列表、试卷、成绩、留言
- [ ] `services::map_service` — 地理位置 job/company 查询（MySQL 5.7+ `ST_Distance_Sphere`）
- [ ] `handlers::wap::{part, once, tiny, zph, special, gongzhao, redeem, evaluate, map}`

#### 验收标准

- 每个模块的列表 + 详情 + 核心写操作都可用
- 店铺招聘发布（不支付）→ 生成订单记录，等待支付回调（PHP）
- 地图职位按 20km 范围筛选，查询 < 100ms

#### 交付物

- 9 个业务模块 + 对应 handlers
- e2e 测试：从搜索 → 详情 → 操作 完整链路

---

### **M6 — 个人会员中心**（4 周）

**目标**：`usertype=1` 用户所有会员中心功能。

#### 任务

- [ ] **简历管理**（工作 / 教育 / 项目 / 技能 / 培训 / 证书 / 其他）
  - 按 model 拆：`resume_work` / `resume_edu` / `resume_project` / `resume_skill` …
  - CRUD、设置默认、刷新、置顶
- [ ] **个人资料**
  - info 编辑、头像上传、形象照、隐私设置
- [ ] **求职意向（expect）**
  - 多意向、附加意向、意向职位/城市/薪资
- [ ] **求职操作**
  - 收藏职位 / 兼职 / 企业
  - 投递记录（含取消）
  - 谁看了我、我的足迹（浏览历史）
  - 屏蔽企业（黑名单）
  - 职位速配、猎头搜寻
- [ ] **面试邀请**
  - 列表、同意 / 拒绝、屏蔽企业
- [ ] **账号安全**
  - 绑定手机 / 邮箱 / 身份证
  - 改密码、改用户名
  - 账户注销（含密码验证 + 短信二次确认）
  - 账户分离（transfer）
- [ ] **积分 / 任务 / 充值**
  - 签到、任务中心、消费规则
  - 充值（生成订单，PHP 处理支付回调后回写）
  - 账单明细、兑换记录
- [ ] **消息**
  - 系统消息、站内信、求职咨询（commsg）
- [ ] **问答**
  - 我的提问、我的回答、关注的问题、我的评论、点赞

#### 验收标准

- 真实个人账号所有页面 + 写操作都能用
- 和 PHP 版对比，字段 diff < 5%（允许 deprecated 字段差异）

#### 交付物

- `crates/handlers/member/user/` 全量 45 个 actions
- 业务测试覆盖率 > 70%

---

### **M7 — 企业会员中心**（4 周）

**目标**：`usertype=2` 企业用户所有会员中心功能。

#### 任务

- [ ] **企业信息**
  - 基本信息、Logo、Banner、形象照、企业展示
  - 地图定位、工作地址管理（多地址）
  - 认证：营业执照 / 身份证 / 法人
- [ ] **职位管理**
  - CRUD（发布 / 编辑 / 删除）
  - 上架 / 下架 / 刷新
  - 职位推广（置顶、紧急、推荐）
  - 自动刷新设置
  - 批量操作
- [ ] **简历搜索与操作**
  - 搜索、查看、下载简历
  - 人才库（收藏）
  - 面试邀请（含模板管理 yqmb）
  - 黑名单、屏蔽
- [ ] **兼职管理**
  - 发布、编辑、上下架
  - 报名列表、处理报名
- [ ] **招聘会 / 专题**
  - 企业报名、摊位选择
- [ ] **订单 / 套餐 / 积分**
  - 充值、生成订单
  - 账单明细、退款记录
  - 会员套餐管理、积分任务
- [ ] **HR 子账号**
  - 添加 / 编辑 / 删除子账号
  - 权限分配（role-based）
- [ ] **统计报表**
  - 今日数据、趋势图（近 7/30 天）
  - 职位浏览 / 投递 / 下载数据
- [ ] **消息**
  - 求职咨询、系统消息、站内信
  - 对我感兴趣

#### 验收标准

- 企业端所有核心流程（发职位 → 搜简历 → 邀请面试）走通
- HR 子账号登录只能看到授权范围
- 订单生成 → PHP 支付回调 → Rust 侧状态更新

#### 交付物

- `crates/handlers/member/com/` 全量 53 actions
- 企业场景 e2e 测试

---

### **M8 — 文件上传与零碎接口**（2 周）

**目标**：剩下的小模块收尾。

#### 任务

- [ ] `services::upload_service`
  - 统一 `upload_image(bytes, kind, uid)` 接口
  - 白名单后缀 `.jpg/.jpeg/.png/.gif/.bmp/.webp`
  - magic bytes 校验（`infer` crate）
  - 图片压缩、生成缩略图（`image` crate）
  - 水印（企业 logo 场景）
  - OSS 支持（阿里云 SDK for Rust）
- [ ] 扫码上传流程：
  - PC 生成 token 二维码
  - 手机端扫码进入 `/wap/upload/p?t=<token>`
  - 上传 → 回写到原 session
- [ ] `handlers::wap::upload` — 4 actions
- [ ] `handlers::wap::advice` — 3 actions（意见反馈）
- [ ] `handlers::wap::claim` — 2 actions（会员认领）
- [ ] `handlers::wap::reportlist` — 2 actions（举报）

#### 验收标准

- **上传被故意构造的 PHP 脚本文件，返回 400**（扩展名 + magic bytes 双保险）
- 10MB 图片上传 < 2 秒
- 扫码上传流程真实手机可用

#### 交付物

- 上传相关全部接口
- 上传安全红队测试报告

---

### **M9 — 兼容层与灰度切换**（2 周）

**目标**：Rust 和 PHP 并行运行；按 endpoint 粒度切流；可一键回滚。

#### 任务

- [ ] Nginx 分流配置（见 §7）
- [ ] **Cookie ↔ JWT 双栈兼容**：
  - 用户带老 `uid/shell` cookie → Rust 自动签 JWT 写回（`Set-Cookie` + `Authorization` 响应 header）
  - 用户带新 JWT → Rust 同时也写老 cookie（为了 PHP 还在处理的页面也认得）
- [ ] **数据一致性校验**：
  - 同一请求，让 Rust + PHP 并行处理（shadow mode），对比响应
  - 发现 diff 写入 `diff_log` 表 + Sentry
- [ ] **切流开关**：
  - Redis key `route_rust:/wap/job/list` = `1/0` 决定该路由走 Rust 还是 PHP
  - 管理后台加一个页面管理开关（可简单 CLI）
- [ ] **回滚预案**：
  - 任一路由切回 PHP 只需改 Redis key（Nginx reload 可选）
  - 故障恢复演练

#### 验收标准

- 同一账号，切换 Rust ↔ PHP 之间登录态不丢失（双 cookie/token 都认）
- 把任意一个路由从 Rust 切回 PHP < 10 秒
- 灰度 1% → 10% → 100% 可控

#### 交付物

- 切流开关 + 管理 UI
- 回滚手册

---

### **M10 — 压测、安全加固、上线**（2 周）

**目标**：生产级部署 + 可观测。

#### 任务

- [ ] **压测**：
  - wrk / k6 模拟 1000 QPS 30 分钟
  - 混合读写：70% 读 / 30% 写
  - 指标：P50/P95/P99 延迟、错误率、CPU/内存
- [ ] **安全审计**：
  - `cargo audit` — 依赖漏洞
  - SQL 注入红队测试
  - XSS / CSRF 测试
  - 文件上传红队
  - JWT 伪造测试
- [ ] **部署**：
  - Dockerfile 多阶段构建（musl 静态）
  - systemd service 或 k8s Deployment
  - 反向代理：Nginx → Rust upstream
  - 零停机部署（socket reuse）
- [ ] **监控**：
  - Prometheus exporter（请求量、延迟、错误率）
  - Grafana 仪表板
  - Alertmanager 告警（错误率 > 1% / P99 > 500ms）
  - Sentry 错误聚合
- [ ] **日志**：
  - stdout JSON 日志 → Loki / ELK
  - 审计日志：登录、改密码、关键写操作
- [ ] **灰度发布**：
  - 先切 1% 流量 24 小时观察
  - 逐步 10% → 50% → 100%

#### 验收标准

- 生产压测 1000 QPS 无错误
- 所有 Sentry Critical 事件 < 0.1%
- 100% 流量跑 Rust 一周无 P1 事故

#### 交付物

- 生产部署脚本 / 镜像
- Grafana 仪表板 JSON
- 压测报告
- 上线 runbook + 回滚 runbook

---

## 5. 数据库与数据兼容

### 5.1 策略：**不迁表**

- Rust 和 PHP 连同一个 MySQL
- 表名保留 `phpyun_` 前缀
- 字段类型、索引、约束原封不动
- 唯一新增的 migration：性能索引（Rust 查询模式与 PHP 略有不同）

### 5.2 `sqlx offline` 模式

```bash
# 开发机：
cargo sqlx prepare --workspace  # 生成 sqlx-data.json

# CI / 生产：
SQLX_OFFLINE=true cargo build --release
```

生产构建不连 DB，用 sqlx-data.json 离线校验 SQL。

### 5.3 关键表映射

| PHPYun 表 | Rust module |
|---|---|
| `phpyun_member` | `models::user::member` |
| `phpyun_resume` + 6 附表 | `models::resume::{main, work, edu, project, skill, cert, train, other}` |
| `phpyun_company`, `phpyun_company_cert` | `models::company::{info, cert}` |
| `phpyun_company_job` | `models::job::job` |
| `phpyun_partjob` | `models::part::partjob` |
| `phpyun_once` | `models::once::once` |
| `phpyun_tiny_resume` | `models::tiny::resume` |
| `phpyun_zph_info`, `phpyun_zph_com` | `models::zph::{info, com}` |
| `phpyun_special` | `models::special::special` |
| `phpyun_admin_config` | `models::common::admin_config` |
| `phpyun_admin_reg_config` | `models::common::reg_config` |

### 5.4 migration 示例

```sql
-- migrations/20260501_add_perf_indices.sql
CREATE INDEX idx_job_status_lastupdate 
  ON phpyun_company_job(status, lastupdate DESC);

CREATE INDEX idx_resume_uid_defaults 
  ON phpyun_resume_expect(uid, defaults);

CREATE INDEX idx_member_login 
  ON phpyun_member(username(20), moblie(15), email(32));
```

### 5.5 读写分离（可选，M10 做）

- 主 MySQL 写 + 读
- 1 个 MySQL 从库只读（搜索、列表）
- 在 `core::db` 里抽象 `db.read() / db.write()`

---

## 6. 鉴权迁移策略

### 6.1 过渡期：双栈并存

```
┌─ 新用户（第一次登录用 Rust）──────┐
│  登录 → Rust 签 JWT                │
│  → Set-Cookie: token=...; HttpOnly │
│  → 也回写老 cookie: uid, shell     │ ← 兼容还在 PHP 的页面
│                                      │
└──────────────────────────────────────┘

┌─ 老用户（cookie 里已有 uid/shell）──┐
│  请求到 Rust → 检测到 shell cookie │
│  → 验证 shell == md5(user,pw,salt) │
│  → 立即签发 JWT，写两套 cookie     │
│                                      │
└──────────────────────────────────────┘
```

### 6.2 JWT 格式

```json
{
  "sub": 12345,          // uid
  "usertype": 1,         // 1=个人, 2=企业
  "did": 0,              // 分站
  "iat": 1714800000,
  "exp": 1715404800,     // 7 天
  "jti": "uuid-v7"       // 用于吊销
}
```

- **签名**：HS256，secret 用 `sy_safekey`（现有配置里就有）
- **吊销**：jti 入黑名单 Redis key，TTL = exp - now

### 6.3 密码兼容

```rust
pub fn verify_password(pw: &str, hash: &str, salt: &str) -> bool {
    if hash.starts_with("$argon2") {
        // 新格式
        argon2::PasswordHash::new(hash)
            .and_then(|h| Argon2::default().verify_password(pw.as_bytes(), &h))
            .is_ok()
    } else {
        // 老格式：md5(md5($pw) . $salt)
        let md5_pw = format!("{:x}", md5::compute(pw));
        let combined = format!("{}{}", md5_pw, salt);
        format!("{:x}", md5::compute(&combined)) == hash
    }
}

pub async fn upgrade_password_if_legacy(
    pool: &MySqlPool, uid: u64, pw: &str, old_hash: &str
) -> Result<()> {
    if !old_hash.starts_with("$argon2") {
        let new = argon2_hash(pw)?;
        sqlx::query!("UPDATE phpyun_member SET password=? WHERE uid=?", new, uid)
            .execute(pool).await?;
    }
    Ok(())
}
```

---

## 7. Nginx 分流与灰度

### 7.1 方案 A — 按路径前缀（粗粒度）

```nginx
# /www/server/panel/vhost/nginx/zzzz.com.conf

# 已 Rust 化的路由
location ~ ^/wap/(login|register|forgetpw)(/|\b) {
    proxy_pass http://127.0.0.1:3000;
    proxy_set_header Host $host;
    proxy_set_header X-Real-IP $remote_addr;
    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
}

# 其他保留 PHP
location ~ \.php$ {
    fastcgi_pass unix:/run/php/php-fpm-74.sock;
    ...
}
```

### 7.2 方案 B — `map` 指令（中粒度）

```nginx
map $uri $upstream {
    default            php_backend;
    ~^/wap/login       rust_backend;
    ~^/wap/register    rust_backend;
    ~^/wap/job/list    rust_backend;
    ~^/wap/resume/list rust_backend;
}

location / {
    if ($upstream = rust_backend) {
        proxy_pass http://127.0.0.1:3000;
        break;
    }
    # 否则走 PHP
    include fastcgi_params;
    fastcgi_pass unix:/run/php/php-fpm-74.sock;
}
```

### 7.3 方案 C — Rust 做反代（细粒度 + 灰度）

```
Browser ──→ Nginx (443/TLS) ──→ Rust Gateway (3000)
                                      │
         ┌────────────────────────────┼──────────────────┐
         │                             │                   │
         │ 已 Rust 化的业务逻辑        │ 未 Rust 化的       │
         │ (内部 dispatch)             │ → 转发 PHP-FPM    │
         │                             │ 127.0.0.1:9000    │
```

Rust gateway 读 Redis 开关决定：
```
GET /wap/job/list → Rust 自己处理
GET /wap/something_not_ready → 转发到 PHP-FPM
```

**推荐：M9 阶段用方案 C**，因为最灵活，随时回滚。

### 7.4 回滚

- **方案 A/B**：改 nginx conf + `nginx -s reload`（~5s）
- **方案 C**：改 Redis key（~instant）

---

## 8. 测试策略

### 8.1 金字塔

```
        ┌──────────────┐
        │  E2E (10%)   │  真实浏览器 / curl 全流程
        ├──────────────┤
        │  Integration │  handler 级，真 DB/Redis
        │    (30%)     │
        ├──────────────┤
        │  Unit (60%)  │  service/model 单元
        └──────────────┘
```

### 8.2 测试技术

| 层次 | 工具 | 示例 |
|---|---|---|
| 单元 | `#[tokio::test]` + `mockito` | `test_argon2_verify()` |
| 集成 | `axum-test` + `testcontainers-rs` | 真启动 handler，真 MySQL container |
| 对照 | 自己写 diff 工具 | Rust vs PHP 响应 diff |
| 压测 | `k6` / `wrk` | `k6 run scripts/job_list.js` |
| 安全 | `cargo audit`, `cargo deny` | 依赖漏洞扫描 |

### 8.3 Rust ↔ PHP 对照测试（关键）

```rust
// tests/parity_test.rs
#[tokio::test]
async fn job_list_parity() {
    let queries = read_golden_queries("tests/fixtures/job_list.json");
    for q in queries {
        let php_resp = http_get(&format!("http://d.com/wap/index.php?c=job&a=list&{}", q.params)).await;
        let rust_resp = http_get(&format!("http://localhost:3000/wap/job/list?{}", q.params)).await;
        assert_json_eq_ignoring(&php_resp, &rust_resp, &["_time", "_request_id"]);
    }
}
```

把 100 组真实查询存 JSON，CI 跑 parity。

### 8.4 覆盖率目标

- service 层 > 85%
- handler 层 > 70%
- 全局 > 75%
- `cargo tarpaulin` 生成 coverage，CI 跟踪

---

## 9. 部署策略

### 9.1 环境

```
dev          本地开发 (OrbStack VM)
staging      仿真生产 (单独一台机)
prod         生产环境
```

### 9.2 构建

```dockerfile
# deploy/Dockerfile
FROM rust:1.84 AS builder
WORKDIR /app
RUN apt-get update && apt-get install -y musl-tools
RUN rustup target add x86_64-unknown-linux-musl
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release --target x86_64-unknown-linux-musl --bin app

FROM alpine:3.20
RUN apk add --no-cache ca-certificates tzdata
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/app /usr/local/bin/
COPY sqlx-data.json /
EXPOSE 3000
CMD ["app"]
```

### 9.3 systemd（aapanel 场景）

```ini
# /etc/systemd/system/phpyun-rs.service
[Unit]
Description=PHPYun Rust Backend
After=network.target mysqld.service

[Service]
Type=simple
User=www
Group=www
WorkingDirectory=/www/wwwroot/phpyun-rs
EnvironmentFile=/www/wwwroot/phpyun-rs/.env
ExecStart=/www/wwwroot/phpyun-rs/app
Restart=on-failure
RestartSec=5s
LimitNOFILE=65535

[Install]
WantedBy=multi-user.target
```

### 9.4 零停机部署

两个 Rust 实例（3000 和 3001），Nginx upstream 做健康检查，滚动重启一个。

---

## 10. 监控与运维

### 10.1 指标（Prometheus）

```
http_requests_total{method, route, status}
http_request_duration_seconds{method, route}
db_pool_size
db_query_duration_seconds
redis_commands_total
sqlx_errors_total
argon2_verify_duration_seconds
```

### 10.2 仪表板（Grafana）

- 总 QPS / 错误率 / P99 延迟
- 按路由的延迟分布
- DB 连接池使用率
- 最慢的 10 个接口
- JWT 签发/校验速率

### 10.3 告警（Alertmanager）

| 条件 | 严重 |
|---|---|
| error rate > 1% 持续 5min | P1 |
| P99 > 500ms 持续 10min | P2 |
| DB pool exhausted | P1 |
| memory > 80% 持续 10min | P2 |
| 500 ↑ 持续 1min | P0（立即回滚） |

### 10.4 日志

- stdout → Loki / ELK
- 级别：生产 INFO，开发 DEBUG
- 必记字段：`request_id`、`uid`、`route`、`method`、`status`、`duration_ms`

### 10.5 Sentry 接入

- 所有 `AppError::InternalServerError` 自动上报
- 业务级错误（4xx）默认不报
- 携带 breadcrumb：DB 查询、Redis、HTTP client

---

## 11. 风险清单与缓解

| # | 风险 | 影响 | 缓解 |
|---|---|---|---|
| 1 | PHP 和 Rust 对同一字段写入时机不同 → 数据不一致 | 高 | MySQL 事务；灰度期间同一实体写操作只走一侧 |
| 2 | 旧 md5 密码升级 argon2 过程异常 → 用户登不进 | 高 | try/catch，失败时不阻断登录；写 Sentry |
| 3 | 支付回调仍在 PHP，Rust 改了订单状态 → 对账错 | 高 | 订单状态字段只 PHP 写；Rust 只读 |
| 4 | Smarty 模板生成的 HTML 和 Rust 渲染 diff | 中 | M3 之前决定：前后端分离 or SSR |
| 5 | 后台信标 `init.ov6.com` JS 残留 | 中 | Rust 返回的 HTML 清理掉信标 script |
| 6 | 老 cookie `shell` 在浏览器里存在 → XSS 可盗 | 中 | M9 灰度结束后服务器主动 Set-Cookie 清掉 `shell` |
| 7 | sqlx offline 模式 sqlx-data.json 过期 | 低 | pre-commit hook 检查，CI 校验 |
| 8 | 第三方 SMS / OAuth 密钥泄漏 | 高 | 用 env + Vault；绝不提交到 git |
| 9 | 压测数据写生产 DB | 高 | staging 独立 DB；压测前做 schema dump |
| 10 | 团队 Rust 经验不足 | 高 | M0 做 Rust 培训；关键模块 pair programming |
| 11 | 生产 OrbStack VM 容量（已见 swap 1GB）| 中 | 上生产前迁到独立服务器 |
| 12 | 跨部门协调（PHP → Rust 切流）| 中 | 灰度先切 1%，留 24h 观察窗 |

---

## 12. 工作量估算与排期

### 12.1 人/周 估算

| M | 内容 | 1 人全栈 | 2 人并行 | 3 人并行 |
|---|---|---|---|---|
| M0 | 启动 | 1.5w | 1w | 1w |
| M1 | 核心基础设施 | 3w | 2w | 1.5w |
| M2 | 鉴权 + 用户 | 5w | 3w | 2w |
| M3 | 公开内容 | 3w | 2w | 1.5w |
| M4 | OAuth | 3w | 2w | 2w |
| M5 | 业务模块招聘 | 5w | 3w | 2w |
| M6 | 个人会员中心 | 6w | 4w | 3w |
| M7 | 企业会员中心 | 6w | 4w | 3w |
| M8 | 上传 + 零碎 | 3w | 2w | 1.5w |
| M9 | 兼容切换 | 3w | 2w | 2w |
| M10 | 压测上线 | 3w | 2w | 2w |
| **合计** | | **41.5w** | **27w** | **21.5w** |

> **1 人全栈 ≈ 10 个月**  
> **2 人并行（Senior+Senior）≈ 6.5 个月**  
> **3 人并行（1 Senior + 2 Mid）≈ 5 个月**

### 12.2 关键路径

```
M0 → M1 → M2 ──┬─→ M3 ──┬──→ M5 ──→ M6 ──┐
               │        │                  ├──→ M9 → M10
               └─→ M4 ──┘              M7──┘
                                           └──→ M8 ──┘
```

M2 是瓶颈，所有业务模块都依赖鉴权。

### 12.3 推荐节奏

**最低限度跑起来（MVP）**：M0 + M1 + M2 + M3 = **约 3 个月**，可以替换登录 + 首页 + 搜索等"压力最大 + 安全最敏感"的路径。

**全量替换**：M0-M10 = **6~10 个月**。

**务实建议**：**做到 M5 就停**（5 个月），剩下的会员中心业务继续跑 PHP，但所有"未登录公开接口"和"登录注册"都是 Rust。这能吃到 80% 的收益，工期减半。

---

## 13. 交付物清单

### 代码

- [ ] Rust workspace（见 §3）
- [ ] sqlx-data.json（offline 构建用）
- [ ] 测试代码覆盖率 > 75%
- [ ] CI/CD 配置

### 文档

- [ ] [API_SPEC.md](WAP_API_SPEC.md) 已有
- [ ] 本 [PROJECT_PLAN.md](PROJECT_PLAN.md)
- [ ] ADR/001-jwt-algorithm.md
- [ ] ADR/002-password-hashing.md
- [ ] ADR/003-template-engine-or-spa.md
- [ ] ADR/004-router-gateway-vs-nginx.md
- [ ] RUNBOOK.md — 上线流程
- [ ] ROLLBACK.md — 回滚流程
- [ ] README.md — 上手指引
- [ ] 每个 crate 的 `lib.rs` 顶部文档

### 部署

- [ ] Dockerfile + docker-compose.yml
- [ ] systemd unit file
- [ ] Nginx 配置模板
- [ ] Grafana 仪表板 JSON
- [ ] Prometheus 告警规则

### 运营

- [ ] 压测报告（含 P50/P95/P99、TPS、错误率）
- [ ] 安全审计报告
- [ ] 对照测试报告（Rust vs PHP）

---

## 14. 团队与协作

### 14.1 角色

- **Tech Lead / Senior Rust**（1 人）：架构决策、Review、M0/M1/M9/M10 主导
- **Senior Rust + PHP**（1 人）：业务迁移，M2-M8
- **DevOps**（0.5 人）：CI/CD、监控、部署
- **QA**（0.5 人，后期）：压测、红队
- **产品 / 业务**（兼职）：字段语义、业务规则确认

### 14.2 协作流程

- 分支：`main` (protected) → `feat/*`
- PR 必须：CI 绿 + 1 reviewer approve
- 代码规范：`cargo fmt` + `cargo clippy -- -D warnings` 强制
- Commit message：Conventional Commits（`feat:`, `fix:`, `docs:`, …）

### 14.3 里程碑评审

每个 M 结束 demo + retro：
- 演示：跑一下新功能
- 度量：测试覆盖率、性能、bug 数
- 复盘：下一个 M 的风险调整

---

## 附录 A：M0 脚手架

见同仓库 [phpyun-rs/](phpyun-rs/) 子目录（cargo workspace 已初始化）。你可以：

```bash
cd /www/wwwroot/zzzz.com/phpyun-rs
cargo build        # 应该能过
cargo run -p app   # localhost:3000 起一个 /health 服务
```

---

## 附录 B：决策清单（需和业务对齐）

以下几个决策会显著影响工期，**建议项目启动前先敲定**：

1. **前后端分离 or SSR？**
   - 分离（Rust 只出 JSON）：M3 -30% 工期，但 SEO 要 Nuxt/Next.js 接管
   - SSR（Rust 用 askama/maud 渲染 HTML）：M3 +30%，但 SEO 无痛

2. **保留 `/admin/` PHP 管理后台吗？**
   - 保留：工期 -30%
   - 重写：+3 个月

3. **保留老 URL 格式（伪静态）？**
   - 保留：Rust handler 要处理 `/job/list/1-2-3-4-5-6-7-8.html` 这种 URL
   - 新格式：只需要处理 `/api/v1/jobs?page=1&city=1...`

4. **SMS / OAuth 要不要换供应商？**
   - `u.ov6.com` 现在是"二次分发者"的（见安全审计），**强烈建议换成阿里云短信**
   - QQ/微信 可保留原 appid，这是你自己的

5. **MySQL 要不要升级到 8.0？**
   - 当前是 5.x（看 `$db->mysql_server()` 返回值）
   - 8.0 有 window function、JSON 原生支持，sqlx 体验更好

---

**文档版本**：v1.0  
**维护者**：待填  
**下一版计划**：M0 结束后根据实际工期调整估算
