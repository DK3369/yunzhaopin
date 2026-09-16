import { isMemberModuleOn } from '../utils/site'
import { useSiteSettings } from './useSiteSettings'

export type MemberNavLink = {
  to: string
  label: string
  icon: string
  badge?: 'interview' | 'resume'
}

export type MemberMoreGroup = {
  title: string
  items: Array<{ to: string; label: string }>
}

export function useMemberNav() {
  const { t } = useI18n()
  const { settings } = useSiteSettings()
  const on = (to: string) => isMemberModuleOn(settings.value, to)

  const userMain = computed<MemberNavLink[]>(() => [
    { to: '/user', label: t('member_user_00183'), icon: 'left_navicon_i1' },
    { to: '/user/resume', label: t('wap_user_00204'), icon: 'left_navicon_i2' },
    { to: '/user/interviews', label: t('wap_user_00216'), icon: 'left_navicon_i4', badge: 'interview' },
    { to: '/user/applications', label: t('wap_user_00270'), icon: 'left_navicon_i5' },
    { to: '/user/views', label: t('wap_com_00407'), icon: 'left_navicon_i6' },
    { to: '/user/favorites', label: t('member_user_00103'), icon: 'left_navicon_i7' },
    { to: '/user/looks', label: t('wap_user_00275'), icon: 'left_navicon_i8' },
  ])

  const userMoreTitle = computed(() => t('member_user_00186'))
  const userMore = computed<MemberMoreGroup[]>(() => {
    const groups: MemberMoreGroup[] = [
      {
        title: t('wap_com_00105'),
        items: [
          { to: '/user/outbox', label: t('member_user_00188') },
          { to: '/user/resume-tpls', label: t('member_user_00189') },
        ],
      },
      {
        title: t('member_user_00187'),
        items: [
          { to: '/user/parts', label: t('member_user_00185') },
          { to: '/user/searches', label: t('member_user_00108') },
        ],
      },
      {
        title: t('wap_user_00213'),
        items: [
          { to: '/user/integral', label: t('wap_user_00008') },
          { to: '/user/invite', label: t('wap_user_00253') },
          { to: '/user/finance', label: t('member_user_00190') },
        ],
      },
    ]
    if (on('/questions')) {
      groups.push({
        title: t('member_user_00184'),
        items: [{ to: '/questions', label: t('wap_00331') }],
      })
    }
    return groups
      .map((g) => ({ ...g, items: g.items.filter((it) => on(it.to)) }))
      .filter((g) => g.items.length)
  })

  const comMain = computed<MemberNavLink[]>(() =>
    [
      { to: '/com', label: t('member_com_00290'), icon: 'com_left_icon1' },
      { to: '/com/jobs', label: t('wap_com_00106'), icon: 'com_left_icon2' },
      { to: '/com/applications', label: t('wap_com_00105'), icon: 'com_left_icon4', badge: 'resume' },
      { to: '/com/interviews', label: t('member_com_00213'), icon: 'com_left_icon10' },
      { to: '/com/member-right', label: t('wap_com_00097'), icon: 'com_left_icon7' },
      { to: '/com/talent', label: t('member_com_00597'), icon: 'com_left_icon3' },
      { to: '/com/fairs', label: t('member_com_00293'), icon: 'com_left_icon12' },
      { to: '/com/profile', label: t('wap_com_00096'), icon: 'com_left_icon8' },
      { to: '/com/binding', label: t('member_user_00059'), icon: 'com_left_icon11' },
    ].filter((it) => on(it.to)),
  )

  const comMoreTitle = computed(() => t('member_com_00292'))
  const comMore = computed<MemberMoreGroup[]>(() => {
    const groups: MemberMoreGroup[] = [
      {
        title: t('common_02156'),
        items: [
          { to: '/com/record', label: t('member_com_00555') },
          { to: '/com/finder', label: t('member_com_00086') },
        ],
      },
      {
        title: t('wap_user_00196'),
        items: [
          { to: '/com/specials', label: t('wap_com_00310') },
          { to: '/com/report', label: t('member_com_00291') },
          { to: '/com/parts', label: t('wap_user_00271') },
          { to: '/com/recommend', label: t('wap_user_00211') },
          { to: '/com/hrs', label: t('ui.hr') },
          { to: '/com/broadcasts', label: t('ui.broadcasts') },
          { to: '/com/warnings', label: t('ui.warnings') },
          { to: '/com/stats', label: t('admin_tool_00224') },
          { to: '/questions', label: t('wap_01141') },
        ],
      },
      {
        title: t('wap_user_00213'),
        items: [
          { to: '/com/added', label: t('wap_com_00393') },
          { to: '/com/integral', label: t('wap_user_00008') },
          { to: '/com/pay', label: t('common_01946') },
          { to: '/com/orders', label: t('common_02029') },
        ],
      },
      {
        title: t('wap_com_00096'),
        items: [
          { to: '/com/gallery', label: t('wap_user_00157') },
          { to: '/com/news', label: t('ui.com_news') },
          { to: '/com/products', label: t('ui.com_products') },
          { to: '/com/banners', label: t('ui.com_banner') },
          { to: '/com/addresses', label: t('ui.map_addr') },
          { to: '/com/map', label: t('ui.map_addr') },
          { to: '/com/tpls', label: t('ui.com_tpl') },
          { to: '/com/interview-tpls', label: t('ui.interview_tpl') },
          { to: '/com/messages', label: t('common.message') },
          { to: '/com/job-messages', label: t('member_user_00115') },
          { to: '/com/follows', label: t('wap_01142') },
        ],
      },
    ]
    return groups
      .map((g) => ({ ...g, items: g.items.filter((it) => on(it.to)) }))
      .filter((g) => g.items.length)
  })

  function flatten(main: MemberNavLink[], more: MemberMoreGroup[], extras: Array<{ to: string; label: string }>) {
    const out: Array<{ to: string; label: string }> = main.map(({ to, label }) => ({ to, label }))
    for (const g of more) out.push(...g.items)
    out.push(...extras)
    return out.filter((it) => on(it.to))
  }

  const userItems = computed(() =>
    flatten(userMain.value, userMore.value, [
      { to: '/user/follows', label: t('wap_01142') },
      { to: '/user/recommend', label: t('wap_user_00211') },
      { to: '/user/messages', label: t('common.message') },
      { to: '/user/consults', label: t('member_user_00115') },
      { to: '/user/eval-logs', label: t('wap_00194') },
      { to: '/user/expects', label: t('wap_00460') },
      { to: '/user/password', label: t('member_user_00226') },
      { to: '/user/privacy', label: t('wap_user_00215') },
      { to: '/user/blacklist', label: t('member_user_00044') },
      { to: '/user/inbox', label: t('admin_user_00263') },
      { to: '/user/reports', label: t('ui.my_reports') },
      { to: '/user/account', label: t('wap_user_00338') },
      { to: '/user/binding', label: t('wap_00389') },
      { to: '/user/ident', label: t('wap_user_00340') },
      { to: '/user/set', label: t('wap_user_00214') },
      { to: '/user/pay', label: t('common_01946') },
      { to: '/advice', label: t('wap_user_00203') },
    ]),
  )

  const comItems = computed(() =>
    flatten(comMain.value, comMore.value, [
      { to: '/com/jobs/new', label: t('wap_00322') },
      { to: '/com/looks', label: t('member_com_00007') },
      { to: '/com/views', label: t('member_com_00006') },
      { to: '/com/fans', label: t('wap_com_00407') },
      { to: '/com/downloads', label: t('wap_00451') },
      { to: '/com/cert', label: t('member_user_00235') },
      { to: '/com/binding', label: t('member_user_00059') },
      { to: '/com/password', label: t('member_user_00226') },
      { to: '/advice', label: t('wap_user_00203') },
    ]),
  )

  return {
    userMain,
    userMore,
    userMoreTitle,
    userItems,
    comMain,
    comMore,
    comMoreTitle,
    comItems,
  }
}
