export type NavItem = {
  id?: number
  label: string
  url?: string
  to: string
  icon?: string
  icon_n?: string
  parent_id?: number
  sort?: number
  /** PHP `phpyun_navigation.config` — module key used by 模块设置. */
  config?: string
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
  pr_n?: string
  mun_n?: string
  job_city_three?: string
  newtime?: boolean
  is_urgent?: boolean
  is_rec?: boolean
  lastupdate?: number
  lastupdate_n?: string
  welfare?: string[] | string
  welfare_n?: string[] | string
  yyzz_status?: number
  fact_status?: number
  number_n?: string
  age_n?: string
  sex_n?: string
  is_favorited?: boolean
  is_applied?: boolean
  istop?: boolean
  distance_km?: number
  wxurl?: string | null
  purl?: string | null
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
  hot_pic?: string | null
  hot_pic_n?: string | null
  hits?: number
  job_num?: number
  yyzz_status?: number
  fact_status?: number
  rec?: number
  ant_num?: number
  isatn?: number
  welfare_n?: string[] | string
  rating?: number
  rating_name?: string | null
  open_jobs?: Array<{ id: number; name: string }>
}

const MODULE_PATH: Record<string, string> = {
  job: '/jobs',
  resume: '/resumes',
  company: '/companies',
  article: '/articles',
  news: '/articles',
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

/** Reverse of MODULE_PATH for `sy_{module}_web`. Home / custom links have no key. */
const PATH_TO_MODULE: Record<string, string> = {
  '/jobs': 'job',
  '/resumes': 'resume',
  '/companies': 'company',
  '/articles': 'article',
  '/fairs': 'zph',
  '/announcements': 'announcement',
  '/tiny': 'tiny',
  '/once': 'once',
  '/parts': 'part',
  '/questions': 'ask',
  '/map': 'map',
  '/eval': 'evaluate',
  '/advice': 'advice',
  '/redeem': 'redeem',
  '/hr': 'hr',
  '/specials': 'special',
  '/gongzhao': 'gongzhao',
}

/**
 * PHP 模块开关：`sy_{m}_web == 2` 关闭。后台保存模块时会把对应导航 `display` 打成 0，
 * 但 `config` 列经常是空的，所以前台再按 URL 推断一次。
 */
export function isNavModuleOn(
  settings: Record<string, string>,
  to: string,
  config?: string | null,
): boolean {
  if (!to || to === '/') return true
  const key = String(config || '').trim() || PATH_TO_MODULE[to] || ''
  if (!key) return true
  return String(settings[`sy_${key}_web`] || '') !== '2'
}

export function modulePathFor(path: string): string {
  const keys = Object.keys(PATH_TO_MODULE).sort((a, b) => b.length - a.length)
  return keys.find((k) => path === k || path.startsWith(`${k}/`)) || ''
}

export function isPathModuleOn(settings: Record<string, string>, path: string): boolean {
  const to = modulePathFor(path)
  if (!to) return true
  return isNavModuleOn(settings, to)
}

export function errKey(err: unknown): string {
  if (!err || typeof err !== 'object') return ''
  const e = err as { key?: string; data?: { key?: string } }
  return String(e.key || e.data?.key || '')
}

export function isUnauthErr(err: unknown): boolean {
  const k = errKey(err)
  return k === 'unauth' || k === 'unauthenticated'
}

export const DEFAULT_NAV: NavItem[] = [
  { label: '', to: '/' },
  { label: '', to: '/jobs' },
  { label: '', to: '/resumes' },
  { label: '', to: '/companies' },
  { label: '', to: '/fairs' },
  { label: '', to: '/articles' },
]

export const DEFAULT_H5_NAV: NavItem[] = [
  { label: '', to: '/jobs', icon: '/legacy/h5/images/manage_full-time.png' },
  { label: '', to: '/companies', icon: '/legacy/h5/images/company.png' },
  { label: '', to: '/resumes', icon: '/legacy/h5/images/Please_resume.png' },
  { label: '', to: '/fairs', icon: '/legacy/h5/images/diy_tit4_zph.png' },
  { label: '', to: '/parts', icon: '/legacy/h5/images/Part-time_management.png' },
  { label: '', to: '/articles', icon: '/legacy/h5/images/news.png' },
  { label: '', to: '/map', icon: '/legacy/h5/images/map_nav.png' },
  { label: '', to: '/eval', icon: '/legacy/h5/images/icon_question.png' },
]

export function mediaUrl(path?: string | null, fallback = ''): string {
  if (!path) return fallback
  const p = String(path).trim()
  if (!p) return fallback
  if (/^https?:\/\//i.test(p) || p.startsWith('//') || p.startsWith('/')) return p
  return `/${p.replace(/^(\.\/)+/, '')}`
}

function modulePath(name?: string | null): string | undefined {
  if (!name) return undefined
  const key = name.replace(/^\.\//, '').replace(/\/+$/, '').replace(/\.(html?|php)$/i, '').toLowerCase()
  if (!key) return '/'
  return MODULE_PATH[key]
}

/** PHP `phpyun_navigation.url` is often a relative module path (`job/`, `evaluate`). */
export function mapNavUrl(url?: string | null): string {
  if (!url) return '/'
  const raw = String(url).trim().replace(/^\.\//, '')
  if (!raw) return '/'

  if (raw.startsWith('/')) {
    const pathOnly = raw.split('?')[0]
    if (pathOnly.startsWith('/about/')) {
      const code = pathOnly.replace(/^\/about\//, '').replace(/\.html?$/i, '')
      return code ? `/pages/${code}` : '/'
    }
    if (!pathOnly.toLowerCase().includes('index.php')) {
      const first = pathOnly.replace(/^\//, '').split('/')[0]
      return modulePath(first) || pathOnly || '/'
    }
  }

  try {
    const u = raw.includes('://') ? new URL(raw) : new URL(raw, 'http://local.invalid/')
    const q = u.searchParams
    const m = q.get('m') || ''
    const c = q.get('c') || ''
    if (m === 'member' || raw.includes('/member')) return '/user'
    if (m === 'wap' && c && MODULE_PATH[c]) return MODULE_PATH[c]
    if (m && MODULE_PATH[m]) return MODULE_PATH[m]
    if (!m && c && MODULE_PATH[c]) return MODULE_PATH[c]
    const segs = u.pathname.split('/').filter(Boolean)
    const fromPath = modulePath(segs[0])
    if (fromPath) return fromPath
  } catch {
    /* ignore */
  }

  const first = raw.split(/[/?#]/).filter(Boolean)[0]
  const fromRel = modulePath(first)
  if (fromRel) return fromRel
  const m = raw.match(/[?&]m=(\w+)/)
  if (m?.[1] && MODULE_PATH[m[1]]) return MODULE_PATH[m[1]]
  return '/'
}

/** PHP footer `{yun:}desc{/yun}`: about/*.html is CMS detail; other urls follow nav mapping. */
export function descHref(item: { id: number; link_url?: string | null }): string {
  const u = String(item.link_url || '').trim()
  if (!u) return `/get/${item.id}`
  if (/^https?:\/\//i.test(u) || u.startsWith('//')) return u
  const path = u.startsWith('/') ? u : `/${u}`
  if (/^\/about\//i.test(path) || path.toLowerCase().includes('/about/')) return `/get/${item.id}`
  return mapNavUrl(path)
}

export function listFailMsg(err: unknown, rateLimit: string, fallback: string): string {
  if (!err) return ''
  const e = err as { data?: { key?: string; msg?: string }; message?: string; statusCode?: number; status?: number }
  const key = e.data?.key || ''
  const status = e.statusCode || e.status || 0
  if (key === 'rate_limit' || status === 429) return e.data?.msg || rateLimit
  return e.data?.msg || e.message || fallback
}

/** PHP `salaryUnit($minsalary, $maxsalary)` — `resume_salarytype` 1=元 / 2=千 / 3=K / 4=k. */
export function formatSalary(
  job: JobLike,
  negotiable = '',
  salaryType = 1,
  plus = '',
  units?: { yuan?: string; qian?: string },
): string {
  const min = Number(job.min_salary ?? job.minsalary ?? 0)
  const max = Number(job.max_salary ?? job.maxsalary ?? 0)
  if (!min && !max) return negotiable
  const type = Number(salaryType) || 1
  const yuan = units?.yuan || '元'
  const qian = units?.qian || '千'
  const unit = type === 2 ? qian : type === 3 ? 'K' : type === 4 ? 'k' : yuan
  const n = (v: number) => (type === 1 ? String(v) : String(Math.floor((v / 1000) * 10) / 10))
  if (min && max) {
    if (max < 2000) return type === 1 ? `2000${unit}${plus}` : `2${unit}${plus}`
    return `${n(min)}-${n(max)}${unit}`
  }
  return `${n(min || max)}${unit}`
}

/** Skip mixing CJK dict labels with Latin suffixes (e.g. 不限 + experience). */
export function dictReqLabel(name: string, suffix = ''): string {
  const n = String(name || '').trim()
  const s = String(suffix || '')
  if (!n || !s) return n
  const nCjk = /[\u4e00-\u9fff]/.test(n)
  const sCjk = /[\u4e00-\u9fff]/.test(s)
  if (nCjk !== sCjk) return n
  if (!nCjk && !sCjk) return `${n} ${s.trim()}`
  return `${n}${s}`
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

/** PHP `smarty_internal_compile_joblist` `$list.time` + `lastupdateStyle`. */
export function formatJobListTime(
  ts?: number | string | null,
  labels?: { yesterday: string; hoursAgo: string; minutesAgo: string },
  fallback = '',
): { text: string; hot: boolean } {
  const n = Number(ts || 0)
  if (!n) return { text: fallback, hot: false }
  const ms = n * 1000
  const now = new Date()
  const beginToday = new Date(now.getFullYear(), now.getMonth(), now.getDate()).getTime()
  const beginYesterday = beginToday - 86_400_000
  if (ms >= beginYesterday && ms < beginToday) {
    return { text: labels?.yesterday || fallback, hot: false }
  }
  if (ms >= beginToday) {
    const hours = Math.floor((Date.now() - ms) / 3_600_000)
    if (hours >= 1) return { text: `${hours}${labels?.hoursAgo || ''}`, hot: true }
    const mins = Math.max(1, Math.ceil((Date.now() - ms) / 60_000))
    return { text: `${mins}${labels?.minutesAgo || ''}`, hot: true }
  }
  return { text: formatUnixDate(n) || fallback, hot: false }
}

export function companyName(c: CompanyLike, fallback = ''): string {
  return String(c.name || c.shortname || fallback)
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
