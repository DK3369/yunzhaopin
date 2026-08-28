export function useMemberNav() {
  const { t } = useI18n()
  const userItems = computed(() => [
    { to: '/user', label: t('member_user_00183') },
    { to: '/user/resume', label: t('wap_user_00204') },
    { to: '/user/interviews', label: t('wap_user_00216') },
    { to: '/user/applications', label: t('wap_user_00270') },
    { to: '/user/views', label: t('wap_user_00276') },
    { to: '/user/favorites', label: t('member_user_00103') },
    { to: '/user/follows', label: t('wap_01142') },
    { to: '/user/messages', label: t('common.message') },
    { to: '/user/expects', label: t('wap_00460') },
    { to: '/user/searches', label: t('member_user_00108') },
    { to: '/user/resume-tpls', label: t('member_user_00189') },
    { to: '/user/password', label: t('member_user_00226') },
    { to: '/user/privacy', label: t('wap_user_00215') },
    { to: '/user/binding', label: t('ui.binding') },
    { to: '/user/integral', label: t('wap_user_00008') },
    { to: '/user/pay', label: t('common_01946') },
    { to: '/user/outbox', label: t('member_user_00188') },
    { to: '/user/parts', label: t('ui.part') },
    { to: '/advice', label: t('wap_user_00203') },
  ])
  const comItems = computed(() => [
    { to: '/com', label: t('member_com_00290') },
    { to: '/com/jobs', label: t('wap_com_00106') },
    { to: '/com/jobs/new', label: t('common.publish_job') },
    { to: '/com/applications', label: t('ui.received_resumes') },
    { to: '/com/talent', label: t('ui.talent_pool') },
    { to: '/com/cert', label: t('member_user_00235') },
    { to: '/com/messages', label: t('common.message') },
    { to: '/com/downloads', label: t('wap_00451') },
    { to: '/com/follows', label: t('wap_01142') },
    { to: '/com/fairs', label: t('member_com_00293') },
    { to: '/com/interviews', label: t('wap_user_00216') },
    { to: '/com/profile', label: t('wap_com_00096') },
    { to: '/com/orders', label: t('common_02029') },
    { to: '/com/pay', label: t('common_01946') },
    { to: '/com/stats', label: t('ui.stats') },
    { to: '/com/password', label: t('member_user_00226') },
    { to: '/advice', label: t('wap_user_00203') },
  ])
  return { userItems, comItems }
}
