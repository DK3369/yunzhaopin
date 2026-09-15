import { rustLangFor, rustLangFromAcceptLanguage, SITE_LOCALE_KEY } from '../../app/utils/locale'

/**
 * 转给 Rust 的 Accept-Language 只有 `zh-CN` | `en`。
 * 单条入站 AL（useApi 已按 cookie 写好）优先；浏览器列表丢掉；否则 cookie；缺省 `en`。
 * 不读 `?lang=`。
 */
export function rustLangHeaders(event: Parameters<typeof getCookie>[0]): Record<string, string> {
  const pub = useRuntimeConfig(event).public as { localeCookieKey?: string }
  const key = pub.localeCookieKey || SITE_LOCALE_KEY
  const fromAl = rustLangFromAcceptLanguage(getHeader(event, 'accept-language'))
  return {
    'accept-language': fromAl ?? rustLangFor(getCookie(event, key)),
  }
}
