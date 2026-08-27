/**
 * Admin i18n: one locale (`zh` | `en`), one PHP-numbered pack, optional aliases.
 *
 * - Packs: `/admin/php-admin/lang/{zh_cn,en_us}.json`
 * - Aliases: Chinese/literal → `module_00000` (same map as PHP `aliases.php`)
 * - Persist: cookie + localStorage `admin_lang` (`zh`/`en`); legacy `lang=zh_cn|en_us` still read
 * - `lc()` reads reactive state so Vue templates re-render when the pack changes
 */
import { reactive } from 'vue'

export type AdminLocale = 'zh' | 'en'

const COOKIE = 'admin_lang'
const LS = 'admin_lang'
const LS_LEGACY = 'lang'
const AUTO_KEY = /^([a-z][a-z0-9_]*)_([0-9]{5})$/

type Pack = Record<string, string>

export const adminI18n = reactive({
  locale: 'zh' as AdminLocale,
  pack: {} as Pack,
  aliases: {} as Pack,
  ready: false,
})

export function isAutoKey(key: string): boolean {
  const m = String(key || '').match(AUTO_KEY)
  if (!m) return false
  return m[1].split('_').length <= 3
}

export function parseAdminLocale(raw?: string | null): AdminLocale {
  const s = String(raw || '')
    .trim()
    .toLowerCase()
    .replace(/-/g, '_')
  if (s.startsWith('en')) return 'en'
  return 'zh'
}

export function phpFileFor(locale: AdminLocale): 'zh_cn' | 'en_us' {
  return locale === 'en' ? 'en_us' : 'zh_cn'
}

export function rustLangFor(locale: AdminLocale): string {
  return locale === 'en' ? 'en' : 'zh-CN'
}

function readCookie(name: string): string {
  if (!import.meta.client) return ''
  const hit = document.cookie.split(';').map((x) => x.trim()).find((x) => x.startsWith(`${name}=`))
  if (!hit) return ''
  return decodeURIComponent(hit.slice(name.length + 1))
}

export function readStoredLocale(): AdminLocale {
  if (!import.meta.client) return 'zh'
  const ls = localStorage.getItem(LS) || localStorage.getItem(LS_LEGACY)
  if (ls) return parseAdminLocale(ls)
  const cookie = readCookie(COOKIE) || readCookie('lang')
  if (cookie) return parseAdminLocale(cookie)
  return 'zh'
}

export function persistLocale(locale: AdminLocale) {
  if (!import.meta.client) return
  localStorage.setItem(LS, locale)
  localStorage.setItem(LS_LEGACY, phpFileFor(locale))
  const maxAge = 31536000
  const val = `${COOKIE}=${locale}; max-age=${maxAge}; SameSite=Lax`
  document.cookie = `${val}; path=/`
  document.cookie = `${val}; path=/admin/`
}

function replaceParams(text: string, params: unknown): string {
  if (!params) return text
  const arr = Array.isArray(params) ? params : [params]
  let output = text
  for (let i = 0; i < arr.length; i++) {
    output = output.split(`{${i}}`).join(String(arr[i] ?? ''))
  }
  return output
}

function lookup(key: string): string | undefined {
  // Touch reactive fields so Options/setup templates re-render on locale change.
  const pack = adminI18n.pack
  const aliases = adminI18n.aliases
  void adminI18n.locale
  if (!key) return undefined
  if (Object.prototype.hasOwnProperty.call(pack, key)) return pack[key]
  const alias = aliases[key]
  if (alias && Object.prototype.hasOwnProperty.call(pack, alias)) return pack[alias]
  const prefixed = key.indexOf('.') === -1 ? `lc.${key}` : ''
  if (prefixed && Object.prototype.hasOwnProperty.call(pack, prefixed)) return pack[prefixed]
  return undefined
}

export function lc(key: string, params?: unknown, fallback?: string): string {
  const text = lookup(String(key || ''))
  return replaceParams(text ?? fallback ?? key ?? '', params)
}

let aliasesPromise: Promise<void> | null = null

export function loadAliases(): Promise<void> {
  if (!aliasesPromise) {
    aliasesPromise = $fetch<Pack>('/admin/php-admin/lang/aliases.json')
      .then((a) => {
        adminI18n.aliases = a && typeof a === 'object' ? a : {}
      })
      .catch(() => {
        adminI18n.aliases = {}
      })
  }
  return aliasesPromise
}

export async function loadLangPack(locale?: AdminLocale | string): Promise<void> {
  const loc = parseAdminLocale(locale ?? readStoredLocale())
  adminI18n.locale = loc
  const file = phpFileFor(loc)
  try {
    const pack = await $fetch<Pack>(`/admin/php-admin/lang/${file}.json`)
    adminI18n.pack = pack && typeof pack === 'object' ? pack : {}
  } catch {
    adminI18n.pack = {}
  }
  adminI18n.ready = true
  if (import.meta.client) {
    window.yunAdminI18n = {
      lang: file,
      messages: adminI18n.pack,
      lc: adminI18n.pack,
      keys: Object.keys(adminI18n.pack),
    }
    window.lc = lc
  }
}

/** Persist + load pack. Caller should also `setLocale` (Nuxt / Element Plus) and reload if needed. */
export async function setAdminLocale(locale: AdminLocale | string): Promise<AdminLocale> {
  const loc = parseAdminLocale(locale)
  persistLocale(loc)
  await loadAliases()
  await loadLangPack(loc)
  return loc
}

declare global {
  interface Window {
    yunAdminI18n?: { lang: string; messages: Pack; lc: Pack; keys: string[] }
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
