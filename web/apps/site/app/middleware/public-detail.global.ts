import { isLoggedIn } from '~/utils/site'

/**
 * 职位/简历/企业详情必须登录。这只是跳转，不是授权；Rust 详情接口仍会 401。
 * 不要写成 startsWith('/jobs')，否则列表页也会被赶走。
 */
function needsLogin(path: string, query: Record<string, unknown>) {
  if (/^\/jobs\/\d+(\/|$)/.test(path)) return true
  if (/^\/resumes\/\d+(\/|$)/.test(path)) return true
  if (/^\/companies\/\d+(\/|$)/.test(path)) return true
  if (/^\/share\/job\/\d+(\/|$)/.test(path)) return true
  if (/^\/share\/company\/\d+(\/|$)/.test(path)) return true
  if (path === '/map' || path.startsWith('/map/')) {
    return Number(query.job_id || 0) > 0
  }
  return false
}

export default defineNuxtRouteMiddleware(async (to) => {
  if (!needsLogin(to.path, to.query as Record<string, unknown>)) return
  const { data: me } = await useAuthMe()
  if (isLoggedIn(me.value)) return
  return navigateTo({ path: '/login', query: { next: to.fullPath } })
})
