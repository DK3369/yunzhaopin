import { createHash } from 'node:crypto'
import { brotliCompressSync, gzipSync } from 'node:zlib'
import type { H3Event } from 'h3'
import {
  bundledLegacyCss,
  bundledSiteCss,
  cssCacheVer,
  cssMember,
  cssSkin,
  type CssEnd,
  type CssPack,
} from '../utils/legacyCss'

type CssBlob = { raw: Buffer; gz: Buffer; br: Buffer; etag: string }

const blobs = new Map<string, CssBlob>()

function packCss(css: string): CssBlob {
  const raw = Buffer.from(css, 'utf8')
  return {
    raw,
    gz: gzipSync(raw, { level: 6 }),
    br: brotliCompressSync(raw),
    etag: createHash('sha256').update(raw).digest('hex').slice(0, 16),
  }
}

function blobFor(key: string, css: string): CssBlob {
  const hit = blobs.get(key)
  if (hit) return hit
  const blob = packCss(css)
  blobs.set(key, blob)
  if (blobs.size > 48) {
    const first = blobs.keys().next().value
    if (first) blobs.delete(first)
  }
  return blob
}

function sendCss(event: H3Event, css: string, cacheKey: string, ver: string) {
  const blob = blobFor(cacheKey, css)
  const inm = String(getHeader(event, 'if-none-match') || '').replaceAll('"', '')
  if (inm && inm === blob.etag) {
    setResponseStatus(event, 304)
    return ''
  }
  setHeader(event, 'content-type', 'text/css; charset=utf-8')
  setHeader(event, 'etag', `"${blob.etag}"`)
  setHeader(event, 'vary', 'Accept-Encoding')
  setHeader(
    event,
    'cache-control',
    ver ? 'public, max-age=31536000, immutable' : 'public, max-age=300',
  )
  const enc = String(getHeader(event, 'accept-encoding') || '').toLowerCase()
  if (enc.includes('br')) {
    setHeader(event, 'content-encoding', 'br')
    return blob.br
  }
  if (enc.includes('gzip') || enc.includes('deflate')) {
    setHeader(event, 'content-encoding', 'gzip')
    return blob.gz
  }
  return blob.raw
}

export async function serveLegacyCss(event: H3Event, kind: CssPack) {
  const q = getQuery(event)
  const ver = cssCacheVer(q.v ?? q.cachecode)
  const body = await bundledLegacyCss(kind, ver || '0')
  return sendCss(event, body, `pack:${kind}:${ver || '0'}`, ver)
}

export async function serveSiteCss(event: H3Event, end: CssEnd) {
  const q = getQuery(event)
  const ver = cssCacheVer(q.v ?? q.cachecode)
  const member = cssMember(q.m)
  const skin = cssSkin(q.skin)
  const body = await bundledSiteCss(end, member, skin, ver || '0')
  return sendCss(event, body, `site:${end}:${member}:${skin || '_'}:${ver || '0'}`, ver)
}
