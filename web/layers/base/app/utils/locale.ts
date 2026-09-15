/**
 * 全站语言合同：只有 `en` | `zh`。默认 `en`。选了中文才是中文。
 *
 * 前台 cookie `lang`，后台 `admin_lang`，互不污染。不要在页面或
 * `/api/proxy` 带 `?lang=`，不要嗅探浏览器，不要再写 `zh-CN` / `en-US`。
 */

export type WebLocale = 'en' | 'zh'

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

/** Accept-Language / Rust 线上标签：与 cookie 相同，默认 `en`。 */
export function rustLangFor(locale: WebLocale | string | null | undefined): WebLocale {
  return parseWebLocale(locale)
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
  const mappedLs = mapWebLocale(localStorage.getItem(key))
  if (mappedLs) return mappedLs
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
