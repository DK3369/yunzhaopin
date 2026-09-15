import { isComMemberPath, isMemberPath, isUserMemberPath } from '~/utils/site'

/**
 * 会员路径按 /api/auth/me 的 usertype 互斥：求职进不了 /com，招聘进不了 /user。
 * 必须用 `/com` 与 `/com/`，不能 `startsWith('/com')`，否则公开 `/companies` 会被当成企业中心。
 */
export default defineNuxtRouteMiddleware(async (to) => {
  if (!isMemberPath(to.path)) return

  const { data: me } = await useAuthMe()

  if (!me.value?.uid) {
    return navigateTo({ path: '/login', query: { next: to.fullPath } })
  }

  const ut = Number(me.value.usertype)
  if (isUserMemberPath(to.path) && ut === 2) return navigateTo('/com')
  if (isComMemberPath(to.path) && ut === 1) return navigateTo('/user')
})
