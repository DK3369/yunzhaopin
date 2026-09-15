import { DEFAULT_WEB_LOCALE, SITE_LOCALE_KEY, type WebLocale } from '../utils/locale'

/**
 * Which locale cookie this app owns. Site: `lang`. Admin: `admin_lang`.
 * Fallback is always English.
 */
export function useLocaleScope(): { key: string; fallback: WebLocale } {
  const pub = useRuntimeConfig().public as { localeCookieKey?: string }
  return {
    key: pub.localeCookieKey || SITE_LOCALE_KEY,
    fallback: DEFAULT_WEB_LOCALE,
  }
}
