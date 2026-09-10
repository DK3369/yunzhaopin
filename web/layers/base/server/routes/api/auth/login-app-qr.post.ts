import { rustFetch } from '../../../utils/rust'

type AppQr = {
  login_id: string
  usertype: number
  payload: string
  scan_url: string
  expire_seconds: number
}

export default defineEventHandler(async (event) => {
  const body = await readBody<{ usertype: number }>(event)
  return await rustFetch<AppQr>(event, '/v1/wap/login/app-qr', { body })
})
