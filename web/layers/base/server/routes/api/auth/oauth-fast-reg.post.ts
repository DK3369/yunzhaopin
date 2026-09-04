import { rustFetch } from '../../../utils/rust'
import { setAccessCookie } from '../../../utils/auth-cookie'

type TokenData = { uid: number; usertype: number; access_token: string }

export default defineEventHandler(async (event) => {
  const body = await readBody<Record<string, unknown>>(event)
  const data = await rustFetch<TokenData>(event, '/v1/wap/oauth/fast-reg', { body })
  if (data.access_token) setAccessCookie(event, data.access_token)
  return { uid: data.uid, usertype: data.usertype }
})
