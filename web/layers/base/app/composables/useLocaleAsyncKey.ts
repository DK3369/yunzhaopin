/**
 * useAsyncData key stamped with the current Vue locale.
 * Call only in setup. Language switch reloads the page, so this is a string
 * (not a getter): a reactive key would refetch nav/settings when i18n settles.
 */
export function localeAsyncKey(base: string, extra?: string | number) {
  const { locale } = useI18n()
  const loc = String(locale.value || 'en')
  return extra == null || extra === '' ? `${base}-${loc}` : `${base}-${loc}-${extra}`
}

/** Reuse payload / in-flight request; still refetch on explicit `refresh()`. */
export function reuseAsyncCache() {
  return {
    dedupe: 'defer' as const,
    getCachedData(key: string, nuxtApp: { payload: { data: Record<string, unknown> } }, ctx?: { cause?: string }) {
      if (ctx?.cause === 'refresh:manual') return undefined
      return nuxtApp.payload.data[key]
    },
  }
}
