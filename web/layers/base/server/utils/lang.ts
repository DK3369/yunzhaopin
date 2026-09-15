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
 * Unique sources: this app's locale cookie (`lang` / `admin_lang`), then
 * `Accept-Language` (site only). Do not read `?lang=` — pages and `/api/proxy`
 * must not carry language in the URL. Admin skips the browser sniff, matching
 * PHP `global.php` cookie then English.
 */
export function rustLangHeaders(event: Parameters<typeof getCookie>[0]): Record<string, string> {
  const pub = useRuntimeConfig(event).public as { localeCookieKey?: string; localeFallback?: string }
  const key = pub.localeCookieKey || 'lang'
  const isAdmin = key !== 'lang'
  const fallback = pub.localeFallback === 'en' ? 'en' : 'zh-CN'

  const cookie = getCookie(event, key)
  const header = isAdmin ? '' : getHeader(event, 'accept-language')
  const tag = toRustLang(String(cookie || header || fallback))
  return {
    'accept-language': tag,
  }
}
