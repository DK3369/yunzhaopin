export default defineNitroPlugin((nitroApp) => {
  const rustApi = useRuntimeConfig().rustApi
  console.log(`[admin] rustApi=${rustApi}`)

  nitroApp.hooks.hook('beforeResponse', (event) => {
    const path = event.path || ''
    if (event.method !== 'GET' && event.method !== 'HEAD') return
    if (path.includes('/_nuxt/') || path.includes('/_n/') || path.includes('/php-admin/')) return
    const accept = getHeader(event, 'accept') || ''
    if (!accept.includes('text/html') && path !== '/' && path !== '/admin' && path !== '/admin/') return
    setHeader(event, 'Clear-Site-Data', '"cache"')
  })
})
