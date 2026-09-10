export default defineEventHandler(async (event) => {
  const q = getQuery(event)
  const ver = cssCacheVer(q.v ?? q.cachecode)
  const body = await bundledLegacyCss('pc', ver || '0')
  setHeader(event, 'content-type', 'text/css; charset=utf-8')
  setHeader(
    event,
    'cache-control',
    ver ? 'public, max-age=31536000, immutable' : 'public, max-age=300',
  )
  if (ver) setHeader(event, 'etag', `"pc-${ver}"`)
  return body
})
