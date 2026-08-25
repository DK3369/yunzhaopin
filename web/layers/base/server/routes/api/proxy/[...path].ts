import { ACCESS_COOKIE } from '../../../utils/auth-cookie'

type Envelope = { code: number; key: string; msg: string; data: unknown }

export default defineEventHandler(async (event) => {
  const rustApi = useRuntimeConfig(event).rustApi
  const path = getRouterParam(event, 'path') || ''
  const urlPath = path.startsWith('v1/') || path.startsWith('v2/') ? `/${path}` : `/${path}`
  const method = event.method === 'GET' ? 'GET' : 'POST'
  const token = getCookie(event, ACCESS_COOKIE)
  const headers: Record<string, string> = { accept: 'application/json' }
  if (token) headers.authorization = `Bearer ${token}`

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
