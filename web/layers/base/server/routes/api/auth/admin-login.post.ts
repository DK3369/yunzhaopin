import { rustFetch } from '../../../utils/rust'
import { setAccessCookie } from '../../../utils/auth-cookie'

type LoginBody = { username: string; password: string }
type TokenData = {
  uid: number
  usertype: number
  username?: string
  name?: string
  group_name?: string
  path?: string
  access_token: string
}

export default defineEventHandler(async (event) => {
  const body = await readBody<LoginBody>(event)
  const data = await rustFetch<TokenData>(event, '/v1/admin/login', { body })
  setAccessCookie(event, data.access_token)
  return {
    uid: data.uid,
    usertype: data.usertype,
    username: data.username,
    name: data.name,
    group_name: data.group_name,
    path: data.path,
  }
})
