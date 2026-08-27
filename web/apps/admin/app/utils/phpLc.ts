type Pack = Record<string, string>

let messages: Pack = {}
let lcMessages: Pack = {}

function replaceParams(text: string, params: unknown): string {
  if (!params) return text
  const arr = Array.isArray(params) ? params : [params]
  let output = text
  for (let i = 0; i < arr.length; i++) {
    output = output.split(`{${i}}`).join(String(arr[i] ?? ''))
  }
  return output
}

export function lc(key: string, params?: unknown, fallback?: string): string {
  if (!key) return fallback || ''
  const lookupKey = key.indexOf('.') === -1 ? `lc.${key}` : key
  const text = lcMessages[lookupKey] || lcMessages[key] || messages[key] || fallback || key
  return replaceParams(text, params || [])
}

export async function loadLangPack(lang?: string): Promise<void> {
  const raw = (lang || (import.meta.client ? localStorage.getItem('lang') : '') || 'zh_cn').toLowerCase()
  const file = raw.startsWith('en') ? 'en_us' : 'zh_cn'
  try {
    const pack = await $fetch<Pack>(`/admin/php-admin/lang/${file}.json`)
    messages = pack || {}
    lcMessages = pack || {}
  } catch {
    messages = {}
    lcMessages = {}
  }
  if (import.meta.client) {
    window.yunAdminI18n = { lang: file, messages, lc: lcMessages, keys: Object.keys(messages) }
  }
}

declare global {
  interface Window {
    yunAdminI18n?: { lang: string; messages: Pack; lc: Pack; keys: string[] }
    lc?: typeof lc
    httpPost?: typeof import('./httpPost').httpPost
    homeapp?: Record<string, unknown>
    echarts?: { init: (el: unknown) => { setOption: (...a: unknown[]) => void; resize: () => void }; graphic: { LinearGradient: new (...a: unknown[]) => unknown } }
    $?: unknown
  }
}
