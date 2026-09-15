/** Map PHP / Nuxt / browser tags to Rust `Lang::parse_tag` values. Default is `en`. */

export function toRustLang(raw?: string | null): string {
  const s = String(raw || '')
    .trim()
    .toLowerCase()
    .split(',')[0]
    ?.split(';')[0]
    ?.trim()
    .replace(/_/g, '-') || ''
  if (!s) return 'en'
  if (s.startsWith('en')) return 'en'
  if (s.startsWith('zh') || s === 'cn') return 'zh'
  return 'en'
}

/**
 * Resolve the upstream language for whichever app owns this Nitro instance.
 *
 * Unique sources: this app's locale cookie (`lang` / `admin_lang`), then
 * `Accept-Language` (site only). Tags are `en` / `zh` (default `en`). Do not
 * read `?lang=`. Admin skips the browser sniff: cookie then English.
 */
export function rustLangHeaders(event: Parameters<typeof getCookie>[0]): Record<string, string> {
  const pub = useRuntimeConfig(event).public as { localeCookieKey?: string; localeFallback?: string }
  const key = pub.localeCookieKey || 'lang'
  const isAdmin = key !== 'lang'
  const fallback = pub.localeFallback === 'zh' ? 'zh' : 'en'

  const cookie = getCookie(event, key)
  const header = isAdmin ? '' : getHeader(event, 'accept-language')
  const tag = toRustLang(String(cookie || header || fallback))
  return {
    'accept-language': tag,
  }
}
