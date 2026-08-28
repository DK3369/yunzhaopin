import { bffUrl } from '~/utils/bff'
import type { ApiEnvelope } from '~/utils/envelope'
import { readStoredLocale, rustLangFor } from '~/utils/phpLc'
import { resolvePhpAction } from '~/utils/phpMap'

type PhpEnvelope = { error: number; msg?: string; data?: unknown }

function formToObject(params: unknown): Record<string, unknown> {
  if (!params) return {}
  if (typeof FormData !== 'undefined' && params instanceof FormData) {
    const o: Record<string, unknown> = {}
    params.forEach((v, k) => {
      o[k] = v
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
        data: '',
      },
    }
  }
  const req = action.transformReq ? action.transformReq(body) : body
  try {
    const env = await postAdmin(action.path, req)
    if (env.code !== 200) {
      return { data: { error: 1, msg: env.msg || env.key || 'error', data: env.data } }
    }
    const data = action.transformRes ? action.transformRes(env.data) : env.data
    if (action.rawBody) {
      return { data: (data && typeof data === 'object' ? data : {}) as Record<string, unknown> }
    }
    return { data: { error: 0, msg: env.msg || 'ok', data } }
  } catch (err: unknown) {
    const anyErr = err as { data?: ApiEnvelope<unknown>; message?: string }
    const env = anyErr?.data
    return {
      data: {
        error: 1,
        msg: env?.msg || anyErr?.message || 'request failed',
        data: env?.data ?? '',
      },
    }
  }
}
