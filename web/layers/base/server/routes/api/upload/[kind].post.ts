import { ACCESS_COOKIE } from '../../../utils/auth-cookie'
import { rustLangHeaders } from '../../../utils/lang'

const KINDS = new Set(['cert', 'avatar', 'company-logo', 'resume-photo', 'attachment'])

type Envelope = { code: number; key: string; msg: string; data: unknown }

export default defineEventHandler(async (event) => {
  const kind = String(getRouterParam(event, 'kind') || '')
  if (!KINDS.has(kind)) {
    throw createError({ statusCode: 400, statusMessage: 'unsupported upload kind' })
  }
  const rustApi = useRuntimeConfig(event).rustApi
  const token = getCookie(event, ACCESS_COOKIE)
  const ct = getHeader(event, 'content-type') || 'application/octet-stream'
  const body = await readRawBody(event, false)
  if (!body) {
    throw createError({ statusCode: 400, statusMessage: 'empty body' })
  }
  const headers: Record<string, string> = {
    accept: 'application/json',
    'content-type': ct,
    ...rustLangHeaders(event),
  }
  if (token) headers.authorization = `Bearer ${token}`
  const res = await $fetch<Envelope>(`${rustApi}/v1/wap/upload/${kind}`, {
    method: 'POST',
    headers,
    body,
    ignoreResponseError: true,
  })
  if (!res || res.code !== 200) {
    throw createError({
      statusCode: res?.code || 502,
      statusMessage: res?.msg || 'upload failed',
      data: { key: res?.key, msg: res?.msg },
    })
  }
  return res.data
})
