export const ACCESS_COOKIE = 'token'

export function cookieFlags(event: Parameters<typeof setCookie>[0]) {
  const secure = useRuntimeConfig(event).cookieSecure
  return {
    httpOnly: true,
    sameSite: 'strict' as const,
    path: '/',
    secure,
    maxAge: 60 * 60 * 24 * 7,
  }
}

export function setAccessCookie(event: Parameters<typeof setCookie>[0], token: string) {
  setCookie(event, ACCESS_COOKIE, token, cookieFlags(event))
}

export function clearAccessCookie(event: Parameters<typeof setCookie>[0]) {
  deleteCookie(event, ACCESS_COOKIE, { path: '/', httpOnly: true, sameSite: 'strict' })
}
