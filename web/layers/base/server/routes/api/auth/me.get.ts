import { rustFetch } from '../../../utils/rust'

export default defineEventHandler(async (event) => {
  return rustFetch(event, '/v1/wap/me', { body: {} })
})
