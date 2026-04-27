# PHPYun 安全审计完整报告

**审计目标:** `/www/wwwroot/zzzz.com/uploads/`
**代码规模:** 1526 PHP 文件,~4500 二进制/图片/字体文件,合计 130MB
**审计日期:** 2026-04-26
**说明:** `ov6.com` 已确认为站长自有域名,本报告中 `ov6.com` 相关流量视作正常一方基础设施,不计入风险。

---

## 0. 总体结论

### 没有发现注入式后门
经过对 1526 个 PHP 文件、~4500 个二进制/图片/字体文件、所有 `.htaccess`/隐藏文件/双扩展文件的系统性扫描,**未发现 webshell、China Chopper、c99/r57/wso/b374k/FilesMan 等注入痕迹**:

- 图片/字体/SWF/DLL/SVG 中均无嵌入的 `<?php` 代码
- 所有 263 个用户上传文件 `file` 命令验证均为合法图片,无 magic-byte 不匹配
- 没有 `.zip`/`.rar`/`.tar`/`.phar` 等可疑归档
- `data/templates_c/` 目录下 104 个 Smarty 编译缓存文件 100% 包含合法 Smarty 头部
- 3 个 `.dll` 文件均为 PHPYun 上游打包的 OpenSSL Windows 库(`libeay32.dll`/`ssleay32.dll`/`php_openssl.dll`),Linux 不可执行
- 6 个 `.swf` 文件均为合法上传组件
- `.user.ini` 内容为 `open_basedir=/www/wwwroot/zzzz.com/:/tmp/`,系合法的 PHP 加固

### 但存在严重的"设计型"安全问题
PHPYun 本身是 2010 年代风格的传统 CMS,**框架设计层面的安全债极其严重**——加密、密码哈希、SQL 拼接、Cookie 标志位、CSRF 开关、上传过滤、权限验证等多处不符合 2020 年代标准。如果你不打算重构 PHP 端(仓库中已有 Rust 重构在进行),最低限度需要立即修复"严重"级别的若干处。

---

## 1. 严重 (Critical) — 立即处理

### CR-1. 安装目录可重新安装,导致整站被接管
**位置:** `/uploads/install/index.php:1-23`
**问题:** `install/index.php` 不检查任何 lock 文件。`data/phpyun.lock` 虽存在但只在 `install/php7/install.php` 内被检查,绕过 `index.php` 直接访问 `https://zzzz.com/install/php7/install.php` 时虽会被拦截,但 `index.php` 入口完全无防护——如果某次备份恢复操作误删 `phpyun.lock`,则攻击者可以重新安装、重写 `config/db.config.php` 指向自己的 MySQL,进而获得完全管理员权限。
**修复:** **直接删除整个 `/uploads/install/` 目录**(部署完成后没有任何理由保留)。

### CR-2. UCenter 默认共享密钥未更换
**位置:** `/uploads/data/api/uc/config.inc.php:8`
```php
define("UC_KEY","asaasssaaa");   // 10 字符默认值
define("UC_DBUSER","root");
define("UC_DBPW","root");
```
**问题:** `UC_KEY` 是 PHPYun 安装包的默认值(可被识别),用作 UCenter 跨站 SSO 的 HMAC 密钥。攻击者可使用此密钥构造合法签名的 `code` 参数到达 `/uploads/api/uc/uc.php` / `/uploads/api/uc_php7/uc.php` 中的 SQL sink:
```php
// api/uc_php7/uc.php:75
WHERE `username`='$get[username]'   // pre-auth SQLi
```
**修复:** 如果未启用 Discuz/UCenter,**直接删除 `/uploads/api/uc/`、`/uploads/api/uc_php7/`、`/uploads/data/api/uc/` 三个目录**。如果启用,将 `UC_KEY` 改为 32 字节随机值,并同步到 Discuz 端;同时将 SQL 改为参数化。

### CR-3. 全局密钥 `sy_safekey` 在数据库种子文件中是默认值
**位置:** `/uploads/install/data/phpyun_data.sql:254`
```sql
('sy_safekey', 'b%fB&6#3jg')
```
**问题:** `sy_safekey` 是全站唯一密钥,被用于:
- CSRF/XSS bypass token 验证 (`config/db.safety.php:168`)
- 管理员后台 XSRF token 盐 (`admin/adminCommon.class.php:237,317`)
- 上传 token 加密密钥 (`controller/upload/index.class.php:23`、`wap/upload.class.php:16`)

如果该值未在生产环境更换,攻击者可伪造管理员 XSRF token 与任意用户的上传 token。
**修复:** 立即执行 `UPDATE phpyun_config SET config = '<32 字节随机十六进制>' WHERE name = 'sy_safekey';`,然后强制所有管理员重新登录。

### CR-4. 自更新接口 = 厂商任意 RCE
**位置:** `/uploads/app/controller/upgrade/index.class.php:55-70`
**问题:** 管理员点击"升级"会从远程 API 拉取 zip 包并覆盖线上 PHP 文件;`down_file` 被 `CURLOPT_SSL_VERIFYPEER=FALSE` 调用,中间人也能投毒。
**修复:** 移除整个 `app/controller/upgrade/` 目录,或在路由层禁用该控制器。

### CR-5. 360 webscan 自更新与远程文件读取
**位置:**
- `/uploads/app/include/webscan360/360safe/360webscan.php:262` — 自动从远端拉 `update_360.dat` 覆盖自己
- `/uploads/app/include/webscan360/360safe/360scan.php:31-34, 348-369` — POST `key=<WEBSCAN_KEY>&act=sendfile&filename=<base64>` 可读取 `DOCUMENT_ROOT` 下任意文件

**修复:** 直接删除 `app/include/webscan360/` 整个目录(此 WAF 早已停止维护,且本身就是远程后门通道)。

### CR-6. WAP 支付宝 RSA 回调存在 `$total_fee` 未赋值的支付绕过
**位置:** `/uploads/api/wapalipay/notify_url.php:84-115`
**问题:** RSA 分支中 `$total_fee` 从未被赋值就传入 `$apiPay->payAll($data['out_trade_no'], $total_fee, 'wapalipay')`。攻击者重放任意一次合法签名的 WAP 支付宝回调,可让另一笔订单以"金额匹配"状态成功入账。
**修复:** 加入 `$total_fee = $data['total_fee'];` 并在 `payAll` 内强制金额比对、加幂等检查。

### CR-7. `qqlogin.php` 入口绕过全局过滤 + CRLF 注入
**位置:** `/uploads/qqlogin.php:26-28`
```php
header('Location: '.$config['sy_wapdomain'].'/index.php?c=qqconnect&a=qqlogin&code='.$_GET['code']."&state=".$_GET['state']);
```
**问题:** 该入口文件**没有 include `global.php`/`db.safety.php`**,`$_GET['code']`/`$_GET['state']` 完全未过滤就被拼入 `Location:` 头,可注入 CRLF 完成响应分割(`Set-Cookie:` 注入、XSS、缓存投毒)。
**修复:** 顶部加 `require dirname(__FILE__)."/global.php";`,且对 code/state 调用 `urlencode()`。

### CR-8. `api/wxapp/index.php` 与 `api/version/index.php` Pre-auth LFI
**位置:**
- `/uploads/api/wxapp/index.php:38-46` — `require ('member/user/' . $model . '.class.php')`,`$model = $_GET['m']` 完全无过滤
- `/uploads/api/version/index.php:12` — 同样模式

**问题:** 对比 `api/locoy/index.php:4-7` 与 `/uploads/index.php:67-69` 都有 `preg_match` 白名单,这两个分发器是漏洞遗漏。配合日志投毒或前期上传可升级为 RCE。
**修复:** 加上 `if (!preg_match('/^[0-9a-zA-Z_]+$/', $model)) exit;`。

### CR-9. 资源搜索接口的 SQL 注入
**位置:** `/uploads/app/model/resume.model.php:214-333`
```php
$where .= " AND a.`exp`=".$_POST['exp'];
$where .= " AND `$city_col`= ".$_POST['three_cityid'];
```
**问题:** 数字字段拼接,不带引号,绕过 `addslashes`;`safesql()` 的正则用 `\b` 边界,`)/**/UnIoN/**/SeLect` 等 payload 可通过。**Pre-auth、可达**(`POST /index.php?m=resume&c=ajax&a=getMember`)。
**修复:** 全部 `$_POST['xxx']` 用 `intval()` 强制类型;字段名 `$city_col` 改为白名单查表。

### CR-10. 数据库优化器后台 SQL 注入
**位置:** `/uploads/admin/model/tool/database.class.php:268,272`
```php
$this->db->query("REPAIR TABLE `".$_POST['name']."`");
$this->db->query("OPTIMIZE TABLE `".$_POST['name']."`");
```
**问题:** 反引号内拼接 `$_POST['name']`,反引号本身可被关闭(``a` ; SELECT ...; -- ``);若 `secure_file_priv` 允许还可 `INTO OUTFILE` 写 webshell。
**修复:** 用 `^[a-zA-Z0-9_]+$` 白名单校验表名。

### CR-11. WeChat 回调 SSRF
**位置:** `/uploads/app/controller/weixin/index.class.php:14, 17, 377`
**问题:** WeChat 签名验证只在 `echostr` GET 分支检查;POST 分支接受未签名 XML。`file_get_contents("...&location=$latitude,$longitude")` 中 `$latitude`/`$longitude` 来自 XML body,可注入 CRLF 转向内网元数据接口(如阿里云/腾讯云 169.254.169.254)。
**修复:** POST 分支也强制验签,且对所有从 XML 解析出的字段做严格类型校验。

---

## 2. 高 (High)

### H-1. 弱密码哈希 / Cookie 无 HttpOnly
| 位置 | 问题 |
|---|---|
| `app/model/admin.model.php:419` `makePass($pw){ return md5(md5($pw)); }` | 管理员密码无盐双 MD5,GPU 暴破秒级 |
| `app/include/pwdtype/phpyunpass.php:11,14` 用户密码 `md5(md5(pw).salt)` | salt 是 `substr(uniqid(rand()),-6)` ≈ 24 位熵 |
| `app/model/cookie.model.php:29-42` | `setcookie()` 只 5 个参数,**无 HttpOnly、Secure、SameSite** |
| `cookie.model.php:57` | 认证 cookie `shell = md5(username.pass.salt)` 是永久 token,XSS 即等于永久接管 |

**修复:** 全部迁移到 `password_hash($pw, PASSWORD_BCRYPT)` + `password_verify`;`setcookie` 重写为 `setcookie($n,$v,['expires'=>$t,'path'=>'/','secure'=>true,'httponly'=>true,'samesite'=>'Lax'])`。

### H-2. 密码重置流程可暴破
**位置:** `/uploads/app/controller/forgetpw/index.class.php:16-87` (`editpw_action`)
**问题:**
- SMS 验证码 6 位数字、`rand()` 生成、不是 CSPRNG
- `editpw_action` **无验证次数上限**(只有 `sendCode` 限速)
- 验证成功后**不删除 `company_cert` 中的验证码记录**(可重用)

100 万次空间在分钟级可被穷举。
**修复:** 验证成功后立即删除记录;每条码限定 5 次尝试;改用 `random_int()`。

### H-3. 失败登录写入明文密码到管理日志
**位置:** `/uploads/admin/adminCommon.class.php:264`
```php
addAdminLog('管理员：'.$username.'登录失败，登录密码：'.$password)
```
**问题:** 明文密码进入 `phpyun_admin_log`,任何管理员或 DB 备份持有者可见。
**修复:** 永远不要记录密码。只记 username + IP + UA。

### H-4. 自动注册/SMS 注册的初始密码可猜
**位置:** `/uploads/app/model/userinfo.model.php:2262, 2637`
```php
password = mt_rand(111111, 999999)
```
**修复:** 不要发送密码,改用一次性登录链接。

### H-5. CSRF 总开关被一行 DB 配置控制
**位置:** `/uploads/admin/adminCommon.class.php:22`
```php
if ($this->config['sy_iscsrf'] != '2') { /* check token */ }
```
**问题:** 任何 RCE 翻一下 `phpyun_config` 表中的 `sy_iscsrf=2` 即可关闭整个后台 CSRF 保护。
**修复:** 删掉这个开关,或挪到 `config.php` 文件常量。

### H-6. 管理员 XSRF token 熵不足
**位置:** `/uploads/admin/adminCommon.class.php:26`
```php
substr(md5(uniqid().auid.username.ashell), 8, 12)
```
48 位熵,且基于 `uniqid()`(时间戳)。**修复:** `bin2hex(random_bytes(16))`。

### H-7. 上传扩展名黑名单不全
**位置:** `/uploads/app/model/upload.model.php:235,516`、`/uploads/js/ueditor/php/Uploader.class.php:31,50`
当前黑名单:`('php', 'asp', 'aspx', 'jsp', 'exe', 'do')`
**遗漏:** `phtml`、`phar`、`pht`、`php3`、`php4`、`php5`、`php7`、`phps`、`inc`、`htaccess`、`shtml`
**问题:** `is_picself` 检查只搜 `<?php` 字面量;`<?`、`<?=`、`<script language=php>` 全可绕过。
**修复:** 改为白名单 `['jpg','jpeg','png','gif','webp','pdf','doc','docx','xls','xlsx']`。

### H-8. WAP 支付宝异步回调 XXE
**位置:** `/uploads/api/wapalipay/notify_url.php:30,34`
**问题:** `$doc->loadXML($_POST['notify_data'])` 未调用 `libxml_disable_entity_loader(true)`,PHP < 8.0 默认加载外部实体 → XXE / SSRF。
**修复:** 调用前加 `libxml_disable_entity_loader(true);` 或解析时传入 `LIBXML_NONET`。

### H-9. 自带 SQL "WAF" `db.safety.php` 可绕过
**位置:** `/uploads/config/db.safety.php`
**问题:** 基于黑名单正则的过滤(`select`/`union` 等被替换为全角字符),已知绕过:
- `/*!50000select*/`、`sel/**/ect`
- 数字/标识符上下文(`WHERE id=$x`)`addslashes` 完全无效
- `$_COOKIE` 分支过滤最弱(无 union/select 拦截)
- `$_FILES`、`php://input` 完全不过滤
- `qqlogin.php` 等入口 bypass 整套过滤

**修复:** 不要依赖这层 WAF,所有 SQL 改为参数化 / 强类型。

### H-10. 积分/订单无事务,有竞态
**位置:** `/uploads/app/model/integral.model.php:39-72`、`/uploads/app/model/companyorder.model.php:1142`、`/uploads/app/include/ApiPay.class.php:166`
**问题:** `update_once + insert_into` 没有 `BEGIN/COMMIT`(整个 `mysql.class.php` 没有事务封装),并发可双发积分/双激活 VIP。
**修复:** 加 `START TRANSACTION` + 幂等键(`pay_state` CAS)。

---

## 3. 中 (Medium)

| ID | 位置 | 问题 |
|---|---|---|
| M-1 | `/uploads/global.php:6` | 调试日志写入 `/tmp/phpyun_debug.log`(信息泄露) |
| M-2 | `/uploads/data/plus/outside.php:7` | 死 LFI:`include_once("../outside/".intval($_GET['id']).".php")`。当前 `data/outside/` 不存在,但任何上传漏洞造出该目录即可 RCE。**直接删除此文件** |
| M-3 | `/uploads/data/plus/yunimg.php:1-15` | 包含 `global.php` 后基于 `$_GET['ad_id']` 输出 `document.write('...')`,只 `str_replace("'","\'", ...)`。如管理员存的广告内容含 `</script>` 可逃逸。低-中(需管理员投毒) |
| M-4 | `/uploads/admin/model/system/singlepage.class.php:74-95, 244` | `path_tidy()` 允许 `]/[/!/@`,`str_replace('../','',...)` 易绕过,管理员可在 APP_PATH 下写任意 .html |
| M-5 | `/uploads/admin/model/system/model/tmp_class.php:121, 164` | `extract($post)` + 直接 `file_put_contents($tp_path/$name, $content)`;`tpaddsave_action` 不强制 `.htm` 后缀。管理员认证后可写任意 PHP |
| M-6 | `/uploads/admin/model/tool/database.class.php:85,172` | `extract($_POST)`/`extract($_GET)`(管理员 + CSRF 后等于 RCE) |
| M-7 | `/uploads/admin/model/yunying/yingxiao_tuiguang.class.php:1031, 1406` | 同上 |
| M-8 | `/uploads/api/uc_php7/uc_client/data/cache/apps.php:8, 26, 39` | UCenter sync 指向**第三方 `x.ghzpb.com` / `www.ghzpb.com`**(非 ov6.com)。如 UCenter 启用,用户名/密码会被同步出去。**确认未用即清空此文件** |
| M-9 | `/uploads/api/wapalipay/openssl/*.dll` | 上游打包的 Windows DLL,Linux 上不可执行,但混淆审计视图。**建议删除** |
| M-10 | `/uploads/api/wapalipay/key/rsa_private_key.pem`、`alipay_public_key.pem` | 当前仅占位文本;若运维以后填入真私钥,该路径在 webroot 下世界可读。**移到 webroot 外 + chmod 600** |
| M-11 | `/uploads/api/wapalipay/cacert.pem` | 2012-10-18 的 Mozilla CA bundle,13 年未更新,新 LE 证书链可能验签失败 |
| M-12 | `/uploads/api/wapalipay/alipayapi.php:50,54` | dev 残留 `127.0.0.1:8800` 示例回调地址 |
| M-13 | `/uploads/api/wapalipay/lib/alipay_submit.class.php:153,155` | 注释中残留 `5601559c.nat123.net` 内网穿透地址 |
| M-14 | `/uploads/data/api/uc/config.inc.php` | 即使不用 UCenter,`userinfo.model.php:2640` 也会 include 此文件,默认 root/root MySQL 凭据。**整目录删除** |
| M-15 | `/uploads/app/include/des.class.php`、`desjava.class.php` | DES-CBC + key 复用为 IV + `mcrypt_*`(PHP 7.2+ 已删除),8 字节 DES 已被破解几十年 |
| M-16 | `/uploads/app/include/public.function.php:1655-1679` `yunEncrypt()` | 自制 XOR + md5(rand()) 标签从不验证;只加密前 32 字节,后续明文泄露;已知明文可还原 `md5(sy_safekey)` |
| M-17 | `/uploads/app/controller/wap/login.class.php:20-31` | `$_GET['wxid']` 直接写 cookie `wxnickname/wxpic`,模板不转义,**Stored XSS** |
| M-18 | 富文本字段 (`content/body/description/intro/doc/...`) | `common_htmlspecialchars` 调 `RemoveXSS(rawurldecode($v))`;双重解码可让 `%2522%253E%253Cscript%253E...` 存活,**Stored XSS** |
| M-19 | `/uploads/api/wapalipay/interrupt_back_url.php:11` | `header("Location: $Loaction\n");` 末尾 `\n` 可疑,如 `$Loaction` 来自 GET 即响应分割 |
| M-20 | `/uploads/app/include/dbbackup/inc/header.php:21` | 备份工具用 `md5(username.password)` 无盐 token |

---

## 4. 低 (Low) / 信息

- **L-1** `config/db.config.php:11` `coding` 是 32 位静态值 `8383ab339427a01cc193c741656ba8e9`,确认是本站独立生成而非安装包公共值。
- **L-2** `config/db.config.php` 在 webroot 内,如 nginx 误配 `.bak`/`.php~` 可泄漏 DB 凭据。建议加 `location ~* ^/config/ { deny all; }`。
- **L-3** 管理员后台 XSRF token 是按 session 生成,不是按表单——同一 session 内可跨表单重放(影响小)。
- **L-4** `member/index.php:23-41` 与 `/uploads/index.php:22-41` 中 `usertype` 来自 `$_COOKIE`,不过后续 `common::GET_user_shell()` 会校验,目前无利用路径,但属于"信任 cookie 选 controller"的代码味,值得加固。
- **L-5** 5 个 `.swf` 文件虽干净但 Flash 已停止支持,可删除。

---

## 5. 网络出站清单(`ov6.com` 已排除)

### 5.1 第三方厂商出站(用户触发,有合同关系)
| 厂商 | 触发 | 域名 |
|---|---|---|
| WeChat OAuth/JS-SDK | 用户登录/分享 | `api.weixin.qq.com`、`open.weixin.qq.com`、`qyapi.weixin.qq.com`、`mp.weixin.qq.com` |
| QQ Connect | 用户登录 | `graph.qq.com` |
| 新浪微博 | 用户登录 | `api.weibo.com` |
| 百度小程序 | 用户登录 | `spapi.baidu.com` |
| Alipay | 支付 | `openapi.alipay.com`、`mapi.alipay.com`、`wappaygw.alipay.com`、`notify.alipay.com` |
| Geetest | 验证码 | `api.geetest.com` |
| Vaptcha | 验证码 | `0.vaptcha.com` |
| 腾讯云验证码 | 验证码 | `captcha.tencentcloudapi.com` |
| 百度/高德地图 | 地理编码 | `api.map.baidu.com`、`restapi.amap.com` |
| 阿里云 OSS | 文件存储 | `*.aliyuncs.com` |

### 5.2 ⚠️ 第三方非合同关系出站
| 触发 | 域名 | 风险 |
|---|---|---|
| `app/include/public.function.php:388` `getLocalCity` | `whois.pconline.com.cn/ipJson.jsp` | **每次请求把访客 IP 发给太平洋电脑网**(无合同) |
| `api/uc_php7/uc_client/data/cache/apps.php:8` | `x.ghzpb.com`、`www.ghzpb.com` | **UCenter 同步指向陌生第三方**——若启用 UCenter,**用户名密码会同步出去**。立即确认 |
| `app/include/webscan360/360safe/360webscan.php:262` | `safe.webscan.360.cn/papi/update` | 360 自更新,**等于厂商对你 root** |

### 5.3 硬编码 IP(非噪音)
| 位置 | IP | 说明 |
|---|---|---|
| `api/wapalipay/alipayapi.php:50,54` | `127.0.0.1:8800` | 老的 dev 回调示例,无害但应删 |
| `data/plus/config.php:5` | `sy_wxredpack_ip => 127.0.0.1` | 微信红包源 IP,应改为公网 IP |
| `app/include/aliyunemail/aliyun-php-sdk-core/Config.php:46` | `HTTP_PROXY_IP=127.0.0.1` | SDK 默认代理,未启用 |

无任何公网硬编码 IP。

### 5.4 内置硬编码邮件
| 位置 | 值 |
|---|---|
| `api/uc_php7/uc_client/data/cache/settings.php:17,22,23` | `username@21cn.com`(占位符,未启用) |

无可疑收件人。

---

## 6. 文件级清单(已逐文件验证)

| 类别 | 数量 | 状态 |
|---|---|---|
| `.php` 文件总数 | 1526 | 全部扫描,无注入式 webshell |
| `data/templates_c/*.htm.php` Smarty 编译缓存 | 104 | 100% 含合法 Smarty 头部,头部指向 `app/template/` 真源文件 |
| `data/upload/*` 用户上传 | 263 | 全部为合法图片(`file` 验证),无双扩展、无嵌入 PHP |
| 图片(PNG/JPG/GIF/JPEG/WEBP) | 3329 | grep `<?php`/`eval`/`base64_decode`/`system` 全无命中 |
| SVG | 3 | 无 `<script>` / `<foreignObject>` |
| SWF | 6 | 全部为合法上传组件,mtime 2023-11-23 |
| 字体(TTF/WOFF/WOFF2/EOT/OTF) | 24 | 无嵌入 PHP |
| DLL | 3 | OpenSSL Windows 库,Linux 不可执行(应删除) |
| 归档(ZIP/RAR/TAR/PHAR/7Z) | **0** | — |
| `.htaccess` | 1 (`wap/.htaccess`) | 仅 URL rewrite,无危险指令 |
| `.user.ini` | 1 | `open_basedir=/www/wwwroot/zzzz.com/:/tmp/`,合法加固 |
| 其他 dotfile | 2 | `.php_cs`、`.php_cs.dist`(Guzzle 代码风格配置,无害) |
| `data/cache/` | 空 | — |
| `data/backup/` | 仅 `index.html` 占位 | — |
| `data/sensitive/` | 空 | — |
| `data/wxpubtpl/` | 空 | — |
| `data/xml/` | 空 | — |

---

## 7. 优先修复清单(按时间排序)

### 今天必做(< 4 小时)
1. **删除 `/uploads/install/` 整个目录**(CR-1)
2. **删除 `/uploads/api/uc/`、`/uploads/api/uc_php7/`、`/uploads/data/api/uc/`** 若 UCenter 未使用(CR-2、M-8、M-14)
3. **删除 `/uploads/app/include/webscan360/`**(CR-5)
4. **删除 `/uploads/data/plus/outside.php`**(M-2)
5. **执行 SQL** 更新 `sy_safekey` 为 32 字节随机值,踢出所有管理员会话(CR-3)
6. **删除 `/uploads/app/controller/upgrade/` 或在路由层禁用**(CR-4)
7. **修补 `qqlogin.php`** 加 `require global.php` + `urlencode($_GET['code'])`(CR-7)
8. **修补 `/uploads/api/wxapp/index.php` 与 `/uploads/api/version/index.php`** 加 `preg_match` 白名单(CR-8)

### 本周内(< 7 天)
9. **`/uploads/api/wapalipay/notify_url.php`** 修复 `$total_fee` 未赋值 + 加幂等检查(CR-6)
10. **`/uploads/app/model/resume.model.php`** 全部 `intval()` / 字段名白名单(CR-9)
11. **`/uploads/admin/model/tool/database.class.php`** `$_POST['name']` 白名单校验(CR-10)
12. **WeChat 回调** 强制 POST 分支验签(CR-11)
13. **Cookie 全部加上 HttpOnly/Secure/SameSite**(H-1)
14. **`/uploads/api/wapalipay/notify_url.php`** 加 `libxml_disable_entity_loader(true)`(H-8)
15. **登录失败日志移除 `password` 字段**(H-3)

### 本月内
16. 密码哈希迁移到 `password_hash`/`password_verify`(H-1)
17. 密码重置流程整改(token 一次性、加次数限制)(H-2)
18. 上传扩展名改为白名单(H-7)
19. SMS / 重置码改用 `random_int()`(H-2)
20. 积分/订单加 SQL 事务 + 幂等键(H-10)
21. 删除 `data/api/uc/config.inc.php`、`api/wapalipay/openssl/*.dll`、`api/wapalipay/key/*.pem`(M-9、M-10、M-14)
22. 移除 `global.php:6` 的调试 `error_log`(M-1)
23. 替换 `app/include/des.class.php` / `desjava.class.php` / `yunEncrypt()` 为 `openssl_encrypt`(M-15、M-16)

### 长期
24. 整体迁移到正在进行的 Rust 重构(`/www/wwwroot/zzzz.com/phpyun-rs`),逐步停用 PHP 端;新接口已经覆盖 region/country/category 等,继续推进可直接绕过 PHP 大量历史问题
25. nginx 加 `location ~* ^/(config|install|data/api)/ { deny all; }`
26. nginx 禁止解析 `.bak`/`.php~`/`.swp`

---

## 8. 风险等级总览

| 严重 | 高 | 中 | 低 |
|---|---|---|---|
| 11 | 10 | 20 | 5 |

**核心结论:**
- ❌ **不是被植入了后门** —— 没有 webshell、没有 base64 编码的 eval payload、没有图片里藏的 PHP、没有时间异常的可疑文件。
- ✅ **是 PHPYun 框架自身的安全债** —— 弱密码、Cookie 无标志位、大量 SQL 拼接、上传黑名单不全、自更新等于厂商 root 权限、UCenter/360 webscan 等组件携带远程控制能力。
- ⚠️ **需要立即关注的真正可疑点:** UCenter `apps.php` 指向 `x.ghzpb.com` / `www.ghzpb.com`(非站长域名)。**站长应立刻确认这两个域名是否归自己或合作方所有。如否,则可能是历史安装包/上游分发就被预置了第三方同步指向。**

无任何证据指向当前服务器被外部攻击者植入后门;所有"后门感"都来自上游打包的远程控制接口(自更新、360webscan)。最安全的做法是把这些接口连同 `install/` 一并清除,然后让 Rust 重构逐步替换余下逻辑。



没有外部攻击者植入的后门
经过 5 路并行深度审计:

1526 PHP 文件 全部扫过,没有 webshell / China Chopper / 已知 backdoor 签名
3329 张图片 + 字体 + SWF 完全无嵌入 PHP / magic-byte 不匹配
263 个用户上传文件 100% 是合法图片
Smarty 编译缓存 104 个文件全部含合法头部
没有 .zip/.rar/.phar 等可疑归档,没有奇怪文件名,没有时间异常
⚠️ 但发现了 1 处真正需要核实的可疑点
api/uc_php7/uc_client/data/cache/apps.php:8 指向 x.ghzpb.com / www.ghzpb.com(非你的 ov6.com)。请立即确认这两个域名是否你本人/合作方所有。如否,且 UCenter 启用,则用户名密码会被同步过去 —— 这是审计中唯一一个可能是"上游打包预置后门"的真实信号。

❌ 但 PHPYun 框架自身有 11 项严重 / 10 项高危安全债
最严重的几条(详见报告):

/install/ 目录 index.php 无 lock 检查 → 重装即接管
UCenter 默认密钥 asaasssaaa + pre-auth SQLi
sy_safekey 默认值 b%fB&6#3jg
自更新接口 = 厂商任意 RCE
360 webscan 自更新 + 远程文件读取
WAP 支付宝 RSA 回调 $total_fee 未赋值 → 支付绕过
qqlogin.php 入口绕过全局过滤 + CRLF
api/wxapp/、api/version/ pre-auth LFI
resume.model.php SQL 注入(数字字段无引号拼接)
数据库优化器后台 SQL 注入
WeChat 回调 SSRF
