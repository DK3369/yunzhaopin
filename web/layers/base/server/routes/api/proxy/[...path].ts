import { ACCESS_COOKIE } from '../../../utils/auth-cookie'
import { rustLangHeaders } from '../../../utils/lang'

type Envelope = { code: number; key: string; msg: string; data: unknown }

export default defineEventHandler(async (event) => {
  const rustApi = useRuntimeConfig(event).rustApi
  const path = getRouterParam(event, 'path') || ''
  const urlPath = path.startsWith('v1/') || path.startsWith('v2/') ? `/${path}` : `/${path}`
  const method = event.method === 'GET' ? 'GET' : 'POST'
  const token = getCookie(event, ACCESS_COOKIE)
  const headers: Record<string, string> = {
    accept: 'application/json',
    ...rustLangHeaders(event),
  }
  if (token) headers.authorization = `Bearer ${token}`
  const ua = getHeader(event, 'user-agent')
  if (ua) headers['user-agent'] = ua
  const xff = getHeader(event, 'x-forwarded-for')
  const ip = getRequestIP(event, { xForwardedFor: true })
  if (xff) headers['x-forwarded-for'] = xff
  else if (ip) headers['x-forwarded-for'] = ip
  const xri = getHeader(event, 'x-real-ip')
  if (xri) headers['x-real-ip'] = xri
  else if (ip) headers['x-real-ip'] = ip
  const cookie = getHeader(event, 'cookie')
  if (cookie) headers.cookie = cookie

  let body: unknown
  if (method === 'POST') {
    headers['content-type'] = 'application/json'
    body = await readBody(event).catch(() => ({}))
  }

  const query = getQuery(event)

  const res = await $fetch<Envelope>(`${rustApi}${urlPath}`, {
    method,
    headers,
    body: body as Record<string, unknown> | undefined,
    query,
    ignoreResponseError: true,
  })

  if (res.key === 'session_expired') {
    try {
      const refreshed = await $fetch<Envelope>(`${rustApi}/v1/wap/refresh`, {
        method: 'POST',
        headers: {
          accept: 'application/json',
          'content-type': 'application/json',
          ...rustLangHeaders(event),
          authorization: token ? `Bearer ${token}` : '',
        },
        body: {},
      })
      if (refreshed.code === 200 && refreshed.data && typeof refreshed.data === 'object') {
        const access = (refreshed.data as { access_token?: string }).access_token
        if (access) {
          setCookie(event, ACCESS_COOKIE, access, {
            httpOnly: true,
            sameSite: 'strict',
            path: '/',
            secure: useRuntimeConfig(event).cookieSecure,
            maxAge: 60 * 60 * 24 * 7,
          })
          headers.authorization = `Bearer ${access}`
          return await $fetch<Envelope>(`${rustApi}${urlPath}`, {
            method,
            headers,
            body: body as Record<string, unknown> | undefined,
            query,
            ignoreResponseError: true,
          })
        }
      }
    } catch {
      // fall through
    }
  }

  setResponseStatus(event, res.code === 200 ? 200 : res.code)
  return res
})
