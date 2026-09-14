type SiteMe = { uid?: number; usertype?: number }

/**
 * 会员路径按 /api/auth/me 的 usertype 互斥：求职进不了 /com，招聘进不了 /user。
 */
export default defineNuxtRouteMiddleware(async (to) => {
  if (!to.path.startsWith('/user') && !to.path.startsWith('/com')) return

  const cached = useNuxtData<SiteMe | null>('auth-me')
  let me = cached.data.value
  if (me == null) {
    try {
      const headers: Record<string, string> = {}
      if (import.meta.server) {
        const cookie = useRequestHeaders(['cookie']).cookie
        if (cookie) headers.cookie = cookie
      }
      me = await $fetch<SiteMe>(bffUrl('/api/auth/me'), { credentials: 'include', headers })
      cached.data.value = me
    } catch {
      me = null
      cached.data.value = null
    }
  }

  if (!me?.uid) {
    return navigateTo({ path: '/login', query: { next: to.fullPath } })
  }

  const ut = Number(me.usertype)
  if (to.path.startsWith('/user') && ut === 2) return navigateTo('/com')
  if (to.path.startsWith('/com') && ut === 1) return navigateTo('/user')
})
