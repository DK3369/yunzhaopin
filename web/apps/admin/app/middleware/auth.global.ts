type AdminMe = { usertype?: number }

export default defineNuxtRouteMiddleware(async (to) => {
  if (to.path === '/login' || to.path === '/hello') return

  // SPA 下每次点菜单都会跑全局中间件；鉴权结果缓存到本次会话，避免每页卡住等 /admin-me。
  const me = useState<AdminMe | null>('admin-me', () => null)
  if (me.value?.usertype === 3) return

  try {
    const data = await $fetch<AdminMe>(bffUrl('/api/auth/admin-me'), { credentials: 'include' })
    if (data.usertype !== 3) {
      me.value = null
      return navigateTo('/login')
    }
    me.value = data
  } catch {
    me.value = null
    return navigateTo('/login')
  }
})
