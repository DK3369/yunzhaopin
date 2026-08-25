import { unwrapEnvelope, ApiError, type ApiEnvelope } from '~/utils/envelope'

type Verb = 'GET' | 'POST'

export function useApi() {
  const request = async <T>(path: string, method: Verb, payload?: Record<string, unknown>): Promise<T> => {
    const url = `/api/proxy${path}`
    try {
      const body = await $fetch<ApiEnvelope<T>>(url, {
        method,
        query: method === 'GET' ? payload : undefined,
        body: method === 'POST' ? payload ?? {} : undefined,
        credentials: 'include',
      })
      return unwrapEnvelope(body)
    } catch (err: unknown) {
      const anyErr = err as { data?: ApiEnvelope<unknown>; statusCode?: number }
      const envelope = anyErr?.data
      if (envelope?.key === 'session_expired') {
        await $fetch('/api/auth/refresh', { method: 'POST', credentials: 'include' }).catch(() => undefined)
        const retry = await $fetch<ApiEnvelope<T>>(url, {
          method,
          query: method === 'GET' ? payload : undefined,
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
