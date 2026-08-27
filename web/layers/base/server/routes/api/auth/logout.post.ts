import { rustEnvelope } from '../../../utils/rust'
import { clearAccessCookie } from '../../../utils/auth-cookie'

export default defineEventHandler(async (event) => {
  await rustEnvelope(event, '/v1/wap/logout', { body: {} })
  clearAccessCookie(event)
  return { ok: true }
})
