export default defineEventHandler(async (event) => {
  const rustApi = String(useRuntimeConfig(event).rustApi || 'http://127.0.0.1:3003').replace(/\/$/, '')
  const rawUrl = event.path || '/callback'
  const suffix = rawUrl.replace(/^\/callback\/?/, '')
  const u = getRequestURL(event)
  u.searchParams.delete('lang')
  const qs = u.search
  const target = `${rustApi}/callback/${suffix}${qs}`
  const method = getMethod(event)
  const contentType = getHeader(event, 'content-type') || ''
  const body = method === 'GET' || method === 'HEAD' ? undefined : await readRawBody(event)
  const cookie = getHeader(event, 'cookie')
  const res = await fetch(target, {
    method,
    headers: {
      ...(contentType ? { 'content-type': contentType } : {}),
      accept: getHeader(event, 'accept') || '*/*',
      ...rustLangHeaders(event),
      ...(cookie ? { cookie } : {}),
    },
    body: body as BodyInit | undefined,
  })
  const buf = Buffer.from(await res.arrayBuffer())
  setResponseStatus(event, res.status)
  const ct = res.headers.get('content-type')
  if (ct) setHeader(event, 'content-type', ct)
  return buf
})
