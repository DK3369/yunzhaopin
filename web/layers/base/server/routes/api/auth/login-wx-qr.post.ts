import { rustFetch } from '../../../utils/rust'

type WxQr = { login_id: string; show_url: string; expire_seconds: number }

export default defineEventHandler(async (event) => {
  return await rustFetch<WxQr>(event, '/v1/wap/login/wx-qr', { body: {} })
})
