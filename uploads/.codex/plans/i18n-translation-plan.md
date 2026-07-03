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

Batch 11 执行范围：设置/日志/刷新小组件 `app/template/admin/user/users/component/userset_index.vue`、`userlog.vue`、`app/template/admin/user/company/component/comlog_index.vue`、`refresh.vue`。扫描器未发现可见裸中文时，本批只清理中文注释和注释掉的旧状态片段；不改筛选字段、日期范围、日志删除、刷新关闭和批量操作逻辑。

Batch 12 执行范围：会员记录/申诉与积分优惠小组件 `app/template/admin/user/member/component/logoff.vue`、`ltlogin.vue`、`qylogin.vue`、`shensu.vue`、`app/template/admin/system/set/component/jifenyouhui.vue`。本批只清理中文注释和调试日志文案；不改分页、搜索、短信发送、账户详情和优惠配置逻辑。

Batch 13 执行范围：后台内容/系统旧页面 `app/template/admin/neirong/news/newslb.html`、`app/template/admin/system/category/city.html`、`app/template/admin/neirong/gz/gzmanage.html`、`app/template/admin/neirong/announcement/index.html`、`app/template/admin/system/role/component/groupadd.vue`、`app/template/admin/system/domain/component/adminGroup.vue`。本批只清理中文注释和注释掉的旧模板片段；不改分类树、导航设置、公告列表、分站切换、菜单权限提交和编辑器配置逻辑。

Batch 14 执行范围：后台自有 JS `app/template/admin/js/xjhlive.js`、`base64ToFile.js`。本批只清理中文注释和注释掉的旧调试文案；`faces()` 表情名称字典属于协议/表情编码映射，按排除规则保留不改。

Batch 15 执行范围：财务/广告/问答模块 `app/template/admin/yunying/caiwu/component/comhytc.vue`、`comhyzzb.vue`、`app/template/admin/yunying/ad/component/ad_edit.vue`、`app/template/admin/neirong/question/index.html`、`class.html`。扫描器未发现运行态裸中文时，本批只清理中文注释和注释掉的旧按钮/状态片段；不改套餐搜索、广告字段、问答审核、回答/评论列表和删除逻辑。

Batch 16 执行范围：严格裸中文扫描命中的调试日志 `app/template/admin/yunying/special/component/special_view_audit.vue`、`app/template/admin/yunying/caiwu/xiaofei.html`、`chongzhidd.html`、`app/template/admin/tool/database/dataRecycle.html`、`app/template/admin/system/domain/domainAdminList.html`、`app/template/admin/system/domain/component/domainAdminGroup.vue`。本批只将分页 `console.log` 调试文案英文化；不改分页状态、接口参数、表格刷新和域名管理员权限逻辑。

Batch 16 后严格裸中文扫描剩余项：`app/template/admin/js/xjhlive.js:200` 为聊天表情名称映射，按表情编码字典排除；`app/template/admin/login.htm:70` 为 `$config.code_web` 中 `后台登录` 的配置匹配值，不是展示文案；`app/template/admin/tool/weixin/addpubtemp.html:438` 为微信模板内容解析 `样式=` 标记，需确认上游模板协议后再决定是否改为 key/常量。

Batch 17 执行范围：验证码配置匹配兼容 `app/include/i18n.functions.php`、`app/model/notice.model.php`、`app/template/admin/login.htm`。新增 i18n 候选匹配 helper，用语言 key 同时匹配当前语言、`zh_cn`、`en_us` 和 key 本身；后台登录页和 `jycheck()` 改用 helper，避免默认英文环境下 `code_web` 保存为英文后验证码判断失效，同时兼容旧中文配置。

Batch 18 执行范围：后台 JSON 返回运行时 key 解析 `app/include/i18n.class.php`。补齐 `yun_json_encode()` -> `yun_auto_array()` -> `autoT()` 对 `msg/message/error` 等输出字段中纯自动 key（如 `admin_01452`）和带分隔符的自动 key 片段（如 `admin_01452, admin_01453`）的翻译能力；不处理 `admin_01350` 与 ID 直接粘连成 `admin_01350123` 的错误拼接，该类必须后续逐文件改为 `yun_t()`/占位符或明确分隔符。

Batch 19 执行范围：后台模型返回文案显式 key 化 `admin/model/yunying/yingxiao_hrlog.class.php`、`admin/model/tool/dataOss.class.php`、`admin/model/tool/dataCall.class.php`、`admin/model/user/company_order.class.php`、`admin/model/yunying/finance_company_order.class.php`。将年度报告、OSS 配置、数据调用、订单合同图片增删等含 ID 的中文拼接改为 `yun_t()` + `{id}` 占位符，并同步补齐 `data/lang/auto/zh_cn.php` 与 `data/lang/auto/en_us.php` 的 `admin_model_00001` 起始 key。

Batch 20 执行范围：后台壳页 tab/菜单标题 `app/template/admin/index.htm`、已知 `window.parent.homeapp.checkMenuTwo()` 调用点、后台菜单翻译 helper `app/model/navigation.model.php`、`app/include/i18n.class.php`、`app/include/i18n.functions.php`。修复顶部小标签页仍显示中文的问题：菜单接口返回时必须保留稳定 `name_key`，中文菜单名如果存在 `data/lang/auto/aliases.php` 映射则转换为对应 auto key；前端 tabList 只能优先保存/使用 key，显示时再通过 `lc()` 翻译。旧 `localStorage.tabList` 中已缓存中文标题时，在非 `zh_cn` 环境下清理 tab 缓存，避免继续显示历史中文。禁止把 `localStorage` key 本身改名。

Batch 21 执行范围：问答与投诉后台模型 `admin/model/neirong/question.class.php`、`admin/model/yunying/report_job.class.php`、`report_ask.class.php`、`report_advise.class.php`、`report_xjh.class.php`、`report_resume.class.php`。将 `admin_json()`、`render_json()`、`addAdminLog()`、直接 `echo` 和问答审核系统通知中的中文拼接改为 `yun_t()` + 占位符；同步补齐 `admin_model_00007` 起始的中英文语言包 key。仅处理可见/日志文案和注释，不改投诉处理、返还积分/金额/简历数、审核状态、接口字段和枚举值。

Batch 21.1 修正范围：复核 Batch 19-21 新增的 `yun_t()` 占位符调用。`Yun_I18n::replaceParams()` 会在参数 key 外自动拼 `{}`，因此调用必须传 `array('id' => $id)`，不能传 `array('{id}' => $id)`；语言包 value 仍保留 `{id}`、`{ids}` 等占位符。本批只修正参数 key，不改语言包 key、文案、业务逻辑或接口结构。

Batch 22 执行范围：招聘会、招聘会场地、招聘专题、广告和广告分类模型 `admin/model/neirong/zhaopinhui.class.php`、`zph_space.class.php`、`admin/model/yunying/special_special.class.php`、`ad.class.php`、`ad_class.class.php`。将 `admin_json()`、`render_json()`、`addAdminLog()`、系统通知和少量运行态 `<option>` 中文改为 `yun_t()`/稳定 key，占位符参数使用不带 `{}` 的 key；同步补齐 `admin_model_00025` 起始中英文语言包。触碰到的中文注释改为英文。本批不改广告类型、审核枚举、搜索条件、导出字段结构、第三方 Excel 库调用、数据库字段或业务状态值。

Batch 23 执行范围：系统设置、导航、关键词、SEO 相关后台模型 `admin/model/system/set_config.class.php`、`set_regset.class.php`、`set_web_config.class.php`、`set_guanjianci.class.php`、`set_navigation.class.php`、`admin_nav.class.php`、`set_navmap.class.php`、`set_seo.class.php`。将后台返回消息、日志消息和已命中的运行态中文改为 `yun_t()` 占位符；同步补齐 `admin_model_00072` 起始中英文语言包。触碰到的中文注释改为英文。本批不改配置字段名、缓存文件名、菜单 URL、导航类型、SEO 模型标识、审核枚举、验证码厂商参数或上传安全规则。

Batch 24 执行范围：高频用户/企业/职位/简历后台模型 `admin/model/user/company.class.php`、`users_member.class.php`、`company_job.class.php`、`users_resume.class.php`。本批只迁移管理员实际可见的 `admin_json()`、`render_json()`、`addAdminLog()` 和相关 `$msg/$content` 运行态中文，新增 key 从 `admin_model_00106` 继续；大文件中的历史中文注释、导出字段、省市县展示值、搜索枚举和业务字典不在本批统一清理。禁止改动账户认证、分站分配、套餐绑定、职位审核、简历保存和导出业务逻辑。

Batch 25 执行范围：剩余用户相关小模型 `admin/model/user/admin_appeal.class.php`、`admin_member.class.php`、`admin_loginlog.class.php`、`company_cert.class.php`、`company_comrating.class.php`、`company_news.class.php`、`company_product.class.php`、`company_pic.class.php`、`hotjob.class.php`、`partjob.class.php`、`weipin_once.class.php`、`weipin_tiny.class.php`。迁移管理员可见的返回消息、日志消息和触碰处注释，新增 key 从 `admin_model_00141` 继续。本批不改图片/文件处理、审核状态、刷新记录、分站分配、兼职/微招聘/名企删除和企业内容审核业务逻辑。

Batch 26 执行范围：内容模块新闻与测评 `admin/model/neirong/news.class.php`、`evaluate.class.php`。迁移管理员可见的新闻分站、属性设置、删除、类别 ajax、导航取消，以及测评试题日志、试卷/问题/类别删除等运行态中文；新增 key 从 `admin_model_00175` 继续。本批不改新闻静态页生成、新闻内容字段、分类树结构、测评分数计算、上传处理、模型返回结构或业务状态值。

Batch 27 执行范围：剩余内容小模块 `admin/model/neirong/announcement.class.php`、`gongzhao.class.php`、`question_class.class.php`、`toolbox_class.class.php`、`toolbox_doc.class.php`。迁移公告、公招、问答分类、工具箱分类/文档的新增、修改、删除、分站、显示状态等管理员可见返回消息；新增 key 从 `admin_model_00187` 继续。本批不改上传处理、权限弹窗、缓存生成、分类删除级联、工具箱文档内容或模型返回结构。

Batch 28 执行范围：系统设置小模块 `admin/model/system/set_integral.class.php`、`set_payset.class.php`、`singlepage.class.php`、`set_module.class.php`。迁移积分优惠删除/编辑日志、银行转账新增修改删除、单页面保存结果、SEO 保存结果等管理员可见消息；新增 key 从 `admin_model_00206` 继续。本批不改支付配置字段、银行账号字段、单页面 URL 校验/静态生成、SEO 配置结构、缓存生成或第三方支付参数。

Batch 29 执行范围：积分商城后台模块 `admin/model/yunying/shop_class.class.php`、`shop_reward.class.php`、`shop_list.class.php`、`shop_set.class.php`。迁移商品类别日志、商品状态/推荐/热门日志、兑换审核系统通知、商城分类下拉默认项和搜索过滤显示值；新增 key 从 `admin_model_00219` 继续。本批不改库存回退、积分返还、商品/分类 CRUD、审核状态值、兑换记录结构或缓存生成逻辑。

Batch 30 执行范围：分站后台模块 `admin/model/system/domain_list.class.php`、`domain_group.class.php` 及 `app/template/admin/system/domain/component/domainAdminGroup.vue`。迁移分站新增/更新、管理员自改后重新登录提示等管理员可见返回消息，并清理模板中残留的中文注释按钮文本；新增 key 从 `admin_model_00229` 继续。本批不改分站 URL/目录校验、管理员权限、分站缓存、删除逻辑或表单字段结构。

Batch 31 执行范围：后台工具与海报相关消息 `admin/model/tool/weixinrecord.class.php`、`weixinmenu.class.php`、`emailset.class.php`、`fabutool.class.php`、`admin/model/yunying/yingxiao_hbconfig.class.php`。迁移微信用户解绑、微信菜单增删改日志、自动回复关键词删除、邮件测试内容/删除、微信发布模板删除、海报删除等管理员可见消息；新增 key 从 `admin_model_00232` 继续。本批不改微信接口调用、素材上传、邮件发送配置、模板解析、海报保存或第三方返回信息。

Batch 32 执行范围：剩余后台工具/系统小消息 `admin/model/tool/messagelog.class.php`、`emaillog.class.php`、`generate_page.class.php`、`generate_cache.class.php`、`gsdConfig.class.php`、`admin/model/system/set_cron.class.php`、`admin/model/yunying/finance_company_pay.class.php`。迁移短信/邮件记录删除、页面/缓存生成、计划任务执行与时间标签、IP 归属地配置、后台支付金额展示等管理员可见消息；新增 key 从 `admin_model_00243` 继续。本批不改生成流程、缓存项选择、计划任务执行、IP 配置字段、订单金额原始值或支付记录结构。

Batch 33 执行范围：公共职位/城市分类选择 JS `js/public_class.js`、`js/newclass.public.js`。迁移选择弹窗标题、搜索 placeholder、已选提示、确定/取消按钮、无结果提示和最多选择提示，新增 key 使用 `public_js_00001` 起始；触碰到的中文注释改为英文。本批不改分类数据结构、DOM class/id、事件绑定、搜索算法、选择数量限制、layer 参数或回填字段逻辑。`js/member_public.js` 文案较多，单独留到后续批次。

Batch 34 执行范围：会员中心公共 JS `js/member_public.js`。迁移 layer loading、msg、alert、confirm、弹窗 title、动态 HTML/option 文案、签到/消息/订单/职位上下架/金额转换提示，新增 key 使用 `member_js_00001` 起始；触碰到的中文注释改为英文。本批不改 Ajax URL、表单字段、localStorage key、CSS class/id、正则、验证码流程、套餐/积分计算、职位发布条件判断、订单取消逻辑或服务端返回结构。

Batch 35 执行范围：前台公共 JS `js/public.js`。迁移邀请面试提示、收藏状态、收藏确认按钮、设为首页浏览器提示、找回密码错误、举报职位弹窗、展开/收起、验证码消息比较、举报理由默认值、城市选择默认值和时间连接符等运行态文案，新增 key 使用 `front_js_00001` 起始；触碰到的中文注释改为英文。本批不改 Ajax URL、表单字段、DOM class/id、localStorage key、接口返回结构、收藏/邀请/举报业务逻辑、深拷贝工具或日期格式函数。

Batch 36 执行范围：前台职位/城市/行业旧选择器 `js/class.public.js`。迁移搜索器提示、全选标签、移除按钮、最多选择提示、必须选择具体类别提示、目标元素缺失提示、职位/城市/行业弹窗标题、关闭/确定/取消按钮和已选择提示，新增 key 使用 `class_js_00001` 起始；触碰到的中文注释改为英文。本批不改分类数组 `jn/cn/hyname`、DOM class/id、checkbox 类型、选择层级计算、全选/父子级联、回填字段逻辑或回调函数。

Batch 37 执行范围：前台日期选择器 `js/date.js`。迁移上一段/下一段 title、年份范围单位、单个年月日单位、月份/日期标题，新增 key 使用 `date_js_00001` 起始；保持点击后取值仍按原逻辑写入两位月/日和四位年。本批不改插件方法名、DOM class/id、日期计算、闰年判断、下一步脚本注入或表单回填逻辑。

Batch 38 执行范围：前台列表搜索展开 JS `js/search.js`。迁移更多/收起按钮和保存搜索器空条件提示，新增 key 使用 `search_js_00001` 起始；切换判断需兼容旧中文文案和新英文文案。触碰到的中文注释改为英文。本批不改搜索 URL、筛选参数、DOM class/id、hover 行为、选中项删除或跳转逻辑。

Batch 39 执行范围：省市县联动下拉 JS `js/city.js`。迁移默认 option “请选择”，新增 key 使用 `city_js_00001` 起始；触碰到的中文注释改为英文。本批不改 layui form 事件、ct/cn 分类数据、select name/id、三级联动显示逻辑或 form.render 调用。

Batch 40 执行范围：旧弹窗窗口 JS `js/isven_window/isven_window.js`。迁移关闭按钮 title，新增 key 使用 `isven_js_00001` 起始；触碰到的中文注释改为英文。本批不改弹窗 DOM 结构、iframe、遮罩层、拖拽坐标、滚动锁定或关闭函数名。

Batch 41 执行范围：地图交互 JS `js/map.js`。迁移地图弹窗标题和路线起点校验提示，新增 key 使用 `map_js_00001` 起始；触碰到的中文注释和 console 调试文案改为英文。本批不改 AMap API、坐标字段、DOM id、公交/驾车策略枚举、异步 JSONP 配置、地图中心点或路线搜索逻辑。

Batch 42 执行范围：会员绑定与认证 JS `js/binding.js`。迁移手机/邮箱绑定、验证码、企业认证、个人认证相关 layer 提示、loading 文案、倒计时文案、重新绑定按钮和弹窗标题，新增 key 使用 `binding_js_00001` 起始；触碰到的中文注释改为英文。本批不改 Ajax URL、表单字段、验证码刷新、手机号/邮箱/身份证校验、上传字段、提交表单、layer 参数或认证业务分支。

Batch 43 执行范围：WebUploader 上传组件 `js/webuploader/upload.js`。迁移 Flash 安装提示、浏览器不支持、选择/继续添加图片、删除/旋转按钮、上传错误、预览状态、统计状态、暂停/继续/开始上传、上传成功和 loading 文案，新增 key 使用 `webuploader_js_00001` 起始；触碰到的中文注释改为英文。本批不改 `pedding/ready/uploading/paused/confirm/finish/done` 状态值、`file.statusText`、`stats.successNum/uploadFailNum`、`fileCount/fileSize`、`percentages`、`retry/ignore/cancel/rotateRight/rotateLeft` class、WebUploader 配置字段、上传 URL、`formData`、文件数量/大小限制、进度百分比、跳转 `returnUrl` 或上传成功后的 reload/location 行为。状态统计 HTML 结构继续在 JS 内拼接，语言包 value 只保存纯文本和 `{count}`、`{size}`、`{success}`、`{fail}` 占位符。

Batch 44 执行范围：WebUploader 裁剪上传组件 `js/webuploader/uploader.js`。迁移浏览器不支持、执行中、上传成功、上传失败、不能预览、预览出错等可见文案，新增 key 使用 `webuploader_js_00028` 起始，并复用 `webuploader_js_00004`、`webuploader_js_00015`、`webuploader_js_00026`；触碰到的中文注释改为英文。本批不改 `file._cropData`、`file.source`、`file.size`、`FRAME_WIDTH`、`cropImage` hook、`before-send-file`、`selectCb(src)`、`serverPath/tplPath`、`uploadSuccess` 返回值判断、裁剪尺寸、坐标、状态码或上传返回结构。

Batch 45 执行范围：Layui formSelects 多选组件 `js/layui/formSelects-v4.js`。迁移默认提示、快捷按钮、远程搜索状态、无匹配项、服务异常、搜索 placeholder 和空选项等运行态文案，新增 key 使用 `form_selects_js_00001` 起始；触碰到的中文注释改为英文。本批不改 `xm-select` 配置结构、DOM class/id、快捷按钮数组形状、事件回调、ajax 请求/响应字段、`value/name/selected/disabled/children` 映射、job/city 搜索数据结构、联动/单选/树形模式或返回数据判断。

Batch 46 执行范围：自有上传封装、前端 i18n helper 和 layer 兼容封装 `js/layui.upload.js`、`js/yun-i18n.js`、`js/layui/phpyun_layer.js`。本批只清理中文注释和注释示例，不新增语言包 key；不改上传 URL、上传字段、预览 DOM 结构、`res.data.url/picurl` 返回读取、layer 兼容方法签名、默认参数、遮罩配置、`yunT/yunAt/yunAutoT/yunLc` 对外 API、语言包加载流程或自动 key 匹配规则。

Batch 47 执行范围：验证码接入 wrapper 与灰度工具注释 `js/geetest/pc.js`、`js/geetest/mobile.js`、`js/tecent/pc.js`、`js/dingxiang/pc.js`、`js/vaptcha/pc.js`、`js/grayscale.js`。本批只清理中文注释，不新增语言包 key；不改验证码厂商 SDK 参数、appId/vid/scene/lang/token 字段、提交/模拟点击流程、隐藏字段名、接口 URL、错误票据生成、回调函数名或灰度算法。

Batch 48 执行范围：可控 CSS 注释和字体别名 `js/layui/css/formSelects-v4.css`、`js/imgareaselect/imgareaselect.css`、`js/article/css/style.css`。本批只清理中文注释，并将 `微软雅黑/宋体` 替换为等价英文 CSS 字体族名 `Microsoft YaHei/SimSun`；不新增语言包 key，不改 selector、class/id、尺寸、颜色、URL、图标字体、裁剪样式、文章轮播布局或任何 JS 数据结构。

## 11. 推荐执行顺序

1. 修复语言包损坏项和扫描脚本误报规则。
2. 建立第三方资源排除清单。
3. 先 WAP 用户主链路，再后台管理主链路。
4. 再处理独立 JS 和 CSS 视觉适配。
5. 最后处理 SQL 安装数据和新增语言扩展。
6. 每个阶段保存 baseline，避免长期迁移中无法判断回归。
