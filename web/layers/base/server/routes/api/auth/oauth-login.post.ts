import { rustFetch } from '../../../utils/rust'
import { setAccessCookie } from '../../../utils/auth-cookie'

type OAuthBody = { provider: string; code: string; state: string }
type TokenData = { uid: number; usertype: number; access_token: string }

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
  const data = await rustFetch<TokenData>(event, path, {
    body: { code: body.code, state: body.state },
  })
  setAccessCookie(event, data.access_token)
  return { uid: data.uid, usertype: data.usertype }
})
