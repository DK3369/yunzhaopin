/**
 * Unwrap stored blobs: numbered key, Chinese, or key+suffix
 * (`common_02313`, `用户:namecommon_02313`). Longest zh-pack prefix → key → lc().
 */

export const AUTO_KEY_RE = /^([a-z][a-z0-9_]*)_([0-9]{5})$/
const AUTO_KEY_PREFIX_RE = /^([a-z]+(?:_[a-z]+){0,2})_([0-9]{5})/
const CJK_RE = /[\u3400-\u9fff]/

export type PackedLocale = 'zh' | 'en'

export type PackedLookup = {
  locale: PackedLocale
  lc: (key: string) => string
  extraZh?: Record<string, string>
  zhRoot?: Record<string, unknown>
}

export function isAutoKey(key: string): boolean {
  const m = AUTO_KEY_RE.exec(key)
  if (!m) return false
  return m[1].split('_').length <= 3
}

export function leadingAutoKey(text: string): string {
  const m = AUTO_KEY_PREFIX_RE.exec(text)
  if (!m) return ''
  const key = `${m[1]}_${m[2]}`
  return isAutoKey(key) ? key : ''
}

function joinPacked(left: string, next: string): string {
  if (!left) return next
  if (!next) return left
  const a = left.charAt(left.length - 1)
  const b = next.charAt(0)
  if (/[A-Za-z0-9]/.test(a) && /[A-Za-z]/.test(b)) return `${left} ${next}`
  return left + next
}

type ZhPrefix = { zh: string; key: string }

const catalogCache = new Map<string, ZhPrefix[]>()

function zhPrefixCatalog(lookup: PackedLookup): ZhPrefix[] {
  const root = lookup.zhRoot
  const n = root && typeof root === 'object' ? Object.keys(root).length : 0
  const extraN = lookup.extraZh ? Object.keys(lookup.extraZh).length : 0
  const cacheKey = `${lookup.locale}:${n}:${extraN}`
  const hit = catalogCache.get(cacheKey)
  if (hit) return hit
  const items: ZhPrefix[] = []
  const seen = new Set<string>()
  const push = (zh: string, key: string) => {
    const t = String(zh || '')
    if (t.length < 2 || t.includes('{') || seen.has(t)) return
    seen.add(t)
    items.push({ zh: t, key })
  }
  if (lookup.extraZh) {
    for (const [key, zh] of Object.entries(lookup.extraZh)) push(zh, key)
  }
  if (root && typeof root === 'object') {
    for (const [key, val] of Object.entries(root)) {
      if (typeof val === 'string' && isAutoKey(key)) push(val, key)
    }
  }
  items.sort((a, b) => b.zh.length - a.zh.length)
  catalogCache.set(cacheKey, items)
  return items
}

function longestZhPrefix(text: string, lookup: PackedLookup): ZhPrefix | undefined {
  for (const row of zhPrefixCatalog(lookup)) {
    if (text.startsWith(row.zh)) return row
  }
  return undefined
}

export function runPackedTranslate(text: unknown, lookup: PackedLookup): string {
  let s = String(text ?? '')
  if (!s) return ''
  const zhOnlyKeys = lookup.locale === 'zh'
  let out = ''
  let guard = 0
  while (s && guard++ < 80) {
    const key = leadingAutoKey(s)
    if (key) {
      out = joinPacked(out, lookup.lc(key))
      s = s.slice(key.length)
      continue
    }
    if (!zhOnlyKeys) {
      const hit = longestZhPrefix(s, lookup)
      if (hit) {
        out = joinPacked(out, lookup.lc(hit.key))
        s = s.slice(hit.zh.length)
        continue
      }
    }
    if (zhOnlyKeys) {
      out += s
      break
    }
    if (!CJK_RE.test(s.charAt(0))) {
      let i = 1
      while (i < s.length && !CJK_RE.test(s.charAt(i)) && !leadingAutoKey(s.slice(i))) i++
      out += s.slice(0, i)
      s = s.slice(i)
      continue
    }
    out += s.charAt(0)
    s = s.slice(1)
  }
  return out
}
