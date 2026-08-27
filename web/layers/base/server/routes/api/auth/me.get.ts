import { rustEnvelope, sendEnvelope } from '../../../utils/rust'

export default defineEventHandler(async (event) => {
  const res = await rustEnvelope(event, '/v1/wap/me', { body: {} })
  if (res.code !== 200) {
    return sendEnvelope(event, res)
  }
  return res.data
})
