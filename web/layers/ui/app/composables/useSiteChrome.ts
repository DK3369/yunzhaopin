import {
  DEFAULT_H5_NAV,
  mapNavUrl,
  mediaUrl,
  type NavItem,
} from '../utils/site'

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

  const siteName = computed(
    () => settings.value.sy_webname || String(runtime.public.siteName || t('common.job')),
  )
  const phone = computed(() => settings.value.sy_freewebtel || '')
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
        .get<Array<{ id: number; label: string; url: string; icon?: string; icon_n?: string }>>(
          '/v1/wap/nav',
          { position: '1' },
        )
        .catch(() => []),
    { default: () => [] },
  )

  const NAV_LABEL_KEY: Record<string, string> = {
    '/': 'common.home',
    '/jobs': 'default_00246',
    '/resumes': 'common.resume',
    '/companies': 'common.company',
    '/fairs': 'wap_00223',
    '/articles': 'common.article',
    '/parts': 'ui.part',
    '/map': 'ui.map',
    '/eval': 'ui.eval',
  }

  function labelForNav(item: { to: string; label: string }) {
    const key = NAV_LABEL_KEY[item.to]
    if (key) return t(key)
    if (item.label && te(item.label)) return t(item.label)
    return item.label
  }

  const nav = computed<NavItem[]>(() => {
    const list = (navRaw.value || [])
      .map((n) => ({
        id: n.id,
        label: n.label,
        url: n.url,
        to: mapNavUrl(n.url),
        icon: n.icon_n || n.icon,
      }))
      .filter((n) => n.label)
    const rows = list.length
      ? list
      : [
          { label: t('common.home'), to: '/' },
          { label: t('default_00246'), to: '/jobs' },
          { label: t('common.resume'), to: '/resumes' },
          { label: t('common.company'), to: '/companies' },
          { label: t('wap_00223'), to: '/fairs' },
          { label: t('common.article'), to: '/articles' },
        ]
    return rows.map((n) => ({ ...n, label: labelForNav(n) }))
  })

  const h5Nav = computed<NavItem[]>(() => {
    const withIcon = nav.value.filter((n) => n.icon && n.to !== '/')
    const rows = withIcon.length >= 4 ? withIcon.slice(0, 8) : DEFAULT_H5_NAV
    return rows.map((n) => ({ ...n, label: labelForNav(n) }))
  })

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

  const h5Title = computed(() => {
    const titles: Record<string, string> = {
      '/': t('common.home'),
      '/jobs': t('common.job'),
      '/companies': t('common.company'),
      '/resumes': t('common.resume'),
      '/articles': t('common.article'),
      '/fairs': t('wap_00223'),
      '/parts': t('wap_00223'),
      '/questions': t('common.more'),
      '/announcements': t('common.site_notice'),
      '/login': t('common.login'),
      '/register': t('common.register'),
      '/forgetpw': t('common.search'),
      '/search': t('common.search'),
      '/eval': t('common.more'),
      '/map': t('wap_00223'),
      '/advice': t('common.site_notice'),
      '/user': t('common.user_center'),
      '/com': t('common.user_center'),
    }
    if (titles[route.path]) return titles[route.path]
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
    h5Nav,
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
