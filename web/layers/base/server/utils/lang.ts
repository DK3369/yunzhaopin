/** Map PHP / Nuxt / browser tags to Rust `Lang::parse_tag` values. Web default is zh-CN. */

export function toRustLang(raw?: string | null): string {
  const s = String(raw || '')
    .trim()
    .toLowerCase()
    .split(',')[0]
    ?.split(';')[0]
    ?.trim()
    .replace(/_/g, '-') || ''
  if (!s) return 'zh-CN'
  if (s.startsWith('en')) return 'en'
  if (s.includes('tw') || s.includes('hk') || s.includes('mo') || s.includes('hant')) return 'zh-TW'
  if (s.startsWith('zh') || s === 'cn') return 'zh-CN'
  return 'zh-CN'
}

/**
 * Resolve the upstream language for whichever app owns this Nitro instance.
 *
 * `localeCookieKey` is `lang` for the site and `admin_lang` for admin, so the
 * console never inherits what a job seeker picked on PC/H5. Admin also skips
 * the browser `Accept-Language` sniff, matching PHP `global.php`: `?lang=`,
 * then the admin cookie, then English.
 */
export function rustLangHeaders(event: Parameters<typeof getCookie>[0]): Record<string, string> {
  const pub = useRuntimeConfig(event).public as { localeCookieKey?: string; localeFallback?: string }
  const key = pub.localeCookieKey || 'lang'
  const isAdmin = key !== 'lang'
  const fallback = pub.localeFallback === 'en' ? 'en' : 'zh-CN'

  const q = getQuery(event).lang
  const cookie = getCookie(event, key)
  const header = isAdmin ? '' : getHeader(event, 'accept-language')
  const tag = toRustLang(String(q || cookie || header || fallback))
  return {
    'accept-language': tag,
  }
}
