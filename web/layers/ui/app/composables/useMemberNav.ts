export function useMemberNav() {
  const { t } = useI18n()
  const userItems = computed(() => [
    { to: '/user', label: t('member_user_00183') },
    { to: '/user/resume', label: t('wap_user_00204') },
    { to: '/user/interviews', label: t('wap_user_00216') },
    { to: '/user/applications', label: t('wap_user_00270') },
    { to: '/user/views', label: t('wap_com_00407') },
    { to: '/user/favorites', label: t('member_user_00103') },
    { to: '/user/follows', label: t('wap_00385') },
    { to: '/user/messages', label: t('common.message') },
    { to: '/user/expects', label: t('wap_user_00275') },
    { to: '/user/searches', label: t('common.search') },
    { to: '/user/resume-tpls', label: t('wap_00328') },
    { to: '/user/password', label: t('common_02389') },
    { to: '/user/privacy', label: t('wap_user_00205') },
    { to: '/user/binding', label: t('member_user_00150') },
    { to: '/user/integral', label: t('default_00101') },
    { to: '/user/pay', label: t('wap_00375') },
    { to: '/user/outbox', label: t('wap_00574') },
    { to: '/user/parts', label: t('ui.part') },
    { to: '/advice', label: t('common.site_notice') },
  ])
  const comItems = computed(() => [
    { to: '/com', label: t('member_com_00290') },
    { to: '/com/jobs', label: t('wap_com_00106') },
    { to: '/com/jobs/new', label: t('common.publish_job') },
    { to: '/com/applications', label: t('wap_com_00105') },
    { to: '/com/talent', label: t('wap_00576') },
    { to: '/com/cert', label: t('activate_00005') },
    { to: '/com/messages', label: t('common.message') },
    { to: '/com/downloads', label: t('wap_com_00235') },
    { to: '/com/follows', label: t('wap_00385') },
    { to: '/com/fairs', label: t('wap_00223') },
    { to: '/com/interviews', label: t('wap_user_00216') },
    { to: '/com/profile', label: t('member_com_00378') },
    { to: '/com/orders', label: t('wap_00375') },
    { to: '/com/pay', label: t('wap_00375') },
    { to: '/com/stats', label: t('admin_00031') },
    { to: '/com/password', label: t('common_02389') },
    { to: '/advice', label: t('common.site_notice') },
  ])
  return { userItems, comItems }
}
