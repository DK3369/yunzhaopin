export default defineEventHandler((event) => {
  if (event.method !== 'GET' && event.method !== 'HEAD') return
  const url = getRequestURL(event)
  const path = url.pathname.replace(/\/+$/, '') || '/'
  if (path !== '/' && path !== '/admin') return
  if (url.searchParams.has('b')) return
  const tag = String(useRuntimeConfig(event).public.adminAssetTag || 'dev')
  return sendRedirect(event, `/admin/?b=${encodeURIComponent(tag)}`, 302)
})
