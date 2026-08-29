import { ACCESS_COOKIE } from '../../utils/auth-cookie'
import { rustLangHeaders } from '../../utils/lang'

type Envelope = { code: number; key: string; msg: string; data?: { url?: string; key?: string } | '' }

function phpFail(c: string, msg: string) {
  if (c === 'uploadfile') return { errno: 1, message: msg, data: {} }
  return { code: 1, msg, data: {} }
}

function phpOk(c: string, url: string) {
  if (c === 'uploadfile') return { errno: 0, data: { url } }
  return { code: 0, msg: 'ok', data: { url } }
}

/** PHP `baseUrl + 'm=index&c=uploadfile'` / `layui_upload` / `common_upload` multipart. */
export default defineEventHandler(async (event) => {
  const q = getQuery(event)
  const c = String(q.c || '')
  const allowed = new Set(['uploadfile', 'layui_upload', 'common_upload', 'wangeditorUpfile'])
  if (!allowed.has(c)) {
    setResponseStatus(event, 404)
    return phpFail(c, `未映射的上传接口: ${c}`)
  }

  const parts = await readMultipartFormData(event)
  const file = parts?.find((p) => p.name === 'file' && p.data && p.data.length > 0)
  if (!file) {
    return phpFail(c, 'empty body')
  }

  const rustApi = useRuntimeConfig(event).rustApi
  const token = getCookie(event, ACCESS_COOKIE)
  const ct = file.type || 'application/octet-stream'
  const headers: Record<string, string> = {
    accept: 'application/json',
    'content-type': ct,
    ...rustLangHeaders(event),
  }
  if (token) headers.authorization = `Bearer ${token}`

  const res = await $fetch<Envelope>(`${rustApi}/v1/admin/upload`, {
    method: 'POST',
    headers,
    body: file.data,
    ignoreResponseError: true,
  }).catch(() => ({ code: 502, key: 'upstream', msg: 'upstream error', data: '' as const }))

  if (res.code !== 200 || !res.data || typeof res.data !== 'object' || !res.data.url) {
    return phpFail(c, res.msg || 'upload failed')
  }
  return phpOk(c, res.data.url)
})
