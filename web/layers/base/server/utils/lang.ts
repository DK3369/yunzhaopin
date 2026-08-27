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

export function rustLangHeaders(event: Parameters<typeof getCookie>[0]): Record<string, string> {
  const q = getQuery(event).lang
  const cookie = getCookie(event, 'admin_lang') || getCookie(event, 'lang')
  const header = getHeader(event, 'accept-language')
  // Web always sends an explicit tag (default zh-CN). Rust default En is unchanged for App.
  const tag = toRustLang(String(q || cookie || header || 'zh-CN'))
  return {
    'accept-language': tag,
  }
}
