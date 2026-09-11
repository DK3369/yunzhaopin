/**
 * Runtime is Nuxt vue-i18n (`i18n/locales/{zh,en}.json`), which already contains
 * the 16518 numbered keys from the PHP packs. `public/php-admin/lang/*.json` is
 * that same corpus kept on disk — do not fetch it at runtime (that dual-track
 * is why language switch used to do nothing).
 *
 * Menu labels follow PHP `navigation.model.php translateAdminNavRows` +
 * `index.htm translateMenuText`: Chinese name → alias key → lc(key).
 */
import {
  ADMIN_LOCALE_KEY,
  persistWebLocale,
  parseWebLocale,
  readStoredLocale as readStoredWebLocale,
  rustLangFor as rustLangForWeb,
  type WebLocale,
} from '../../../../layers/base/app/utils/locale'

export type AdminLocale = WebLocale

/**
 * Admin language lives in `admin_lang`, separate from the front-end `lang`, so
 * switching language in the console never touches what job seekers / companies
 * see on PC or H5. Same split as PHP `global.php`, English default included.
 */
export function parseAdminLocale(raw?: string | null): AdminLocale {
  return parseWebLocale(raw, 'en')
}

export function persistLocale(locale: AdminLocale) {
  persistWebLocale(locale, ADMIN_LOCALE_KEY)
}

export function readStoredLocale(): AdminLocale {
  return readStoredWebLocale(ADMIN_LOCALE_KEY, 'en')
}

export function rustLangFor(locale: AdminLocale): string {
  return rustLangForWeb(locale, ADMIN_LOCALE_KEY)
}

/** PHP `Yun_I18n::isAutoKey` */
const AUTO_KEY_RE = /^([a-z][a-z0-9_]*)_([0-9]{5})$/

/**
 * PHP `zh_cn.php` / `en_us.php` define `admin_yunying_00201` twice: first 「短信」/SMS
 * (index.json + tuiguang radios), later 「确定移除 {0}？」 (chongzhidd). PHP arrays
 * last-write-win, so nav alias `短信` became Remove {0}? and vue-i18n turned `{0}`
 * into `0`. Keep the first meaning here; remove-confirm uses `admin_vue_00137`.
 */
const LC_FIRST_WINS: Record<WebLocale, Record<string, string>> = {
  zh: {
    admin_yunying_00201: '短信',
    admin_vue_00137: '确定移除 {0}？',
    admin_level1_category_value: '一级分类：{0}',
    admin_tool_00689: 'Google登录配置',
    admin_tool_00690: 'Facebook登录配置',
    admin_tool_00691: 'Client ID',
    admin_tool_00692: 'Client Secret',
    admin_tool_00693: '回调地址为前台 /login，须与开放平台后台填写的一致',
    admin_tool_00694: '当前已开启',
    admin_tool_00695: '当前已关闭',
    admin_php_version: '前后端版本：{0}',
    admin_phpyun_version: 'OV6程序版本：{0}',
    admin_available_space: '可用空间(磁盘区)：{0} M',
    admin_server_software: '服务器软件：{0}',
    admin_mysql_version: 'MySQL 版本：{0}',
    admin_user_server: '用户 - 服务器：{0} - {1}',
    admin_tool_00696: '招聘采集',
    admin_tool_00697: '源站地址',
    admin_tool_00698: '自动采集',
    admin_tool_00699: '采集间隔',
    admin_tool_00700: '小时',
    admin_tool_00701: '分钟',
    admin_tool_00702: '立即采集',
    admin_tool_00703: '最近运行',
    admin_tool_00704: '采集日志',
    admin_tool_00705: '只导入英文职位，按来源链接去重后写入职位库',
    admin_tool_00706: '抓取条数',
    admin_tool_00707: '新增',
    admin_tool_00708: '跳过（已存在）',
    admin_tool_00709: '尚未运行',
    admin_tool_00710: '按间隔自动抓取英文招聘信息并添加到职位库。源站需为英文职位列表页。',
  },
  en: {
    admin_yunying_00201: 'SMS',
    admin_vue_00137: 'Remove {0}?',
    admin_level1_category_value: 'Level 1 Category: {0}',
    admin_tool_00689: 'Google Login',
    admin_tool_00690: 'Facebook Login',
    admin_tool_00691: 'Client ID',
    admin_tool_00692: 'Client Secret',
    admin_tool_00693: 'Callback URL is the site /login page and must match the provider console',
    admin_tool_00694: 'Currently on',
    admin_tool_00695: 'Currently off',
    admin_php_version: 'Frontend / backend: {0}',
    admin_phpyun_version: 'OV6 version: {0}',
    admin_available_space: 'Available disk: {0} M',
    admin_server_software: 'Server software: {0}',
    admin_mysql_version: 'MySQL: {0}',
    admin_user_server: 'User - host: {0} - {1}',
    admin_tool_00696: 'Job scrape',
    admin_tool_00697: 'Source URL',
    admin_tool_00698: 'Auto scrape',
    admin_tool_00699: 'Interval',
    admin_tool_00700: 'hours',
    admin_tool_00701: 'minutes',
    admin_tool_00702: 'Scrape now',
    admin_tool_00703: 'Last run',
    admin_tool_00704: 'Scrape log',
    admin_tool_00705: 'English jobs only; existing source URLs are skipped',
    admin_tool_00706: 'Fetched',
    admin_tool_00707: 'Inserted',
    admin_tool_00708: 'Skipped (exists)',
    admin_tool_00709: 'Not run yet',
    admin_tool_00710: 'Fetch English jobs on a timer and insert them into the job database.',
  },
}

/** Tools children id=123 数据 / id=186 短信 — do not keep the colliding pack strings. */
const NAV_LABEL: Record<WebLocale, Record<number, string>> = {
  zh: { 123: '数据', 186: '短信' },
  en: { 123: 'Data', 186: 'SMS' },
}

/** PHP `aliases.php` for these nav names (avoid loading the full 370KB map). */
const NAME_ALIAS: Record<string, string> = {
  数据: 'admin_tool_00213',
  短信: 'admin_yunying_00201',
}

let mergedFixes = false

function isAutoKey(key: string): boolean {
  const m = AUTO_KEY_RE.exec(key)
  if (!m) return false
  return m[1].split('_').length <= 3
}

function aliasKey(text: string): string {
  const hit = NAME_ALIAS[text]
  return hit && isAutoKey(hit) ? hit : ''
}

function replaceParams(text: string, params: unknown): string {
  if (!params) return text
  const arr = Array.isArray(params) ? params : [params]
  let output = text
  for (let i = 0; i < arr.length; i++) {
    output = output.split(`{${i}}`).join(String(arr[i] ?? ''))
  }
  return output
}

type I18nComposer = {
  t: (key: string, values?: Record<string, unknown>) => unknown
  te: (key: string) => boolean
  locale: { value: string }
  messages?: { value?: Record<string, Record<string, unknown>> }
  mergeLocaleMessage?: (locale: string, msg: Record<string, unknown>) => void
}

function composer(): I18nComposer | null {
  try {
    const i18n = useNuxtApp().$i18n as I18nComposer | undefined
    return i18n || null
  } catch {
    return null
  }
}

function activeLocale(i18n?: I18nComposer | null): WebLocale {
  return parseAdminLocale(i18n?.locale.value)
}

function lookupRaw(i18n: I18nComposer, key: string): string | undefined {
  const loc = i18n.locale.value
  const root = i18n.messages?.value?.[loc]
  if (!root || typeof root !== 'object') return undefined
  const top = root[key]
  if (typeof top === 'string') return top
  const nestedLc = root.lc
  if (nestedLc && typeof nestedLc === 'object') {
    const hit = (nestedLc as Record<string, unknown>)[key]
    if (typeof hit === 'string') return hit
  }
  let cur: unknown = root
  for (const part of key.split('.')) {
    if (!cur || typeof cur !== 'object') return undefined
    cur = (cur as Record<string, unknown>)[part]
  }
  return typeof cur === 'string' ? cur : undefined
}

function translatedOf(i18n: I18nComposer, key: string, params?: unknown): string | undefined {
  if (!i18n.te(key)) return undefined
  const list = Array.isArray(params) ? params : params != null ? [params] : undefined
  const t = String(list ? i18n.t(key, list as never) : i18n.t(key))
  if (!t || t === key) return undefined
  return t
}

function messageOf(i18n: I18nComposer, key: string, params?: unknown): string | undefined {
  const loc = activeLocale(i18n)
  const pinned = LC_FIRST_WINS[loc]?.[key]
  if (pinned) return pinned
  const raw = lookupRaw(i18n, key)
  if (typeof raw === 'string' && raw.length > 0) return raw
  if (key.indexOf('.') === -1) {
    const nested = lookupRaw(i18n, `lc.${key}`)
    if (typeof nested === 'string' && nested.length > 0) return nested
  }
  const t = translatedOf(i18n, key, params)
  if (t) return t
  if (key.indexOf('.') === -1) return translatedOf(i18n, `lc.${key}`, params)
  return undefined
}

/** PHP last-write collision + nav.123 / nav.186. Safe to call more than once. */
export function applyPhpLcFixes(): void {
  if (mergedFixes) return
  const i18n = composer()
  if (!i18n?.mergeLocaleMessage) return
  i18n.mergeLocaleMessage('zh', {
    ...LC_FIRST_WINS.zh,
    nav: { '123': NAV_LABEL.zh[123], '186': NAV_LABEL.zh[186] },
  })
  i18n.mergeLocaleMessage('en', {
    ...LC_FIRST_WINS.en,
    nav: { '123': NAV_LABEL.en[123], '186': NAV_LABEL.en[186] },
  })
  mergedFixes = true
}

export function lc(key: string, params?: unknown, fallback?: string): string {
  const k = String(key || '')
  const i18n = composer()
  let text: string | undefined
  if (i18n && k) {
    void i18n.locale.value
    applyPhpLcFixes()
    text = messageOf(i18n, k, params)
  }
  const base = text && text.length > 0 ? text : fallback || k
  return replaceParams(base, params)
}

/**
 * PHP `index.htm translateMenuText` / `translateAdminNavRows`.
 * Prefer per-id nav labels, then numbered key, then Chinese alias → lc().
 */
export function translateMenuText(name: string, navId?: number): string {
  const i18n = composer()
  if (i18n) {
    void i18n.locale.value
    applyPhpLcFixes()
  }
  const loc = activeLocale(i18n)
  if (navId != null && navId > 0) {
    const pinned = NAV_LABEL[loc]?.[navId]
    if (pinned) return pinned
    const navKey = `nav.${navId}`
    if (i18n?.te(navKey)) {
      const raw = lookupRaw(i18n, navKey)
      if (raw) return raw
      return String(i18n.t(navKey))
    }
  }
  const v = String(name || '')
  if (!v) return ''
  if (isAutoKey(v)) return lc(v)
  const alias = aliasKey(v)
  if (alias) return lc(alias)
  if (i18n?.te(v)) {
    const raw = lookupRaw(i18n, v)
    if (raw) return raw
    return String(i18n.t(v))
  }
  return v
}

export async function setAdminLocale(locale: AdminLocale | string): Promise<AdminLocale> {
  const loc = parseAdminLocale(locale)
  persistLocale(loc)
  mergedFixes = false
  applyPhpLcFixes()
  return loc
}

declare global {
  interface Window {
    lc?: typeof lc
    yunAdminT?: (text: unknown) => string
    yunAdminTransText?: (text: unknown) => string
    httpPost?: typeof import('./httpPost').httpPost
    httpGet?: typeof import('./httpPost').httpPost
    homeapp?: Record<string, unknown>
    custoapp?: { openSeoshezhi?: (data: unknown) => void; seotabRefresh?: () => void; curTab?: string }
    echarts?: {
      init: (el: unknown) => { setOption: (...a: unknown[]) => void; resize: () => void }
      graphic: { LinearGradient: new (...a: unknown[]) => unknown }
    }
    $?: unknown
  }
}
