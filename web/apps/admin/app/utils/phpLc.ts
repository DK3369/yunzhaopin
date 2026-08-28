import { persistWebLocale, parseWebLocale, readStoredLocale, rustLangFor, type WebLocale } from '../../../../layers/base/app/utils/locale'

export type AdminLocale = WebLocale

export function parseAdminLocale(raw?: string | null): AdminLocale {
  return parseWebLocale(raw)
}

export { persistWebLocale as persistLocale, readStoredLocale, rustLangFor }

function replaceParams(text: string, params: unknown): string {
  if (!params) return text
  const arr = Array.isArray(params) ? params : [params]
  let output = text
  for (let i = 0; i < arr.length; i++) {
    output = output.split(`{${i}}`).join(String(arr[i] ?? ''))
  }
  return output
}

type I18nComposer = {
  t: (key: string) => unknown
  te: (key: string) => boolean
  locale: { value: string }
}

function composer(): I18nComposer | null {
  try {
    const i18n = useNuxtApp().$i18n as I18nComposer | undefined
    return i18n || null
  } catch {
    return null
  }
}

export function lc(key: string, params?: unknown, fallback?: string): string {
  const k = String(key || '')
  const i18n = composer()
  let text: string | undefined
  if (i18n && k) {
    void i18n.locale.value
    if (i18n.te(k)) text = String(i18n.t(k))
    else if (k.indexOf('.') === -1 && i18n.te(`lc.${k}`)) text = String(i18n.t(`lc.${k}`))
  }
  return replaceParams(text ?? fallback ?? k, params)
}

export async function setAdminLocale(locale: AdminLocale | string): Promise<AdminLocale> {
  const loc = parseWebLocale(locale)
  persistWebLocale(loc)
  return loc
}

declare global {
  interface Window {
    lc?: typeof lc
    httpPost?: typeof import('./httpPost').httpPost
    homeapp?: Record<string, unknown>
    echarts?: {
      init: (el: unknown) => { setOption: (...a: unknown[]) => void; resize: () => void }
      graphic: { LinearGradient: new (...a: unknown[]) => unknown }
    }
    $?: unknown
  }
}
