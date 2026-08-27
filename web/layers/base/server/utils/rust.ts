import { ACCESS_COOKIE } from './auth-cookie'
import { rustLangHeaders } from './lang'

export type Envelope<T = unknown> = { code: number; key: string; msg: string; data: T | '' }

function fallbackEnvelope(msg = 'upstream error'): Envelope {
  return { code: 502, key: 'upstream_error', msg, data: '' }
}

export async function rustEnvelope<T = unknown>(
  event: Parameters<typeof getCookie>[0],
  path: string,
  opts: { method?: string; body?: unknown; token?: string } = {},
): Promise<Envelope<T>> {
  const rustApi = useRuntimeConfig(event).rustApi
  const token = opts.token ?? getCookie(event, ACCESS_COOKIE)
  const method = (opts.method || 'POST') as 'GET' | 'POST'
  const headers: Record<string, string> = {
    accept: 'application/json',
    ...rustLangHeaders(event),
  }
  if (method === 'POST') {
    headers['content-type'] = 'application/json'
  }
  if (token) {
    headers.authorization = `Bearer ${token}`
  }
  try {
    const res = await $fetch<Envelope<T>>(`${rustApi}${path}`, {
      method,
      headers,
      body: method === 'POST' ? (opts.body as Record<string, unknown> | undefined) : undefined,
      query: method === 'GET' ? (opts.body as Record<string, unknown>) : undefined,
      ignoreResponseError: true,
    })
    if (!res || typeof res !== 'object' || typeof res.code !== 'number') {
      return fallbackEnvelope() as Envelope<T>
    }
    return res
  } catch {
    return fallbackEnvelope() as Envelope<T>
  }
}

/** Same as `/api/proxy`: HTTP status follows `code`, body stays `{code,key,msg,data}`. */
export function sendEnvelope<T>(event: Parameters<typeof setResponseStatus>[0], res: Envelope<T>): Envelope<T> {
  const code = Number(res.code)
  setResponseStatus(event, Number.isFinite(code) && code > 0 ? code : 502)
  return res
}

export async function rustFetch<T>(
  event: Parameters<typeof getCookie>[0],
  path: string,
  opts: { method?: string; body?: unknown; token?: string } = {},
): Promise<T> {
  const res = await rustEnvelope<T>(event, path, opts)
  if (res.code !== 200) {
    throw Object.assign(new Error(res.msg), { envelope: res })
  }
  return (res.data === '' ? undefined : res.data) as T
}
