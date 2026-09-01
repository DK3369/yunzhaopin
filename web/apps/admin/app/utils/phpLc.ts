/**
 * Runtime is Nuxt vue-i18n (`i18n/locales/{zh,en}.json`), which already contains
 * the 16518 numbered keys from the PHP packs. `public/php-admin/lang/*.json` is
 * that same corpus kept on disk — do not fetch it at runtime (that dual-track
 * is why language switch used to do nothing).
 *
 * Menu labels follow PHP `navigation.model.php translateAdminNavRows` +
 * `index.htm translateMenuText`: Chinese name → alias key → lc(key).
 */
import { persistWebLocale, parseWebLocale, readStoredLocale, rustLangFor, type WebLocale } from '../../../../layers/base/app/utils/locale'

export type AdminLocale = WebLocale

export function parseAdminLocale(raw?: string | null): AdminLocale {
  return parseWebLocale(raw)
}

export { persistWebLocale as persistLocale, readStoredLocale, rustLangFor }

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
  },
  en: {
    admin_yunying_00201: 'SMS',
    admin_vue_00137: 'Remove {0}?',
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
  t: (key: string) => unknown
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
  return parseWebLocale(i18n?.locale.value)
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

function messageOf(i18n: I18nComposer, key: string): string | undefined {
  const loc = activeLocale(i18n)
  const pinned = LC_FIRST_WINS[loc]?.[key]
  if (pinned) return pinned
  const raw = lookupRaw(i18n, key)
  if (raw != null) return raw
  if (key.indexOf('.') === -1) {
    const nested = lookupRaw(i18n, `lc.${key}`)
    if (nested != null) return nested
  }
  if (i18n.te(key)) return String(i18n.t(key))
  if (key.indexOf('.') === -1 && i18n.te(`lc.${key}`)) return String(i18n.t(`lc.${key}`))
  return undefined
}

/** PHP last-write collision + nav.123 / nav.186. Safe to call more than once. */
export function applyPhpLcFixes(): void {
  if (mergedFixes) return
  const i18n = composer()
  if (!i18n?.mergeLocaleMessage) return
  i18n.mergeLocaleMessage('zh', {
    admin_yunying_00201: LC_FIRST_WINS.zh.admin_yunying_00201,
    admin_vue_00137: LC_FIRST_WINS.zh.admin_vue_00137,
    nav: { '123': NAV_LABEL.zh[123], '186': NAV_LABEL.zh[186] },
  })
  i18n.mergeLocaleMessage('en', {
    admin_yunying_00201: LC_FIRST_WINS.en.admin_yunying_00201,
    admin_vue_00137: LC_FIRST_WINS.en.admin_vue_00137,
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
    text = messageOf(i18n, k)
  }
  return replaceParams(text ?? fallback ?? k, params)
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
  const loc = parseWebLocale(locale)
  persistWebLocale(loc)
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
    homeapp?: Record<string, unknown>
    custoapp?: { openSeoshezhi?: (data: unknown) => void; seotabRefresh?: () => void; curTab?: string }
    echarts?: {
      init: (el: unknown) => { setOption: (...a: unknown[]) => void; resize: () => void }
      graphic: { LinearGradient: new (...a: unknown[]) => unknown }
    }
    $?: unknown
  }
}
