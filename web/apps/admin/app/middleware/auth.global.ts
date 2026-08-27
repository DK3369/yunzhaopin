export default defineNuxtRouteMiddleware(async (to) => {
  if (to.path === '/login' || to.path === '/hello') return
  try {
    const me = await $fetch<{ usertype?: number }>(bffUrl('/api/auth/admin-me'), { credentials: 'include' })
    if (me.usertype !== 3) {
      return navigateTo('/login')
    }
  } catch {
    return navigateTo('/login')
  }
})
