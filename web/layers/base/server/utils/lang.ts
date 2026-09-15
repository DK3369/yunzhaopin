import { parseWebLocale, SITE_LOCALE_KEY } from '../../app/utils/locale'

/**
 * Cookie `lang` / `admin_lang` → one Accept-Language (`en` | `zh`).
 * Missing cookie is `en`. Do not sniff the browser and do not read `?lang=`.
 */
export function rustLangHeaders(event: Parameters<typeof getCookie>[0]): Record<string, string> {
  const pub = useRuntimeConfig(event).public as { localeCookieKey?: string }
  const key = pub.localeCookieKey || SITE_LOCALE_KEY
  return {
    'accept-language': parseWebLocale(getCookie(event, key)),
  }
}
