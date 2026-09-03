import {
  DEFAULT_H5_NAV,
  descHref,
  isNavModuleOn,
  mapNavUrl,
  mediaUrl,
  type NavItem,
} from '../utils/site'
import { useMemberNav } from './useMemberNav'

type SettingRow = { key: string; value: string }
type Me = { uid: number; username: string; usertype: number }

export function useSiteChrome() {
  const api = useApi()
  const route = useRoute()
  const runtime = useRuntimeConfig()
  const { t, te } = useI18n()

  const { data: settingRows } = useAsyncData(
    'site-settings',
    () =>
      api
        .post<SettingRow[]>('/v1/wap/site/settings', {})
        .catch(() => [] as SettingRow[]),
    { default: () => [] as SettingRow[] },
  )

  const settings = computed(() => {
    const m: Record<string, string> = {}
    for (const row of settingRows.value || []) m[row.key] = row.value
    return m
  })

  const siteName = computed(() => {
    const fromSetting = String(settings.value.sy_webname || '').trim()
    if (fromSetting) return fromSetting
    const fromEnv = String(runtime.public.siteName || '').trim()
    if (fromEnv && fromEnv !== '招聘') return fromEnv
    return ''
  })
  const phone = computed(() => String(settings.value.sy_freewebtel || '').trim())
  const worktime = computed(() => settings.value.sy_worktime || '')
  const copyright = computed(() => settings.value.sy_webcopyright || '')
  const record = computed(() => settings.value.sy_webrecord || '')
  const email = computed(() => settings.value.sy_webemail || '')
  const address = computed(() => settings.value.sy_webadd || '')
  const logoPc = computed(() => mediaUrl(settings.value.sy_logo))
  const logoH5 = computed(() => mediaUrl(settings.value.sy_wap_logo || settings.value.sy_logo))

  const { data: navRaw } = useAsyncData(
    'site-nav-1',
    () =>
      api
        .get<
          Array<{
            id: number
            label: string
            url: string
            icon?: string
            icon_n?: string
            parent_id?: number
            sort?: number
            config?: string
          }>
        >('/v1/wap/nav', { position: '1' })
        .catch(() => []),
    { default: () => [] },
  )

  const NAV_LABEL_KEY: Record<string, string> = {
    '/': 'common.home',
    '/jobs': 'default_00246',
    '/resumes': 'default_00312',
    '/companies': 'default_00114',
    '/fairs': 'member_com_00293',
    '/articles': 'common.article',
    '/parts': 'ui.part',
    '/map': 'ui.map',
    '/eval': 'ui.eval',
    '/questions': 'ui.qa',
    '/announcements': 'ui.announcements',
    '/tiny': 'ui.tiny',
    '/once': 'ui.once',
    '/hr': 'ui.hr',
    '/redeem': 'ui.redeem',
    '/specials': 'ui.specials',
    '/gongzhao': 'ui.gongzhao',
    '/advice': 'wap_user_00203',
  }

  function labelForNav(item: { to: string; label: string }) {
    const key = NAV_LABEL_KEY[item.to]
    if (key) return t(key)
    if (item.label && te(item.label)) return t(item.label)
    return item.label
  }

  function mappedRows() {
    return (navRaw.value || [])
      .map((n) => ({
        id: n.id,
        label: n.label,
        url: n.url,
        to: mapNavUrl(n.url),
        icon: n.icon_n || n.icon,
        parent_id: Number(n.parent_id || 0),
        sort: Number(n.sort || 0),
        config: n.config || '',
      }))
      .filter((n) => n.label)
      .filter((n) => isNavModuleOn(settings.value, n.to, n.config))
      .sort((a, b) => a.sort - b.sort || (a.id || 0) - (b.id || 0))
  }

  const nav = computed<NavItem[]>(() => {
    const list = mappedRows().filter((n) => n.parent_id === 1)
    const rows = list.length
      ? list
      : [
          { label: t('common.home'), to: '/' },
          { label: t('default_00246'), to: '/jobs' },
          { label: t('default_00312'), to: '/resumes' },
          { label: t('default_00114'), to: '/companies' },
          { label: t('member_com_00293'), to: '/fairs' },
          { label: t('common.article'), to: '/articles' },
        ].filter((n) => isNavModuleOn(settings.value, n.to))
    return rows.map((n) => ({ ...n, label: labelForNav(n) }))
  })

  const appNav = computed<NavItem[]>(() =>
    mappedRows()
      .filter((n) => n.parent_id === 11)
      .map((n) => ({ ...n, label: labelForNav(n) })),
  )

  const h5Nav = computed<NavItem[]>(() => {
    const withIcon = mappedRows().filter((n) => n.parent_id === 26 && n.icon)
    const rows = withIcon.length >= 4 ? withIcon : DEFAULT_H5_NAV.filter((n) => isNavModuleOn(settings.value, n.to))
    return rows.map((n) => ({
      ...n,
      icon: mediaUrl(n.icon_n || n.icon, n.icon || ''),
      label: labelForNav(n),
    }))
  })

  type DescClass = { id: number; name: string }
  type DescRow = { id: number; class_id: number; name?: string; title: string; is_nav?: number; link_url?: string; is_type?: number }

  const { data: descClasses } = useAsyncData(
    'site-desc-classes',
    () => api.post<DescClass[]>('/v1/wap/descriptions/classes', {}).catch(() => [] as DescClass[]),
    { default: () => [] as DescClass[] },
  )
  const { data: descRows } = useAsyncData(
    'site-desc-rows',
    () =>
      api
        .post<{ list: DescRow[] }>('/v1/wap/descriptions', { page: 1, page_size: 80 })
        .catch(() => ({ list: [] as DescRow[] })),
    { default: () => ({ list: [] as DescRow[] }) },
  )

  const footerNav = computed(() => {
    const classes = descClasses.value || []
    const rows = descRows.value?.list || []
    return classes
      .map((c) => ({
        id: c.id,
        name: c.name,
        list: rows
          .filter((r) => r.class_id === c.id)
          .slice(0, 5)
          .map((r) => ({
            id: r.id,
            title: String(r.name || '').trim() || r.title,
            to: descHref(r),
          })),
      }))
      .filter((c) => c.list.length)
  })

  const { data: hotSearches } = useAsyncData(
    'site-hot-searches-job',
    () =>
      api
        .get<Array<{ keyword: string }>>('/v1/wap/hot-searches', { scope: 'job', limit: 6 })
        .catch(() => [] as Array<{ keyword: string }>),
    { default: () => [] as Array<{ keyword: string }> },
  )

  const wxQr = computed(() => mediaUrl(settings.value.sy_wx_qcode))
  const wapQr = computed(() => mediaUrl(settings.value.sy_wap_qcode))
  const perfor = computed(() => String(settings.value.sy_perfor || '').trim())
  const hrlicense = computed(() => String(settings.value.sy_hrlicense || '').trim())
  const secord = computed(() => String(settings.value.sy_websecord || '').trim())

  const { data: me } = useAsyncData(
    'auth-me',
    () => $fetch<Me>('/api/auth/me').catch(() => null),
    { default: () => null },
  )

  const isHome = computed(() => route.path === '/')
  const isAuth = computed(() =>
    ['/login', '/register', '/forgetpw'].includes(route.path),
  )
  const isMember = computed(
    () => route.path.startsWith('/user') || route.path.startsWith('/com'),
  )

  const { userItems, comItems } = useMemberNav()

  const h5Title = computed(() => {
    const titles: Record<string, string> = {
      '/': t('common.home'),
      '/jobs': t('common.job'),
      '/companies': t('common.company'),
      '/resumes': t('common.resume'),
      '/articles': t('common.article'),
      '/fairs': t('member_com_00293'),
      '/parts': t('ui.part'),
      '/questions': t('ui.qa'),
      '/announcements': t('ui.announcements'),
      '/login': t('common.login'),
      '/register': t('common.register'),
      '/forgetpw': t('wap_js_00123'),
      '/search': t('common.search'),
      '/eval': t('ui.eval'),
      '/map': t('ui.map'),
      '/advice': t('wap_user_00203'),
      '/tiny': t('ui.tiny'),
      '/once': t('ui.once'),
      '/hr': t('ui.hr'),
      '/redeem': t('ui.redeem'),
      '/specials': t('ui.specials'),
      '/gongzhao': t('ui.gongzhao'),
      '/user': t('common.user_center'),
      '/com': t('common.user_center'),
    }
    if (titles[route.path]) return titles[route.path]
    const member = [...userItems.value, ...comItems.value]
    const memberHit = member
      .filter((i) => i.to !== '/user' && i.to !== '/com' && (route.path === i.to || route.path.startsWith(`${i.to}/`)))
      .sort((a, b) => b.to.length - a.to.length)[0]
    if (memberHit) return memberHit.label
    const hit = Object.keys(titles)
      .filter((k) => k !== '/' && route.path.startsWith(k))
      .sort((a, b) => b.length - a.length)[0]
    return hit ? titles[hit] : siteName.value
  })

  const memberHome = computed(() => {
    if (!me.value) return '/login'
    return me.value.usertype === 2 ? '/com' : '/user'
  })

  async function logout() {
    await $fetch('/api/auth/logout', { method: 'POST' }).catch(() => undefined)
    await navigateTo('/login')
  }

  function navActive(to: string) {
    if (to === '/') return route.path === '/'
    return route.path === to || route.path.startsWith(`${to}/`)
  }

  return {
    settings,
    siteName,
    phone,
    worktime,
    copyright,
    record,
    email,
    address,
    logoPc,
    logoH5,
    nav,
    appNav,
    h5Nav,
    footerNav,
    hotSearches,
    wxQr,
    wapQr,
    perfor,
    hrlicense,
    secord,
    me,
    isHome,
    isAuth,
    isMember,
    h5Title,
    memberHome,
    logout,
    navActive,
  }
}
