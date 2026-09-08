import { bffUrl } from '~/utils/bff'
import type { ApiEnvelope } from '~/utils/envelope'
import { lc, readStoredLocale, rustLangFor } from '~/utils/phpLc'
import { resolvePhpAction } from '~/utils/phpMap'

type PhpEnvelope = { error: number; msg?: string; data?: unknown }

function phpFailData(data: unknown): unknown {
  if (data && typeof data === 'object') return data
  return { list: [], total: 0 }
}

/**
 * PHP `admin_json` / `render_json` drop every parenthesised run before the
 * message leaves the server, turning `下载记录(ID:1,2)删除成功` into
 * `下载记录删除成功`; the ids are for the admin log only. Fullwidth brackets are
 * folded first, and empty `()` is left alone, both as in their regex.
 */
function phpMsg(msg: string): string {
  return msg
    .replace(/（/g, '(')
    .replace(/）/g, ')')
    .replace(/\([^)]+?\)/g, '')
}

/**
 * Translate a `search_list` dropdown in place.
 *
 * The Rust endpoints hand back locale keys for the filter label and for every
 * option, mirroring what PHP put in these payloads. PHP never translated them
 * either — its i18n pass skips output fields named `name`, so the console used
 * to show bare keys like `admin_user_00047`. Resolving them here fixes every
 * page that renders a `search_list` instead of each one doing its own `lc()`.
 */
function localizeSearchList(data: unknown): void {
  const list = (data as Record<string, unknown> | null)?.search_list
  if (!list || typeof list !== 'object') return
  for (const item of Object.values(list as Record<string, unknown>)) {
    const entry = item as Record<string, unknown> | null
    if (!entry || typeof entry !== 'object') continue
    if (typeof entry.name === 'string') entry.name = lc(entry.name)
    const opts = entry.value
    if (!opts || typeof opts !== 'object') continue
    for (const [k, v] of Object.entries(opts as Record<string, unknown>)) {
      if (typeof v === 'string') (opts as Record<string, unknown>)[k] = lc(v)
    }
  }
}

function formToObject(params: unknown): Record<string, unknown> {
  if (!params) return {}
  if (typeof FormData !== 'undefined' && params instanceof FormData) {
    const o: Record<string, unknown> = {}
    params.forEach((v, k) => {
      const key = k.endsWith('[]') ? k.slice(0, -2) : k
      if (typeof File !== 'undefined' && v instanceof File) {
        return
      }
      if (Object.prototype.hasOwnProperty.call(o, key)) {
        const prev = o[key]
        o[key] = Array.isArray(prev) ? [...prev, v] : [prev, v]
      } else {
        o[key] = v
      }
    })
    return o
  }
  if (typeof params === 'object') return { ...(params as Record<string, unknown>) }
  return {}
}

async function postAdmin(path: string, body: Record<string, unknown>): Promise<ApiEnvelope<unknown>> {
  const page = body.page != null ? Number(body.page) : undefined
  const page_size = body.page_size != null ? Number(body.page_size) : undefined
  const query: Record<string, unknown> = {}
  if (page) query.page = page
  if (page_size) query.page_size = page_size
  const loc = readStoredLocale()
  return await $fetch<ApiEnvelope<unknown>>(bffUrl(`/api/proxy${path}`), {
    method: 'POST',
    credentials: 'include',
    query: { ...query, lang: loc },
    headers: { 'accept-language': rustLangFor(loc) },
    body,
  })
}

/**
 * PHP-shaped adapter: pages keep `httpPost('m=user&c=company_job&a=index', params)`.
 * Resolves to POST `/v1/admin/...` via BFF. Returns `{ data: { error, msg, data } }` like axios.
 */
export async function httpPost(
  url: string,
  params: unknown = null,
  _config: Record<string, unknown> = {},
  _newBase = '',
): Promise<{ data: PhpEnvelope | Record<string, unknown> }> {
  const body = formToObject(params)
  delete body.pytoken
  const action = resolvePhpAction(url)
  if (!action) {
    return {
      data: {
        error: 1,
        msg: `未映射的后台接口: ${url}`,
        data: phpFailData(''),
      },
    }
  }
  const req = action.transformReq ? action.transformReq(body) : body
  try {
    const env = await postAdmin(action.path, req)
    if (env.code !== 200) {
      return { data: { error: 1, msg: phpMsg(env.msg || env.key || 'error'), data: phpFailData(env.data) } }
    }
    const data = action.transformRes ? action.transformRes(env.data) : env.data
    localizeSearchList(data)
    if (action.phpError != null) {
      return { data: { error: action.phpError, msg: phpMsg(env.msg || 'ok'), data } }
    }
    if (action.rawBody) {
      return { data: (data && typeof data === 'object' ? data : {}) as Record<string, unknown> }
    }
    return { data: { error: 0, msg: phpMsg(env.msg || 'ok'), data } }
  } catch (err: unknown) {
    const anyErr = err as { data?: ApiEnvelope<unknown>; message?: string }
    const env = anyErr?.data
    return {
      data: {
        error: 1,
        msg: phpMsg(env?.msg || anyErr?.message || 'request failed'),
        data: phpFailData(env?.data),
      },
    }
  }
}
