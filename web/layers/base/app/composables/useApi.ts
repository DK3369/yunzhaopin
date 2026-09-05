import { unwrapEnvelope, ApiError, type ApiEnvelope } from '~/utils/envelope'
import { bffUrl } from '~/utils/bff'
import { readStoredLocale, rustLangFor } from '../utils/locale'

type Verb = 'GET' | 'POST'
type Loc = 'zh' | 'en'

function pagingQuery(payload?: Record<string, unknown>): Record<string, unknown> | undefined {
  if (!payload) return undefined
  const query: Record<string, unknown> = {}
  if (payload.page != null) query.page = payload.page
  if (payload.page_size != null) query.page_size = payload.page_size
  return Object.keys(query).length ? query : undefined
}

function locFromRaw(raw: unknown): Loc | null {
  const s = String(raw || '')
    .trim()
    .toLowerCase()
    .replace(/_/g, '-')
  if (!s) return null
  if (s.startsWith('en')) return 'en'
  if (s.startsWith('zh') || s === 'cn') return 'zh'
  return null
}

export function useApi() {
  const i18n = useI18n()
  const route = useRoute()
  const scope = useLocaleScope()

  function currentLoc(): Loc {
    const fromQuery = locFromRaw(route.query.lang)
    if (fromQuery) return fromQuery
    const fromI18n = locFromRaw(i18n.locale.value)
    if (fromI18n) return fromI18n
    return readStoredLocale(scope.key, scope.fallback)
  }

  const request = async <T>(path: string, method: Verb, payload?: Record<string, unknown>): Promise<T> => {
    const url = bffUrl(`/api/proxy${path}`)
    const loc = currentLoc()
    const query = {
      ...(method === 'GET' ? payload : pagingQuery(payload)),
      lang: loc,
    }
    const headers = { 'accept-language': rustLangFor(loc, scope.key) }
    try {
      const body = await $fetch<ApiEnvelope<T>>(url, {
        method,
        query,
        headers,
        body: method === 'POST' ? payload ?? {} : undefined,
        credentials: 'include',
      })
      return unwrapEnvelope(body)
    } catch (err: unknown) {
      const anyErr = err as { data?: ApiEnvelope<unknown>; statusCode?: number }
      const envelope = anyErr?.data
      if (envelope?.key === 'session_expired') {
        await $fetch(bffUrl('/api/auth/refresh'), { method: 'POST', credentials: 'include' }).catch(() => undefined)
        const retry = await $fetch<ApiEnvelope<T>>(url, {
          method,
          query,
          headers,
          body: method === 'POST' ? payload ?? {} : undefined,
          credentials: 'include',
        })
        return unwrapEnvelope(retry)
      }
      if (envelope?.key) {
        throw new ApiError(envelope.code, envelope.key, envelope.msg)
      }
      throw err
    }
  }

  return {
    get: <T>(path: string, query?: Record<string, unknown>) => request<T>(path, 'GET', query),
    post: <T>(path: string, body?: Record<string, unknown>) => request<T>(path, 'POST', body),
  }
}
