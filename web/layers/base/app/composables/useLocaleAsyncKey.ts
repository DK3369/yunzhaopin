/**
 * useAsyncData key that follows Vue locale.
 * Call only in setup: `useAsyncData(localeAsyncKey('site-nav'), ...)`.
 */
export function localeAsyncKey(base: string, extra?: string | number) {
  const { locale } = useI18n()
  return () => (extra == null || extra === '' ? `${base}-${locale.value}` : `${base}-${locale.value}-${extra}`)
}
