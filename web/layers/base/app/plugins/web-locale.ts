/**
 * Apply the locale cookie (or default `en`) before pages fetch.
 * Site uses `lang`, admin uses `admin_lang`. No browser sniff.
 */
import { parseWebLocale } from '../utils/locale'

export default defineNuxtPlugin({
  name: 'web-locale',
  async setup(nuxtApp) {
    const i18n = nuxtApp.$i18n as { locale?: { value?: string }; setLocale?: (c: string) => Promise<void> } | undefined
    if (!i18n?.setLocale) return
    const scope = useLocaleScope()
    const cookie = useCookie(scope.key)
    const stored = parseWebLocale(cookie.value)
    const current = String(i18n.locale?.value || '')
    if (current !== stored) {
      await i18n.setLocale(stored)
    }
  },
})
