import {
  DEFAULT_H5_NAV,
  descHref,
  isAuthPath,
  isComMemberPath,
  isMemberPath,
  isNavModuleOn,
  isPublicMapTo,
  mapNavUrl,
  mediaUrl,
  type NavItem,
} from '../utils/site'
import { useMemberNav } from './useMemberNav'
import { useSiteSettings } from './useSiteSettings'

export function useSiteChrome() {
  const route = useRoute()
  const runtime = useRuntimeConfig()
  const { t, te } = useI18n()
  const { syWebname, syWebtitle, syLogo } = useSubSite()
  const { data: boot } = useSiteBoot()
  const { settings } = useSiteSettings()

  const siteName = computed(() => {
    const fromSub = String(syWebname.value || '').trim()
    if (fromSub) return fromSub
    const fromSetting = String(settings.value.sy_webname || '').trim()
    if (fromSetting) return fromSetting
    const fromEnv = String(runtime.public.siteName || '').trim()
    if (fromEnv && fromEnv !== '招聘') return fromEnv
    return ''
  })
  const siteTitle = computed(() => String(syWebtitle.value || '').trim() || String(settings.value.sy_webtitle || '').trim())
  const phone = computed(() => String(settings.value.sy_freewebtel || '').trim())
  const worktime = computed(() => settings.value.sy_worktime || '')
  const copyright = computed(() => settings.value.sy_webcopyright || '')
  const record = computed(() => settings.value.sy_webrecord || '')
  const email = computed(() => settings.value.sy_webemail || '')
  const address = computed(() => settings.value.sy_webadd || '')
  const logoPc = computed(() => mediaUrl(String(syLogo.value || '').trim() || settings.value.sy_logo))
  const logoH5 = computed(() => mediaUrl(String(syLogo.value || '').trim() || settings.value.sy_logo || settings.value.sy_wap_logo))

  const navRaw = computed(() => boot.value?.nav || [])

  const NAV_LABEL_KEY: Record<string, string> = {
    '/': 'common.home',
    '/jobs': 'default_00246',
    '/resumes': 'default_00312',
    '/companies': 'default_00114',
    '/fairs': 'member_com_00293',
    '/articles': 'common.article',
    '/parts': 'wap_user_00220',
    '/map': 'wap_00315',
    '/eval': 'wap_00194',
    '/questions': 'wap_user_00223',
    '/announcements': 'wap_00221',
    '/tiny': 'wap_js_00066',
    '/once': 'wap_js_00130',
    '/hr': 'ui.hr',
    '/redeem': 'ui.redeem',
    '/specials': 'wap_com_00310',
    '/gongzhao': 'default_00134',
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
      .filter((n) => !isPublicMapTo(n.to))
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
    return rows.filter((n) => !isPublicMapTo(n.to)).map((n) => ({ ...n, label: labelForNav(n) }))
  })

  const appNav = computed<NavItem[]>(() =>
    mappedRows()
      .filter((n) => n.parent_id === 11)
      .map((n) => ({ ...n, label: labelForNav(n) })),
  )

  const h5Nav = computed<NavItem[]>(() => {
    const withIcon = mappedRows().filter((n) => n.parent_id === 26 && n.icon)
    const rows = withIcon.length >= 4 ? withIcon : DEFAULT_H5_NAV.filter((n) => isNavModuleOn(settings.value, n.to))
    return rows
      .filter((n) => !isPublicMapTo(n.to))
      .map((n) => ({
        ...n,
        icon: mediaUrl(n.icon_n || n.icon, n.icon || ''),
        label: labelForNav(n),
      }))
  })

  const descClasses = computed(() => boot.value?.footer_classes || [])
  const descRows = computed(() => ({ list: boot.value?.footer_pages || [] }))

  const FOOTER_NAME_KEY: Record<string, string> = {
    关于我们: 'wap_00218',
    'About Us': 'wap_00218',
    注册协议: 'wap_00219',
    'Registration Agreement': 'wap_00219',
    隐私政策: 'wap_00313',
    'Privacy Policy': 'wap_00313',
    联系我们: 'wap_00220',
    'Contact Us': 'wap_00220',
    产品与服务: 'common_01579',
    'Products and Services': 'common_01579',
    招聘会: 'member_com_00293',
    'Job Fair': 'member_com_00293',
    店铺招聘: 'wap_js_00130',
    'Store Recruitment': 'wap_js_00130',
    普工专区: 'default_00331',
    'General Worker Zone': 'default_00331',
    收费与推广: 'common_01615',
    'Paid Services & Promotion': 'common_01615',
    网站特色: 'common_01834',
    'Website Features': 'common_01834',
    排行榜: 'default_00156',
    Rankings: 'default_00156',
    求职测评: 'common_01801',
    'Job Assessment': 'common_01801',
    地图搜索: 'default_00139',
    'Map Search': 'default_00139',
    咨询反馈: 'common_01727',
    'Consultation Feedback': 'common_01727',
    客服中心: 'common_01745',
    'Customer Service Center': 'common_01745',
    常见问题: 'common_01760',
    'Common Questions': 'common_01760',
    友情链接: 'default_00256',
    'Friend Links': 'default_00256',
    积分兑换: 'common_06524',
    'Points Redemption': 'common_06524',
    人力资源许可证: 'common_01336',
    'HR License': 'common_01336',
    ICP经营许可证: 'default_00128',
    经营许可证: 'default_00128',
    'ICP License': 'default_00128',
    法律声明: 'ui.legal_notice',
    'Legal Notice': 'ui.legal_notice',
    经营资源: 'ui.business_resources',
    'Business Resources': 'ui.business_resources',
    品牌推广: 'ui.brand_promotion',
    'Brand Promotion': 'ui.brand_promotion',
    广告投放: 'ui.ad_placement',
    'Ad Placement': 'ui.ad_placement',
    收费标准: 'ui.fee_standards',
    'Fee Standards': 'ui.fee_standards',
    订阅服务: 'ui.subscribe_svc',
    'Subscription Service': 'ui.subscribe_svc',
    职场指南: 'ui.career_guide',
    'Career Guide': 'ui.career_guide',
    服务流程: 'ui.service_process',
    'Service Process': 'ui.service_process',
    银行帐户: 'ui.bank_account',
    银行账户: 'ui.bank_account',
    'Bank Account': 'ui.bank_account',
  }

  const FOOTER_PATH_KEY: Record<string, string> = {
    '/fairs': 'member_com_00293',
    '/once': 'wap_js_00130',
    '/tiny': 'default_00331',
    '/map': 'default_00139',
    '/eval': 'common_01801',
    '/links': 'default_00256',
    '/redeem': 'common_06524',
    '/top': 'default_00156',
    '/subscribe': 'ui.subscribe_svc',
  }

  function labelForFooter(name: string, to: string) {
    const byName = FOOTER_NAME_KEY[name]
    if (byName && te(byName)) return t(byName)
    const path = String(to || '').split('?')[0]
    const byPath = FOOTER_PATH_KEY[path]
    if (byPath && te(byPath)) return t(byPath)
    if (name && te(name)) return t(name)
    return name
  }

  const footerNav = computed(() => {
    const classes = descClasses.value || []
    const rows = descRows.value?.list || []
    return classes
      .map((c) => ({
        id: c.id,
        name: labelForFooter(c.name, ''),
        list: rows
          .filter((r) => r.class_id === c.id)
          .slice(0, 5)
          .map((r) => {
            const title = String(r.name || '').trim() || r.title
            const to = descHref(r)
            return {
              id: r.id,
              title: labelForFooter(title, to),
              to,
            }
          })
          .filter((item) => {
            if (isPublicMapTo(item.to)) return false
            const name = String(item.title || '').trim()
            return name !== 'Map Search' && name !== '地图搜索' && name !== 'Nearby'
          }),
      }))
      .filter((c) => c.list.length)
  })

  const wxQr = computed(() => mediaUrl(settings.value.sy_wx_qcode))
  const wapQr = computed(() => mediaUrl(settings.value.sy_wap_qcode))
  const perfor = computed(() => String(settings.value.sy_perfor || '').trim())
  const hrlicense = computed(() => String(settings.value.sy_hrlicense || '').trim())
  const secord = computed(() => String(settings.value.sy_websecord || '').trim())

  const { data: me, refresh: refreshMe } = useAuthMe()

  const isHome = computed(() => route.path === '/')
  const isAuth = computed(() => isAuthPath(route.path))
  const isMember = computed(() => {
    const p = route.path
    if (isMemberPath(p)) return true
    if (p === '/advice' && Number(me.value?.uid || 0) > 0) return true
    return false
  })

  const { userItems, comItems } = useMemberNav()

  const h5Title = computed(() => {
    const titles: Record<string, string> = {
      '/': t('common.home'),
      '/jobs': t('common.job'),
      '/companies': t('common.company'),
      '/resumes': t('common.resume'),
      '/articles': t('common.article'),
      '/fairs': t('member_com_00293'),
      '/parts': t('wap_user_00220'),
      '/questions': t('wap_user_00223'),
      '/announcements': t('wap_00221'),
      '/login': t('common.login'),
      '/register': t('common.register'),
      '/forgetpw': t('wap_js_00123'),
      '/search': t('common.search'),
      '/eval': t('wap_00194'),
      '/map': t('wap_00315'),
      '/advice': t('wap_user_00203'),
      '/tiny': t('wap_js_00066'),
      '/once': t('wap_js_00130'),
      '/hr': t('ui.hr'),
      '/redeem': t('ui.redeem'),
      '/specials': t('wap_com_00310'),
      '/gongzhao': t('default_00134'),
      '/links': t('default_00256'),
      '/top': t('default_00156'),
      '/subscribe': t('ui.subscribe_svc'),
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
      .filter((k) => k !== '/' && (route.path === k || route.path.startsWith(`${k}/`)))
      .sort((a, b) => b.length - a.length)[0]
    return hit ? titles[hit] : siteName.value
  })

  const memberHome = computed(() => {
    if (!me.value) return '/login'
    return me.value.usertype === 2 ? '/com' : '/user'
  })

  /** 会员壳 / 顶栏身份：以 /api/auth/me 为准，路径只作未登录时的兜底。 */
  const memberKind = computed<'user' | 'com'>(() => {
    const ut = Number(me.value?.usertype || 0)
    if (ut === 2) return 'com'
    if (ut === 1) return 'user'
    return isComMemberPath(route.path) ? 'com' : 'user'
  })

  async function logout() {
    await $fetch('/api/auth/logout', { method: 'POST' }).catch(() => undefined)
    await refreshMe()
    await navigateTo('/login')
  }

  function navActive(to: string) {
    if (to === '/') return route.path === '/'
    return route.path === to || route.path.startsWith(`${to}/`)
  }

  return {
    settings,
    siteName,
    siteTitle,
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
    wxQr,
    wapQr,
    perfor,
    hrlicense,
    secord,
    me,
    refreshMe,
    isHome,
    isAuth,
    isMember,
    h5Title,
    memberHome,
    memberKind,
    logout,
    navActive,
  }
}
