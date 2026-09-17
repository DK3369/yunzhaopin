export const ACCESS_COOKIE = 'token'

export function cookieFlags(event: Parameters<typeof setCookie>[0], persist = true) {
  const secure = useRuntimeConfig(event).cookieSecure
  const flags: {
    httpOnly: boolean
    sameSite: 'strict'
    path: string
    secure: boolean
    maxAge?: number
  } = {
    httpOnly: true,
    sameSite: 'strict',
    path: '/',
    secure,
  }
  if (persist) flags.maxAge = 60 * 60 * 24 * 7
  return flags
}

export function setAccessCookie(
  event: Parameters<typeof setCookie>[0],
  token: string,
  persist = true,
) {
  setCookie(event, ACCESS_COOKIE, token, cookieFlags(event, persist))
}

export function clearAccessCookie(event: Parameters<typeof setCookie>[0]) {
  deleteCookie(event, ACCESS_COOKIE, { path: '/', httpOnly: true, sameSite: 'strict' })
}
