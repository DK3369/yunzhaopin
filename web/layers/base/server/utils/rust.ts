import { ACCESS_COOKIE } from './auth-cookie'

type Envelope = { code: number; key: string; msg: string; data: unknown }

export async function rustFetch<T>(
  event: Parameters<typeof getCookie>[0],
  path: string,
  opts: { method?: string; body?: unknown; token?: string } = {},
): Promise<T> {
  const rustApi = useRuntimeConfig(event).rustApi
  const token = opts.token ?? getCookie(event, ACCESS_COOKIE)
  const method = (opts.method || 'POST') as 'GET' | 'POST'
  const headers: Record<string, string> = {
    accept: 'application/json',
  }
  if (method === 'POST') {
    headers['content-type'] = 'application/json'
  }
  if (token) {
    headers.authorization = `Bearer ${token}`
  }
  const res = await $fetch<Envelope>(`${rustApi}${path}`, {
    method,
    headers,
    body: method === 'POST' ? (opts.body as Record<string, unknown> | undefined) : undefined,
    query: method === 'GET' ? (opts.body as Record<string, unknown>) : undefined,
  })
  if (res.code !== 200) {
    throw createError({ statusCode: res.code, statusMessage: res.msg, data: { key: res.key } })
  }
  return res.data as T
}
