export default defineEventHandler(async (event) => {
  const q = getQuery(event)
  const ver = cssCacheVer(q.v ?? q.cachecode)
  const body = await bundledLegacyCss('h5', ver || '0')
  setHeader(event, 'content-type', 'text/css; charset=utf-8')
  setHeader(
    event,
    'cache-control',
    ver ? 'public, max-age=31536000, immutable' : 'public, max-age=300',
  )
  if (ver) setHeader(event, 'etag', `"h5-${ver}"`)
  return body
})
