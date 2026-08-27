import { rustEnvelope } from '../../../utils/rust'
import { setAccessCookie } from '../../../utils/auth-cookie'

type LoginBody = { username: string; password: string }
type TokenData = {
  uid: number
  usertype: number
  username?: string
  name?: string
  group_name?: string
  path?: string
  access_token: string
}

function isTokenData(data: unknown): data is TokenData {
  return !!data && typeof data === 'object' && typeof (data as TokenData).access_token === 'string'
}

export default defineEventHandler(async (event) => {
  const body = await readBody<LoginBody>(event)
  const res = await rustEnvelope<TokenData>(event, '/v1/admin/login', { body })
  // PHP `render_json` always HTTP 200; business result lives in the envelope.
  // Do not throw createError — that becomes Nitro `{error,url,statusCode}`.
  setResponseStatus(event, 200)
  if (res.code !== 200 || !isTokenData(res.data)) {
    return {
      code: res.code || 401,
      key: res.key || 'bad_credentials',
      msg: res.msg || '',
      data: '' as const,
    }
  }
  setAccessCookie(event, res.data.access_token)
  return {
    code: 200,
    key: res.key || 'ok',
    msg: res.msg || 'ok',
    data: {
      uid: res.data.uid,
      usertype: res.data.usertype,
      username: res.data.username,
      name: res.data.name,
      group_name: res.data.group_name,
      path: res.data.path,
    },
  }
})
