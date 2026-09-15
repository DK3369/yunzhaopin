/**
 * 会员路径按 /api/auth/me 的 usertype 互斥：求职进不了 /com，招聘进不了 /user。
 */
export default defineNuxtRouteMiddleware(async (to) => {
  if (!to.path.startsWith('/user') && !to.path.startsWith('/com')) return

  const { data: me } = await useAuthMe()

  if (!me.value?.uid) {
    return navigateTo({ path: '/login', query: { next: to.fullPath } })
  }

  const ut = Number(me.value.usertype)
  if (to.path.startsWith('/user') && ut === 2) return navigateTo('/com')
  if (to.path.startsWith('/com') && ut === 1) return navigateTo('/user')
})
