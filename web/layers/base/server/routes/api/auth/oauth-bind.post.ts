import { rustEnvelope } from '../../../utils/rust'
import { ACCESS_COOKIE } from '../../../utils/auth-cookie'

type OAuthBody = { provider: string; code: string; state: string }
type TokenData = {
  uid: number
  usertype: number
  access_token?: string
  need_bind?: boolean
  ticket?: string
}
type MeData = { uid?: number }

const PATHS: Record<string, string> = {
  wechat: '/v1/wap/oauth/wechat/code-login',
  qq: '/v1/wap/oauth/qq/code-login',
  weibo: '/v1/wap/oauth/weibo/code-login',
  google: '/v1/wap/oauth/google/code-login',
  facebook: '/v1/wap/oauth/facebook/code-login',
}

/** Logged-in member binds WeChat/QQ/Weibo (PHP wap binding). Does not switch accounts. */
export default defineEventHandler(async (event) => {
  const token = getCookie(event, ACCESS_COOKIE)
  if (!token) {
    throw createError({ statusCode: 401, statusMessage: 'login required' })
  }
  const body = await readBody<OAuthBody>(event)
  const provider = String(body.provider || '').toLowerCase()
  const path = PATHS[provider]
  if (!path) {
    throw createError({ statusCode: 400, statusMessage: 'unknown oauth provider' })
  }
  const me = await rustEnvelope<MeData>(event, '/v1/wap/me', { body: {}, token })
  if (me.code !== 200 || !me.data?.uid) {
    throw createError({
      statusCode: me.code || 401,
      statusMessage: me.msg || 'login required',
      data: { key: me.key, msg: me.msg },
    })
  }
  const res = await rustEnvelope<TokenData>(event, path, {
    body: { code: body.code, state: body.state },
    token,
  })
  if (res.code !== 200) {
    throw createError({
      statusCode: res.code || 502,
      statusMessage: res.msg || 'upstream error',
      data: { key: res.key, msg: res.msg },
    })
  }
  const data = (res.data || {}) as TokenData
  if (data.need_bind && data.ticket) {
    const bind = await rustEnvelope(event, '/v1/wap/oauth/bind-pending', {
      body: { ticket: data.ticket },
      token,
    })
    if (bind.code !== 200) {
      throw createError({
        statusCode: bind.code || 502,
        statusMessage: bind.msg || 'bind failed',
        data: { key: bind.key, msg: bind.msg },
      })
    }
    return { ok: true }
  }
  if (Number(data.uid) === Number(me.data.uid)) {
    return { ok: true }
  }
  throw createError({
    statusCode: 409,
    statusMessage: 'oauth already bound',
    data: { key: 'oauth_taken', msg: res.msg },
  })
})
