import { rustEnvelope, sendEnvelope } from '../../../utils/rust'
import { setAccessCookie } from '../../../utils/auth-cookie'

type TokenData = { uid: number; usertype: number; access_token: string }

function isTokenData(data: unknown): data is TokenData {
  return !!data && typeof data === 'object' && typeof (data as TokenData).access_token === 'string'
}

export default defineEventHandler(async (event) => {
  const res = await rustEnvelope<TokenData>(event, '/v1/wap/refresh', { body: {} })
  if (res.code !== 200 || !isTokenData(res.data)) {
    return sendEnvelope(event, res)
  }
  setAccessCookie(event, res.data.access_token)
  return { uid: res.data.uid, usertype: res.data.usertype }
})
