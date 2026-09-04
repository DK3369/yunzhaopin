/** Web locale contract: `zh` | `en`. Cookie + localStorage key is `lang`. */

export type WebLocale = 'zh' | 'en'

const COOKIE = 'lang'
const MAX_AGE = 31536000

export function parseWebLocale(raw?: string | null): WebLocale {
  const s = String(raw || '')
    .trim()
    .toLowerCase()
    .replace(/_/g, '-')
  if (s.startsWith('en')) return 'en'
  return 'zh'
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

export function rustLangFor(locale: WebLocale): string {
  if (locale === 'en') return 'en'
  // UI pack is Simplified Chinese only; Traditional browsers still get zh UI
  // strings, but the API can return zh-TW dictionary names.
  if (import.meta.client) {
    try {
      const raw = localStorage.getItem(COOKIE) || ''
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

export function readStoredLocale(): WebLocale {
  if (!import.meta.client) return 'zh'
  const ls = localStorage.getItem(COOKIE)
  const mappedLs = mapWebLocale(ls)
  if (mappedLs) return mappedLs
  const cookie = readCookie(COOKIE)
  const mappedCookie = mapWebLocale(cookie)
  if (mappedCookie) return mappedCookie
  return 'zh'
}

export function persistWebLocale(locale: WebLocale) {
  if (!import.meta.client) return
  localStorage.setItem(COOKIE, locale)
  const secure = location.protocol === 'https:' ? '; Secure' : ''
  const val = `${COOKIE}=${locale}; max-age=${MAX_AGE}; path=/; SameSite=Lax${secure}`
  document.cookie = val
}
