import { rustEnvelope } from '../../../utils/rust'
import { setAccessCookie } from '../../../utils/auth-cookie'

type OAuthBody = { provider: string; code: string; state: string }
type TokenData = {
  uid: number
  usertype: number
  access_token?: string
  need_bind?: boolean
  ticket?: string
  provider?: string
}

const PATHS: Record<string, string> = {
  wechat: '/v1/wap/oauth/wechat/code-login',
  qq: '/v1/wap/oauth/qq/code-login',
  weibo: '/v1/wap/oauth/weibo/code-login',
}

export default defineEventHandler(async (event) => {
  const body = await readBody<OAuthBody>(event)
  const provider = String(body.provider || '').toLowerCase()
  const path = PATHS[provider]
  if (!path) {
    throw createError({ statusCode: 400, statusMessage: 'unknown oauth provider' })
  }
  const res = await rustEnvelope<TokenData>(event, path, {
    body: { code: body.code, state: body.state },
  })
  if (res.code !== 200) {
    throw createError({
      statusCode: res.code || 502,
      statusMessage: res.msg || 'upstream error',
      data: { key: res.key, msg: res.msg },
    })
  }
  const data = (res.data || {}) as TokenData
  if (data.need_bind) {
    return {
      need_bind: true,
      ticket: data.ticket || '',
      provider: data.provider || provider,
      uid: 0,
      usertype: 0,
    }
  }
  if (data.access_token) setAccessCookie(event, data.access_token)
  return { uid: data.uid, usertype: data.usertype, need_bind: false }
})
