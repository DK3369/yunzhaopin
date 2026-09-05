/**
 * Web locale contract: `zh` | `en`.
 *
 * Site and admin are served from the same origin (`:3001`, admin under
 * `/admin`), so they must not share one storage key or switching language in
 * one would switch the other. PHP `global.php` splits them the same way:
 * front uses `lang`, admin uses `admin_lang`.
 */

export type WebLocale = 'zh' | 'en'

export const SITE_LOCALE_KEY = 'lang'
export const ADMIN_LOCALE_KEY = 'admin_lang'

const MAX_AGE = 31536000

export function parseWebLocale(raw?: string | null, fallback: WebLocale = 'zh'): WebLocale {
  return mapWebLocale(raw) ?? fallback
}

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

export function rustLangFor(locale: WebLocale, key: string = SITE_LOCALE_KEY): string {
  if (locale === 'en') return 'en'
  // UI pack is Simplified Chinese only; Traditional browsers still get zh UI
  // strings, but the API can return zh-TW dictionary names.
  if (import.meta.client) {
    try {
      const raw = localStorage.getItem(key) || ''
      if (/tw|hk|mo|hant/i.test(raw)) return 'zh-TW'
    } catch {
      /* ignore */
    }
    if (typeof navigator !== 'undefined' && /tw|hk|mo|hant/i.test(navigator.language)) {
      return 'zh-TW'
    }
  }
  return 'zh-CN'
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

export function readStoredLocale(key: string = SITE_LOCALE_KEY, fallback: WebLocale = 'zh'): WebLocale {
  if (!import.meta.client) return fallback
  const mappedLs = mapWebLocale(localStorage.getItem(key))
  if (mappedLs) return mappedLs
  const mappedCookie = mapWebLocale(readCookie(key))
  if (mappedCookie) return mappedCookie
  return fallback
}

export function persistWebLocale(locale: WebLocale, key: string = SITE_LOCALE_KEY) {
  if (!import.meta.client) return
  localStorage.setItem(key, locale)
  const secure = location.protocol === 'https:' ? '; Secure' : ''
  document.cookie = `${key}=${locale}; max-age=${MAX_AGE}; path=/; SameSite=Lax${secure}`
}
