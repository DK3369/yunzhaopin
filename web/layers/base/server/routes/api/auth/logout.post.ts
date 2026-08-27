import { rustFetch } from '../../../utils/rust'
import { clearAccessCookie } from '../../../utils/auth-cookie'

export default defineEventHandler(async (event) => {
  try {
    await rustFetch(event, '/v1/wap/logout', { body: {} })
  } catch {
    // still clear the cookie
  }
  clearAccessCookie(event)
  return { ok: true }
})
