/**
 * 全站语言合同：cookie / Vue 只有 `en` | `zh`。默认 `en`。选了中文才是中文。
 *
 * 前台 cookie `lang`，后台 `admin_lang`，互不污染。不要在页面或
 * `/api/proxy` 带 `?lang=`，不要嗅探浏览器列表。发给 Rust 的
 * Accept-Language 是 `en` | `zh-CN`（`zh` → `zh-CN`）。
 */

export type WebLocale = 'en' | 'zh'
export type RustLangTag = 'en' | 'zh-CN'

export const DEFAULT_WEB_LOCALE: WebLocale = 'en'
export const SITE_LOCALE_KEY = 'lang'
export const ADMIN_LOCALE_KEY = 'admin_lang'

const MAX_AGE = 31536000

export function mapWebLocale(raw?: string | null): WebLocale | null {
  const s = String(raw || '')
    .trim()
    .toLowerCase()
    .replace(/_/g, '-')
  if (!s) return null
  if (s.startsWith('en')) return 'en'
  if (s.startsWith('zh') || s === 'cn') return 'zh'
  return null
}

export function parseWebLocale(raw?: string | null, fallback: WebLocale = DEFAULT_WEB_LOCALE): WebLocale {
  return mapWebLocale(raw) ?? fallback
}

/** Rust / Accept-Language：`zh-CN` | `en`，默认 `en`。cookie 仍是 `zh` | `en`。 */
export function rustLangFor(locale: WebLocale | string | null | undefined): RustLangTag {
  return parseWebLocale(locale) === 'zh' ? 'zh-CN' : 'en'
}

/**
 * 单条 Accept-Language（`en` / `zh` / `zh-CN`）。带逗号的浏览器列表返回 null，
 * 不要当站点语言用。
 */
export function rustLangFromAcceptLanguage(raw?: string | null): RustLangTag | null {
  const s = String(raw || '').trim()
  if (!s || s.includes(',')) return null
  const tag = s.split(';')[0]?.trim() || ''
  const mapped = mapWebLocale(tag)
  if (!mapped) return null
  return mapped === 'zh' ? 'zh-CN' : 'en'
}

function readCookie(name: string): string {
  if (!import.meta.client) return ''
  const hit = document.cookie
    .split(';')
    .map((x) => x.trim())
    .find((x) => x.startsWith(`${name}=`))
  if (!hit) return ''
  try {
    return decodeURIComponent(hit.slice(name.length + 1))
  } catch {
    return hit.slice(name.length + 1)
  }
}

export function readStoredLocale(key: string = SITE_LOCALE_KEY, fallback: WebLocale = DEFAULT_WEB_LOCALE): WebLocale {
  if (!import.meta.client) return fallback
  const mappedCookie = mapWebLocale(readCookie(key))
  if (mappedCookie) return mappedCookie
  return fallback
}

export function persistWebLocale(locale: WebLocale, key: string = SITE_LOCALE_KEY) {
  if (!import.meta.client) return
  const tag = parseWebLocale(locale)
  localStorage.setItem(key, tag)
  const secure = location.protocol === 'https:' ? '; Secure' : ''
  document.cookie = `${key}=${tag}; max-age=${MAX_AGE}; path=/; SameSite=Lax${secure}`
  document.cookie = `i18n_redirected=; max-age=0; path=/; SameSite=Lax${secure}`
}
