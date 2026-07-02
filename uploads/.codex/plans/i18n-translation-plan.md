# 多语言翻译企业级实施计划

## 1. 目标与原则

本计划针对当前 PHPYun 招聘系统的多语言改造，覆盖 `htm/html/vue` 模板、独立 `js`、`css`、`sql` 初始化数据及现有 PHP 语言包体系。目标不是一次性机械替换中文，而是建立可持续、可验证、可回滚的企业级 i18n 流程。

核心原则：

- 用户可见文本必须进入语言包，禁止新增硬编码中文、英文或中英混排文案。
- 业务逻辑、HTML 结构、JS 行为、CSS 布局、SQL 数据语义必须与翻译改造解耦。
- 每个批次只处理一个明确范围，例如一个目录、一个页面组、一个 API 模块。
- 所有自动迁移必须先 `--dry-run` 或扫描报告确认，再执行写入。
- 翻译质量以语义准确、上下文一致、占位符完整、页面不破版为验收标准。

## 2. 当前项目现状

项目已经具备基础 i18n 能力：

- 后端推荐入口：`yun_t()` 用于普通 key，`lc()` 用于后台/组件命名空间，`yun_auto_t()` 仅用于存量中文兜底迁移，`lcCoin()` 用于金额展示。
- 后端兼容入口：`yun_at()` 是 `yun_t()` 的历史别名，保留兼容但新增代码不推荐；`yun_auto_array()` 主要供 `yun_json_encode()` 等内部流程使用。
- 前端推荐入口：`yunT()` 用于普通 key，`yunLc()` 用于后台/组件命名空间，`yunAutoT()` 仅用于存量中文兜底迁移，`lcCoin()` 用于金额展示。
- 前端兼容入口：`yunAt()` 是 `yunT()` 的历史别名，保留兼容但新增代码不推荐；前端实现位于 `js/yun-i18n.js`。
- 语言包：`data/lang/zh_cn.php`、`data/lang/en_us.php`、`data/lang/auto/zh_cn.php`、`data/lang/auto/en_us.php`。
- 质量脚本：`tools/php_lint_gate.php`、`tools/scan_i18n_status.php`、`tools/scan_hardcoded_php.php`、`tools/scan_wap_zero_zh.php`、`tools/scan_htm_missed_chinese.php`、`tools/scan_vue_remaining.php`。
- 已存在一批迁移工具，但部分工具已经明确禁用，例如 `tools/i18n_admin_html.php` 提示必须改为手工逐文件 `lc()` 绑定。

当前主要风险：

- 自动语言包中存在被误迁移的模板表达式，应逐条修复，避免把 `{{ lc(...) }}` 之类代码当普通译文。
- 部分中文仍散落在 htm、Vue、JS、SQL 和 CSS 注释/字体名中。
- 批量替换容易破坏 Smarty `{yun:}`、Vue `{{ }}`、JS 字符串、SQL 引号和 CSS 选择器。
- 英文翻译需要按业务上下文复核，不能只做字面翻译。

## 3. 语言包架构

### 3.1 Key 分类

建议保留现有双轨结构：

- 结构化 key：用于长期维护的核心文案，例如 `lc.save`、`home.search_placeholder`、`admin.system.cache_clear`。
- 自动编号 key：用于存量迁移和大规模模板改造，例如 `wap_01234`、`admin_00056`。

新增规则：

- 新功能优先使用结构化 key。
- 存量页面迁移可以继续使用自动编号 key，但同一页面内前缀必须稳定，例如 `wap_job_00001` 或 `admin_tool_00001`。
- 禁止把整段 HTML、Vue 表达式、JS 表达式作为语言包 value；语言包只保存纯文本和必要占位符。
- 占位符统一使用 `{0}`、`{1}` 或 `{name}`，不得拼接半句翻译。

示例：

```php
// 推荐
yun_t('job.salary_range', array('{min}' => $min, '{max}' => $max));

// 兼容存量自动 key；新增代码优先 yun_t()
yun_t('wap_job_00031');

// 禁止
'点击 <span>这里</span> 查看 {{ user.name }}'
```

### 3.2 语言回退

- 默认语言：`en_us`。
- 缺失 key 回退：当前语言 -> `en_us` -> `zh_cn` -> key 本身。
- 线上不得出现 key 本身；扫描阶段允许出现，用于定位缺失。
- 新增语言必须补齐 `data/lang/{lang}.php` 和 `data/lang/auto/{lang}.php`。

## 4. 文件类型处理策略

## 4.1 HTM / HTML / Smarty 模板

范围：`app/template/**/*.htm`、`app/template/**/*.html`、`app/template/admin/**/*.vue` 中的模板区。

处理规则：

- 普通文本节点替换为 `{yun:}t key='xxx'{/yun}` 或 Vue 区域内 `{{ lc('xxx') }}`。
- HTML 属性值如 `placeholder`、`title`、`alt`、`value` 需要单独绑定。
- Smarty 变量、循环、条件、过滤器不得被语言工具改写。
- Vue 模板内禁止使用无效 `{yun:}t` 嵌套表达式，统一使用 `lc('key')`。
- 混合文本要拆成文本 key + 变量占位符，不做字符串拼接。

推荐模式：

```html
<input :placeholder="lc('admin_search_placeholder')">
<span>{{ lc('admin_total_count', [total]) }}</span>
<title>{yun:}$config.sy_webname{/yun} - {yun:}t key='page_title'{/yun}</title>
```

禁止模式：

```html
<span>共 {{ total }} 条记录</span>
<span>{{ lc('共') }} {{ total }} {{ lc('条记录') }}</span>
```

执行流程：

1. 用 `rg -n "[\x{4e00}-\x{9fff}]" app/template/...` 找出目标文件中文。
2. 先人工标注文本类型：纯文本、属性、JS 字符串、注释、示例数据。
3. 每次只改一个页面或组件目录。
4. 运行 `php tools/scan_htm_missed_chinese.php` 和相关扫描脚本。
5. 浏览器验证页面渲染、弹窗、表格、分页、空状态。

## 4.2 JavaScript 文件

范围：`js/**/*.js`、`wap/js/**/*.js`、`app/template/admin/**/*.js`、模板内 `<script>`。

处理规则：

- 业务提示、按钮、确认框、错误信息优先用 `yunT()` 或 `yunLc()`；`yunAutoT()` 仅用于存量中文兜底迁移，`yunAt()` 仅作历史兼容。
- 前端存量 `yunAt()` 按文件逐步迁移到 `yunT()`；每个批次只迁一个文件，只做等价函数名替换，不改 key、参数或业务逻辑，并运行 JS 语法检查。迁移前必须确认目标页面先加载 `js/yun-i18n.js` 或已稳定提供 `yunT()`，否则保持 `yunAt()` 兼容入口。
- 后台 Vue 页面优先使用 `lc()`/`yunLc()`，保持与已有后台语言包一致。
- 不翻译变量名、函数名、接口字段、枚举值、CSS class、URL 参数、localStorage key。
- 第三方库、压缩文件、vendor 文件默认跳过，例如 `layui`、`swiper`、`mui`、`ueditor`，除非确认为项目自定义文案。
- 正则、日期格式、金额格式、接口协议不得因翻译改动。

推荐模式：

```js
this.$message.success(lc('admin_save_success'));
confirm(yunT('common.confirm_delete'));
```

禁止模式：

```js
if (status === '已审核') {}
$('.btn').text('保存成功');
```

执行流程：

1. 先排除第三方库目录。
2. 扫描中文字符串，区分 UI 文案、日志、测试数据、注释。
3. 只替换 UI 文案，行为字符串必须保留或改为稳定枚举。
4. 验证控制台无 JS 语法错误。
5. 验证 Ajax 返回文案与前端弹窗语言一致。

## 4.3 CSS 文件

范围：`*.css`、模板内 `<style>`、移动端样式。

处理规则：

- CSS 通常不做语言包替换。
- 可见中文只可能出现在 `content: "中文"`、字体名、注释、图片路径或旧浏览器 hack 中。
- `content` 中的可见文案应移到 HTML/JS 中渲染；CSS 只保留样式。
- 字体名如 `微软雅黑` 可按兼容性保留或替换为字体栈，不进入语言包。
- 注释可翻译为英文或移除，但不得影响选择器和样式声明。
- 不翻译 class/id 名称，即使包含拼音或中文业务缩写。

执行流程：

1. `rg -n "[\x{4e00}-\x{9fff}]" --glob '*.css'`。
2. 将结果分类为 `content`、字体、注释、路径、异常文本。
3. 对 `content` 改造为 HTML 文案；字体和注释按需处理。
4. 移动端页面必须检查按钮宽度、换行、英文长词溢出。

## 4.4 SQL 文件

范围：`install/data/*.sql`、初始化数据、菜单、分类、配置默认值、消息模板。

处理规则：

- SQL 不直接调用运行时翻译函数。
- 数据库中“系统配置、菜单、分类、枚举、模板文案”应存储稳定 key 或同步写入多语言种子表。
- 用户生成内容、历史业务数据、公司名称、职位名称、文章正文不自动翻译。
- 安装脚本中的默认中文必须同步加入语言包，必要时新增 `name_key`、`title_key`、`desc_key` 字段。
- 菜单和导航优先使用已有 `name_key` 模式，显示时由 PHP 层翻译。
- SQL 改动必须保护引号、编码、转义和默认值。

推荐模式：

```sql
-- 菜单保存展示 key，运行时翻译
INSERT INTO admin_navigation (name, name_key, path) VALUES ('职位管理', 'admin_nav_job', '/job');
```

执行流程：

1. 扫描安装 SQL 中文，按表分类。
2. 判断字段性质：系统文案、枚举、用户内容、配置值。
3. 系统文案字段加 key 或配套语言包；用户内容不处理。
4. 本地重装或导入测试库，验证 SQL 可执行。
5. 验证后台菜单、分类、配置页显示语言正确。

## 5. 分阶段实施路线

### Phase 0：冻结规则与基线

- 保存当前扫描结果：`php tools/scan_i18n_status.php --save-baseline`。
- 确认要支持的语言列表：默认 `en_us`，保留 `zh_cn`，新增语言另开任务。
- 标记禁止批量运行的脚本，例如已禁用的 `tools/i18n_admin_html.php`。
- 建立“每批次一个目录、一个报告、一个验证记录”的机制。

### Phase 1：修复基础设施问题

- 修复 `data/lang/auto/en_us.php` 中明显损坏的表达式翻译。
- 增加语言包完整性检查：key 对齐、PHP include 可执行、无数组语法错误。
- 明确 `lc()` 在后台 Vue 中的注入方式，避免页面局部缺少函数。
- 确认前端语言包加载 URL、缓存版本和切换语言刷新策略。

### Phase 2：HTM/WAP 页面收敛

- 优先处理高流量页面：登录、注册、职位列表、简历列表、职位详情、企业详情、会员中心。
- 每页处理文本节点、属性、弹窗、空状态、分页文案。
- 完成后运行 `scan_wap_zero_zh.php` 与人工移动端检查。

### Phase 3：后台 Admin/Vue 收敛

- 以 `app/template/admin` 为单位分模块推进。
- Vue 文件统一 `lc()`，禁止回退到 `{yun:}t`。
- 表格列名、表单 label、校验提示、按钮、弹窗、空状态必须全覆盖。
- 使用 `tools/scan_vue_remaining.php` 做批次验收。

### Phase 4：JS 独立文件收敛

- 优先处理项目自有 JS：`js/public.js`、`js/yun-i18n.js`、`wap/js` 下业务脚本、admin 自有脚本。
- 第三方库列入排除清单，避免误改。
- 所有确认框、toast、layer、message、alert 进入语言包。

### Phase 5：CSS 与视觉适配

- 清理 CSS `content` 中文。
- 对英文长文案做移动端适配：按钮最小宽度、表格列宽、弹窗宽度、换行策略。
- 检查中英文切换后 header、footer、菜单、筛选项不重叠。

### Phase 6：SQL 与安装数据

- 处理 `install/data/phpyun.sql`、`phpyun_data.sql` 中系统文案。
- 菜单、分类、配置默认提示迁移为 key 或多语言种子。
- 安装流程跑通后，比较新库与旧库关键表字段兼容性。

### Phase 7：发布门禁

每个批次完成前必须通过：

```bash
php tools/php_lint_gate.php
php tools/scan_i18n_status.php
php tools/scan_hardcoded_php.php
php tools/scan_wap_zero_zh.php
```

前端批次额外检查浏览器控制台、页面渲染和语言切换。SQL 批次必须执行导入测试。

## 6. Agent / Skills 工作规则

### 6.1 Agent 角色分工

- `i18n-architect`：维护 key 命名、语言包结构、回退策略和跨模块一致性。
- `template-migrator`：处理 `.htm/.html/.vue`，只改展示文案，不改业务逻辑。
- `js-migrator`：处理项目 JS 文案，保护接口字段、枚举、函数和事件逻辑。
- `css-reviewer`：检查 CSS 可见文本和多语言布局风险。
- `sql-seeder`：处理安装 SQL、菜单 key、配置种子数据。
- `qa-gatekeeper`：运行扫描、语法检查、页面验证和差异复核。

### 6.2 Agent 必须遵守的硬规则

- 修改前必须先读目标文件上下文，不能只基于搜索结果替换。
- 禁止跨目录大批量替换，除非脚本支持 dry-run 且人工审核 diff。
- 禁止翻译代码标识符：变量、函数、类名、字段名、路由、URL、class、id、枚举值。
- 禁止改动第三方库，除非先登记到“允许改动清单”。
- 禁止把 HTML、Vue、JS 代码片段写进语言包 value。
- 任何包含 `{}`、`%s`、`%d`、HTML 标签、Vue 表达式的文案必须人工确认占位符。
- 每次提交必须说明处理范围、运行的检查命令、剩余风险。
- 发现语言包损坏、语法错误、页面白屏时，立即停止扩展范围，先修复基础问题。

### 6.3 Skills 使用条件

- 使用 `repo-inspection` 能力：先扫描目录、现有函数、脚本说明和提交历史。
- 使用 `php-i18n` 能力：新增代码优先调整 `yun_t/lc/lcCoin` 调用及语言包 key；仅维护存量时保留 `yun_at`。
- 使用 `frontend-vue-i18n` 能力：处理 Vue 模板、属性绑定、Element UI 文案、表单校验。
- 使用 `static-js-i18n` 能力：处理 alert、confirm、message、layer、toast、动态 DOM 文案。
- 使用 `sql-seed-i18n` 能力：处理安装 SQL 与系统种子数据。
- 使用 `qa-validation` 能力：运行 PHP lint、i18n scanner、页面抽检和 SQL 导入测试。

如果当前 Agent 不具备对应 skill，必须降级为只输出分析和待办，不允许直接改文件。

## 7. Key 命名规范

建议逐步从纯编号 key 过渡到可读 key：

- 公共：`common.save`、`common.cancel`、`common.confirm_delete`。
- WAP：`wap.job.search_placeholder`、`wap.resume.expected_salary`。
- 后台：`admin.system.cache_clear`、`admin.user.audit_status`。
- API 消息：`api.auth.login_required`、`api.resume.update_success`。
- SQL 种子：`seed.nav.job_manage`、`seed.config.site_name_label`。

存量自动 key 继续保留，但新增文案不得无上下文生成 `common_99999`。

## 8. 翻译质量规范

- 招聘业务术语统一：职位 `Job`、简历 `Resume`、企业 `Company`、会员 `Member`、套餐 `Package`、积分 `Points`。
- 状态词统一：待审核 `Pending Review`、已通过 `Approved`、未通过 `Rejected`、已下架 `Offline`。
- 操作词统一：新增 `Add`、编辑 `Edit`、删除 `Delete`、刷新 `Refresh`、提交 `Submit`。
- 英文 UI 保持简短，按钮通常 1-3 个词。
- 错误提示说明原因和操作，例如 `Please select a resume first.`。
- 不翻译品牌名、第三方平台名、专有配置 key。

## 9. 验收清单

每个批次完成后检查：

- PHP 无语法错误。
- 语言包 PHP 文件可正常 include。
- 当前批次无新增硬编码中文。
- `en_us` 与 `zh_cn` key 对齐，且 `en_us` 不缺默认展示文案。
- 占位符数量一致。
- 页面中无裸 key、无 `undefined`、无空按钮。
- 浏览器控制台无 JS 错误。
- 中英文切换后布局不重叠、不截断关键操作。
- SQL 批次可完整导入测试库。
- diff 中没有第三方库、缓存、上传文件的无关改动。

## 10. Admin d18 大面积后台收敛批次

本轮默认语言继续固定为 `en_us`，`zh_cn` 保留中文翻译。处理范围为管理后台用户可见文案和中文注释；第三方库、协议字典、接口字段、枚举值、CSS class/id、URL、localStorage key、表情编码字典和用户生成内容不纳入翻译。

执行规则：

- 每批必须先限定目录或文件组，先扫描，再修改；禁止跨后台全目录盲目替换。
- Vue/后台页面用户可见文案统一使用 `lc()`；注释改英文，不新增语言包 key。
- 新增 key 必须同时补齐 `data/lang/auto/zh_cn.php` 和 `data/lang/auto/en_us.php`。
- 每批完成后运行 `php tools/php_lint_gate.php`、`php tools/scan_i18n_status.php`，并对目标后台文件运行 `php tools/scan_vue_remaining.php <file>` 或等价扫描。
- 涉及 JS 或模板内脚本时，必须检查 `http://dev.test/admin/?lang=en_us` 返回 200，且不能出现 `Invalid or unexpected token`。
- 每批单独 commit 并 push 到 `origin/dev`；门禁失败时先修复失败点，不继续扩大范围。

批次顺序：

1. 高风险修复：先修复后台模板/脚本中损坏的引号、括号、数组访问、模板表达式，保证迁移基线可运行。
2. 后台壳页与共享组件：处理 `app/template/admin/index.htm`、共享分类组件、`api.js`、`router.js` 的可见文案与中文注释。
3. 高频业务模块：优先用户、企业、简历、职位模块，每批只处理一个目录或 3-5 个强相关文件。
4. 内容、系统、运营、工具模块：按后台菜单模块推进表格列、按钮、placeholder、弹窗、空状态、校验提示。
5. 语言包质量收敛：修复 `en_us` 中中文残留、与 `zh_cn` 相同的错误译文、误存的模板/Vue/JS 表达式。
6. 扫描报告收敛：更新扫描报告，并标注剩余中文属于协议数据、第三方库、表情编码或用户内容。

Batch 2 执行范围：后台壳页 `app/template/admin/index.htm`、共享组件 `app/template/admin/component/*_class.vue` / `admin_add.vue` / `audioyy.vue`、以及 `app/template/admin/js/api.js`、`app/template/admin/js/router.js`。本批优先处理中文注释英文化和可见中文复核；已 `lc()` 的文案不重复迁移。

Batch 3 执行范围：职位相关高频文件 `app/template/admin/user/company/component/company_job.vue`、`joball.vue`、`partjob.vue`。本批先处理中文注释英文化和裸默认文案复核，保持 API 字段、状态值、路由和业务逻辑不变。

Batch 4 执行范围：用户/企业核心大文件 `app/template/admin/user/users/component/usersall.vue`、`app/template/admin/user/company/component/companyuser.vue`。扫描器未发现可见裸中文时，本批只清理中文注释和注释掉的旧模板片段；发现可见文案时必须进入语言包或复用已有 key。

Batch 5 执行范围：简历核心文件 `app/template/admin/user/users/component/resumeall.vue`、`app/template/admin/user/users/component/resume_edit.vue`。扫描器未发现可见裸中文时，本批只清理中文注释和注释掉的旧模板片段；不改简历字段、审核状态、接口参数和表单结构。

Batch 6 执行范围：企业设置文件 `app/template/admin/user/company/component/comset_index.vue`。扫描器未发现可见裸中文时，本批只清理中文注释；不改企业设置字段、开关值、上传字段、接口参数和表单结构。

Batch 7 执行范围：认证/审核小组件 `app/template/admin/user/users/component/renzheng_show.vue`、`renzheng_logo.vue`、`renzheng_renzheng.vue`、`app/template/admin/user/company/component/companyrz_renzheng.vue`。扫描器未发现可见裸中文时，本批只清理中文注释和注释掉的旧状态片段；不改审核状态值、上传字段、接口参数和批量选择逻辑。

Batch 8 执行范围：新增/编辑表单组件 `app/template/admin/user/company/component/addjob.vue`、`company_add.vue`、`addhotjob.vue`。扫描器未发现可见裸中文时，本批只清理中文注释；不改职位字段、企业字段、地图加载、编辑器配置和表单校验逻辑。

Batch 9 执行范围：简历委托/推荐文件 `app/template/admin/user/users/component/resume_trust.vue`、`resume_trust_recom.vue`。扫描器未发现可见裸中文时，本批只清理中文注释；不改委托状态、推荐发送、进度计算和列表筛选逻辑。

Batch 10 执行范围：企业认证图片审核组件 `app/template/admin/user/company/component/companyrz_pic_show.vue`、`companyrz_pic_logo.vue`、`companyrz_pic_banner.vue`。扫描器未发现可见裸中文时，本批只清理中文注释和注释掉的旧状态片段；不改审核状态值、图片上传字段、接口参数和批量选择逻辑。

## 11. 推荐执行顺序

1. 修复语言包损坏项和扫描脚本误报规则。
2. 建立第三方资源排除清单。
3. 先 WAP 用户主链路，再后台管理主链路。
4. 再处理独立 JS 和 CSS 视觉适配。
5. 最后处理 SQL 安装数据和新增语言扩展。
6. 每个阶段保存 baseline，避免长期迁移中无法判断回归。

