import type { H3Event } from 'h3'
import { bundledLegacyCss, cssCacheVer, type CssPack } from '../utils/legacyCss'

export async function serveLegacyCss(event: H3Event, kind: CssPack) {
  const q = getQuery(event)
  const ver = cssCacheVer(q.v ?? q.cachecode)
  const body = await bundledLegacyCss(kind, ver || '0')
  setHeader(event, 'content-type', 'text/css; charset=utf-8')
  setHeader(
    event,
    'cache-control',
    ver ? 'public, max-age=31536000, immutable' : 'public, max-age=300',
  )
  if (ver) setHeader(event, 'etag', `"${kind}-${ver}"`)
  return body
}
