import { rustFetch } from '../../../utils/rust'
import { setAccessCookie } from '../../../utils/auth-cookie'

type LoginBody = { username: string; password: string; authcode?: string; captcha_cid?: string }
type TokenData = { uid: number; usertype: number; access_token: string }

export default defineEventHandler(async (event) => {
  const body = await readBody<LoginBody>(event)
  const data = await rustFetch<TokenData>(event, '/v1/wap/login', { body })
  setAccessCookie(event, data.access_token)
  return { uid: data.uid, usertype: data.usertype }
})
