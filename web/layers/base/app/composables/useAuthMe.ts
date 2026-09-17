import { bffUrl } from '../utils/bff'
import { ssrCookieHeaders } from '../utils/ssrFetch'

export type AuthMe = {
  uid: number
  username: string
  usertype: number
  moblie?: string | null
  email?: string | null
  is_sub?: boolean
}

export const AUTH_ME_KEY = 'auth-me'

/** Single session payload: `/api/auth/me` (unwraps `/v1/wap/me`). SSR forwards Cookie. */
export function useAuthMe() {
  const requestFetch = useRequestFetch()
  return useAsyncData(
    AUTH_ME_KEY,
    async () => {
      try {
        return await requestFetch<AuthMe>(bffUrl('/api/auth/me'), {
          credentials: 'include',
          headers: ssrCookieHeaders(),
        })
      } catch {
        return null
      }
    },
    { default: () => null, ...reuseAsyncCache() },
  )
}
