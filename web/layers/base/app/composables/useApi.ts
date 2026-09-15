import { unwrapEnvelope, ApiError, type ApiEnvelope } from '~/utils/envelope'
import { bffUrl } from '~/utils/bff'
import { parseWebLocale, rustLangFor } from '../utils/locale'

type Verb = 'GET' | 'POST'

function pagingQuery(payload?: Record<string, unknown>): Record<string, unknown> | undefined {
  if (!payload) return undefined
  const query: Record<string, unknown> = {}
  if (payload.page != null) query.page = payload.page
  if (payload.page_size != null) query.page_size = payload.page_size
  return Object.keys(query).length ? query : undefined
}

function dropLangQuery(payload?: Record<string, unknown>): Record<string, unknown> | undefined {
  if (!payload) return undefined
  const query: Record<string, unknown> = {}
  for (const [k, v] of Object.entries(payload)) {
    if (k === 'lang') continue
    query[k] = v
  }
  return Object.keys(query).length ? query : undefined
}

export function useApi() {
  const scope = useLocaleScope()
  const localeCookie = useCookie(scope.key)

  const request = async <T>(path: string, method: Verb, payload?: Record<string, unknown>): Promise<T> => {
    const url = bffUrl(`/api/proxy${path}`)
    const loc = parseWebLocale(localeCookie.value)
    const query = method === 'GET' ? dropLangQuery(payload) : pagingQuery(payload)
    const headers = { 'accept-language': rustLangFor(loc) }
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
