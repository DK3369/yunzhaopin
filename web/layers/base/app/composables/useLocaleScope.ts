import { SITE_LOCALE_KEY, type WebLocale } from '../utils/locale'

/**
 * Which locale storage bucket the running app owns. Site keeps `lang`, admin
 * sets `localeCookieKey` to `admin_lang` so the two never overwrite each other.
 */
export function useLocaleScope(): { key: string; fallback: WebLocale } {
  const pub = useRuntimeConfig().public as { localeCookieKey?: string; localeFallback?: string }
  return {
    key: pub.localeCookieKey || SITE_LOCALE_KEY,
    fallback: pub.localeFallback === 'en' ? 'en' : 'zh',
  }
}
