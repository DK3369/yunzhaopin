/** Headers so SSR $fetch to same-origin BFF keeps the incoming Cookie. */
export function ssrCookieHeaders(): Record<string, string> {
  const headers: Record<string, string> = {}
  if (import.meta.server) {
    const cookie = useRequestHeaders(['cookie']).cookie
    if (cookie) headers.cookie = cookie
  }
  return headers
}
