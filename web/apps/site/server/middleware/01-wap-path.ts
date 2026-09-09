import { mapWapPath } from '~/utils/site'

export default defineEventHandler((event) => {
  const url = getRequestURL(event)
  const path = url.pathname
  if (path !== '/wap' && !path.toLowerCase().startsWith('/wap/')) return
  const dest = mapWapPath(path, url.searchParams.toString())
  if (!dest || dest === path) return
  return sendRedirect(event, dest, 301)
})
