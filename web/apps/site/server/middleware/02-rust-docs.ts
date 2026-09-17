import { rustLangHeaders } from '../../../../layers/base/server/utils/lang'

/**
 * job1 直连 :3001，没有 nginx `/yapi/`。
 * 把 Scalar / OpenAPI JSON 转到 Rust :3003，公网用 https://job1.ov6.com/docs/
 */
export default defineEventHandler(async (event) => {
  const path = getRequestURL(event).pathname
  if (path !== '/docs' && !path.startsWith('/docs/') && !path.startsWith('/api-docs')) {
    return
  }
  const rustApi = String(useRuntimeConfig(event).rustApi || 'http://127.0.0.1:3003').replace(
    /\/$/,
    '',
  )
  const u = getRequestURL(event)
  const target = `${rustApi}${path}${u.search}`
  const method = getMethod(event)
  const contentType = getHeader(event, 'content-type') || ''
  const body = method === 'GET' || method === 'HEAD' ? undefined : await readRawBody(event)
  const cookie = getHeader(event, 'cookie')
  const authorization = getHeader(event, 'authorization')
  const res = await fetch(target, {
    method,
    redirect: 'manual',
    headers: {
      ...(contentType ? { 'content-type': contentType } : {}),
      accept: getHeader(event, 'accept') || '*/*',
      ...rustLangHeaders(event),
      ...(cookie ? { cookie } : {}),
      ...(authorization ? { authorization } : {}),
    },
    body: body as BodyInit | undefined,
  })
  const buf = Buffer.from(await res.arrayBuffer())
  setResponseStatus(event, res.status)
  const pass = ['content-type', 'location', 'cache-control', 'x-robots-tag']
  for (const name of pass) {
    const v = res.headers.get(name)
    if (v) setHeader(event, name, v)
  }
  return buf
})
