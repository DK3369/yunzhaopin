export type NavItem = {
  id?: number
  label: string
  url?: string
  to: string
  icon?: string
  icon_n?: string
}

export type CatNode = {
  id: number
  parent_id: number
  name: string
  sort?: number
  children?: CatNode[]
}

export type JobLike = {
  id: number
  uid?: number
  name: string
  com_name?: string | null
  com_logo?: string | null
  logo?: string | null
  job_city_one?: string
  job_city_two?: string
  city_two?: string
  min_salary?: number
  max_salary?: number
  minsalary?: number
  maxsalary?: number
  salary?: number
  exp_n?: string
  edu_n?: string
  job_hy?: string
  hy_n?: string
  newtime?: boolean
  is_urgent?: boolean
  is_rec?: boolean
  lastupdate?: number
  lastupdate_n?: string
  welfare?: string[] | string
  welfare_n?: string
}

export type CompanyLike = {
  uid: number
  name?: string | null
  shortname?: string | null
  hy_n?: string
  pr_n?: string
  mun_n?: string
  city_one?: string
  city_two?: string
  logo?: string | null
  logo_n?: string | null
  hits?: number
  job_num?: number
  yyzz_status?: number
  rating?: number
  rating_name?: string | null
}

const MODULE_PATH: Record<string, string> = {
  job: '/jobs',
  resume: '/resumes',
  company: '/companies',
  article: '/articles',
  zph: '/fairs',
  announcement: '/announcements',
  login: '/login',
  register: '/register',
  tiny: '/tiny',
  once: '/once',
  part: '/parts',
  ask: '/questions',
  map: '/map',
  evaluate: '/eval',
  eval: '/eval',
  advice: '/advice',
  redeem: '/redeem',
  hr: '/hr',
  special: '/specials',
  gongzhao: '/gongzhao',
  index: '/',
  wap: '/',
  forgetpw: '/forgetpw',
}

export const DEFAULT_NAV: NavItem[] = [
  { label: '首页', to: '/' },
  { label: '找工作', to: '/jobs' },
  { label: '找人才', to: '/resumes' },
  { label: '找企业', to: '/companies' },
  { label: '招聘会', to: '/fairs' },
  { label: '资讯', to: '/articles' },
]

export const DEFAULT_H5_NAV: NavItem[] = [
  { label: '找工作', to: '/jobs', icon: '/legacy/h5/images/manage_full-time.png' },
  { label: '找企业', to: '/companies', icon: '/legacy/h5/images/company.png' },
  { label: '找人才', to: '/resumes', icon: '/legacy/h5/images/Please_resume.png' },
  { label: '招聘会', to: '/fairs', icon: '/legacy/h5/images/diy_tit4_zph.png' },
  { label: '兼职', to: '/parts', icon: '/legacy/h5/images/Part-time_management.png' },
  { label: '资讯', to: '/articles', icon: '/legacy/h5/images/news.png' },
  { label: '地图', to: '/map', icon: '/legacy/h5/images/map_nav.png' },
  { label: '测评', to: '/eval', icon: '/legacy/h5/images/icon_question.png' },
]

export function mediaUrl(path?: string | null, fallback = ''): string {
  if (!path) return fallback
  const p = String(path).trim()
  if (!p) return fallback
  if (/^https?:\/\//i.test(p) || p.startsWith('//') || p.startsWith('/')) return p
  return `/${p.replace(/^(\.\/)+/, '')}`
}

export function mapNavUrl(url?: string | null): string {
  if (!url) return '/'
  const raw = String(url).trim()
  if (!raw) return '/'
  if (raw.startsWith('/')) {
    const path = raw.split('?')[0]
    if (!path.includes('index.php')) return path || '/'
  }
  try {
    const u = raw.includes('://') ? new URL(raw) : new URL(raw, 'http://local.invalid/')
    const q = u.searchParams
    const m = q.get('m') || ''
    const c = q.get('c') || ''
    if (m === 'member' || raw.includes('/member')) return '/user'
    if (m === 'wap' && c && MODULE_PATH[c]) return MODULE_PATH[c]
    if (m && MODULE_PATH[m]) return MODULE_PATH[m]
  } catch {
    /* ignore */
  }
  const m = raw.match(/[?&]m=(\w+)/)
  if (m?.[1] && MODULE_PATH[m[1]]) return MODULE_PATH[m[1]]
  return '/'
}

export function listFailMsg(err: unknown, rateLimit: string, fallback: string): string {
  if (!err) return ''
  const e = err as { data?: { key?: string; msg?: string }; message?: string; statusCode?: number; status?: number }
  const key = e.data?.key || ''
  const status = e.statusCode || e.status || 0
  if (key === 'rate_limit' || status === 429) return e.data?.msg || rateLimit
  return e.data?.msg || e.message || fallback
}

export function formatSalary(job: JobLike, negotiable = '面议'): string {
  const min = Number(job.min_salary ?? job.minsalary ?? 0)
  const max = Number(job.max_salary ?? job.maxsalary ?? 0)
  if (!min && !max) return negotiable
  // PHP `salaryUnit`（resume_salarytype=1）：min-max + 元
  if (min && max) return `${min}-${max}元`
  return `${min || max}元`
}

export function formatUnixDate(ts?: number | string | null): string {
  const n = Number(ts || 0)
  if (!n) return ''
  const d = new Date(n * 1000)
  if (Number.isNaN(d.getTime())) return ''
  const y = d.getFullYear()
  const m = String(d.getMonth() + 1).padStart(2, '0')
  const day = String(d.getDate()).padStart(2, '0')
  return `${y}-${m}-${day}`
}

export function companyName(c: CompanyLike): string {
  return String(c.name || c.shortname || '企业')
}

export function catTree(list: CatNode[], limit = 11): CatNode[] {
  const children = new Map<number, CatNode[]>()
  for (const c of list) {
    const arr = children.get(c.parent_id) || []
    arr.push(c)
    children.set(c.parent_id, arr)
  }
  const roots = (children.get(0) || []).slice(0, limit)
  return roots.map((r) => ({
    ...r,
    children: (children.get(r.id) || []).map((s) => ({
      ...s,
      children: children.get(s.id) || [],
    })),
  }))
}

export const PLACEHOLDER_LOGO = '/legacy/pc/images/lay-loding.png'
export const PLACEHOLDER_BANNER = '/legacy/pc/images/banner.png'
