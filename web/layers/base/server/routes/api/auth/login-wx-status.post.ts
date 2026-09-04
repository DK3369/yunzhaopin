import { rustFetch } from '../../../utils/rust'
import { setAccessCookie } from '../../../utils/auth-cookie'

type Body = { login_id: string }
type StatusData = {
  status: string
  uid?: number
  usertype?: number
  access_token?: string
}

export default defineEventHandler(async (event) => {
  const body = await readBody<Body>(event)
  const data = await rustFetch<StatusData>(event, '/v1/wap/login/wx-status', { body })
  if (data.status === 'ok' && data.access_token) {
    setAccessCookie(event, data.access_token)
  }
  return { status: data.status, uid: data.uid, usertype: data.usertype }
})
