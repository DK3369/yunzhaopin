<script setup lang="ts">
import { formatUnixDate, isUnauthErr } from '~/utils/site'
import { runPackedTranslate } from '../../../../../layers/base/app/utils/packedText'

type Mission = {
  base_info?: boolean
  signin?: boolean
}

const api = useApi()
const { t, te, locale, getLocaleMessage } = useI18n()
function packed(text: unknown) {
  const loc = String(locale.value).toLowerCase().startsWith('en') ? 'en' : 'zh'
  const zhRoot = getLocaleMessage('zh') as Record<string, unknown>
  return runPackedTranslate(text, {
    locale: loc,
    lc: (k) => (te(k) ? String(t(k)) : k),
    zhRoot,
  })
}
const { settings } = useSiteChrome()
const priceName = computed(() => String(settings.value.integral_pricename || t('wap_user_00008')))
const { page, pageSize, inferTotal, go } = useMemberListPage()
const { data: bal, error, refresh: refreshBal } = await useAsyncData('user-finance-bal', () =>
  api.post('/v1/mcenter/integral/balance', {}),
)
const { data: pays } = await useAsyncData(
  () => `user-finance-pay-${page.value}`,
  () => api.post('/v1/mcenter/integral/consumes', { page: page.value, page_size: pageSize }).catch(() => ({ list: [] })),
)
const { data: signSt, refresh: refreshSign } = await useAsyncData('user-finance-sign', () =>
  api.post<{ signed_today?: boolean }>('/v1/mcenter/sign/status', {}).catch(() => null),
)
const { data: mission, refresh: refreshMission } = await useAsyncData('user-finance-mission', () =>
  api.post<Mission>('/v1/mcenter/integral/mission', {}).catch(() => null),
)
const msg = ref('')
const showLogs = ref(false)
function pts(key: string) {
  const n = String(settings.value[key] || '').trim()
  return n ? `+${n}${priceName.value}` : ''
}
const signed = computed(() => Boolean(mission.value?.signin || signSt.value?.signed_today))
const usedPoints = computed(() => {
  const list = (pays.value?.list || []) as { delta?: number }[]
  return list.reduce((s, r) => {
    const n = Number(r.delta || 0)
    return n < 0 ? s + Math.abs(n) : s
  }, 0)
})
async function sign() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/sign', {})
    msg.value = t('common.success')
    await refreshSign()
    await refreshMission()
    await refreshBal()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('wap_user_00213') })
const payTotal = computed(() => inferTotal(pays.value))
</script>

<template>
  <MemberPanel :title="$t('wap_user_00213')" :error="error && !isUnauthErr(error) ? error : undefined">
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('wap_00376') : $t('ui.load_failed') }}</p>
    <template v-else>
      <div class="site-h5 financial_management_header">
        <div class="management_header_card">
          <div class="management_card_data">
            <div class="management_data_top">{{ $t('wap_01016') }} {{ priceName }}</div>
            <div class="management_data_cen">{{ bal?.balance ?? 0 }}</div>
            <NuxtLink to="/user/rewards" class="management_data_bom">{{ $t('wap_user_00170') }}</NuxtLink>
          </div>
          <div class="management_card_data">
            <div class="management_data_top">{{ $t('wap_01017') }} {{ priceName }}</div>
            <div class="management_data_cen">{{ usedPoints }}</div>
            <a href="#paylog" class="management_data_bom" @click.prevent="showLogs = true">{{ $t('wap_user_00169') }}</a>
          </div>
        </div>
        <NuxtLink to="/user/pay" class="management_card_btn">{{ $t('wap_user_00172') }}</NuxtLink>
      </div>
      <div class="site-h5 financial_management_body">
        <div class="management_task_center">
          <div class="min_body_task">
            <div class="body_task_header">
              <div class="min_body_header">{{ $t('wap_user_00168') }}</div>
              <NuxtLink to="/user/integral" class="task_header_nav">
                <i class="task_header_nav_text">{{ $t('wap_user_00171') }}</i>
                <div class="task_header_icon">
                  <img src="/legacy/h5/images/vip_nav.png" alt="" width="100%" height="100%">
                </div>
              </NuxtLink>
            </div>
            <div class="body_task_subject">
              <ul>
                <li>
                  <div class="task_subject_box">
                    <div class="task_subject_icon">
                      <img src="/legacy/h5/images/vip_sign_in.png" alt="" width="100%" height="100%">
                    </div>
                    <div class="task_subject_init">
                      <div class="subject_init_top">{{ $t('wap_user_00114') }}</div>
                      <div class="subject_init_bom">{{ pts('integral_signin') }}</div>
                    </div>
                  </div>
                  <button v-if="signed" type="button" class="task_subject_box_btn_accomplish" disabled>{{ $t('wap_00989') }}</button>
                  <button v-else type="button" class="task_subject_box_btn" @click="sign">{{ $t('wap_user_00118') }}</button>
                </li>
                <li>
                  <div class="task_subject_box">
                    <div class="task_subject_icon">
                      <img src="/legacy/h5/images/vip_register.png" alt="" width="100%" height="100%">
                    </div>
                    <div class="task_subject_init">
                      <div class="subject_init_top">{{ $t('wap_user_00108') }}</div>
                      <div class="subject_init_bom">{{ pts('integral_invite_reg') }}</div>
                    </div>
                  </div>
                  <NuxtLink to="/user/invite" class="task_subject_box_btn">{{ $t('wap_user_00121') }}</NuxtLink>
                </li>
                <li>
                  <div class="task_subject_box">
                    <div class="task_subject_icon">
                      <img src="/legacy/h5/images/vip_datum.png" alt="" width="100%" height="100%">
                    </div>
                    <div class="task_subject_init">
                      <div class="subject_init_top">{{ $t('wap_00990') }}</div>
                      <div class="subject_init_bom">{{ pts('integral_userinfo') }}</div>
                    </div>
                  </div>
                  <span v-if="mission?.base_info" class="task_subject_box_btn_accomplish">{{ $t('wap_user_00125') }}</span>
                  <NuxtLink v-else to="/user/resume" class="task_subject_box_btn">{{ $t('wap_user_00117') }}</NuxtLink>
                </li>
              </ul>
            </div>
          </div>
        </div>
      </div>
      <div class="site-pc job_list_tit">
        <ul>
          <li class="job_list_tit_cur"><a href="javascript:;">{{ $t('member_user_00239') }}</a></li>
          <li><NuxtLink to="/user/pay">{{ $t('common_01946') }}</NuxtLink></li>
          <li><NuxtLink to="/user/integral">{{ $t('wap_user_00008') }}</NuxtLink></li>
        </ul>
      </div>
      <p class="site-pc">
        <span v-if="signSt?.signed_today" class="integral_list_bth_s">{{ $t('wap_01022') }}</span>
        <a v-else href="javascript:;" class="integral_list_bth_a" @click="sign">{{ $t('wap_01023') }}</a>
      </p>
      <div v-if="(pays?.list || []).length" class="paylist_tit site-pc">
        <span class="paylist_span paylist_dh">{{ $t('ui.detail') }}</span>
        <span class="paylist_span paylist_money">{{ $t('wap_00925') }}</span>
        <span class="paylist_span paylist_time">{{ $t('member_user_00106') }}</span>
      </div>
      <div v-for="row in pays?.list || []" :key="row.id" class="paylist_list site-pc">
        <span class="paylist_span paylist_dh">{{ packed(row.detail) }}</span>
        <span class="paylist_span paylist_money">{{ row.delta }}</span>
        <span class="paylist_span paylist_time">{{ row.ctime_n || formatUnixDate(row.ctime) }}</span>
      </div>
      <div v-if="showLogs" id="paylog" class="site-h5 detail_body">
        <div v-if="(pays?.list || []).length" class="detail_body_card">
          <ul>
            <li v-for="row in pays?.list || []" :key="row.id">
              <div class="detail_box">
                <div class="detail_box_title">{{ packed(row.detail) || row.delta }}</div>
                <div class="detail_box_time">{{ row.ctime_n || formatUnixDate(row.ctime) }}</div>
              </div>
              <div class="detail_integral">{{ row.delta }}</div>
            </li>
          </ul>
        </div>
      </div>
      <div class="site-pc">
        <MemberPager :page="page" :page-size="pageSize" :total="payTotal" @update:page="go" />
      </div>
      <div v-if="showLogs" class="site-h5">
        <MemberPager :page="page" :page-size="pageSize" :total="payTotal" @update:page="go" />
      </div>
      <p v-if="msg" class="muted">{{ msg }}</p>
    </template>
  </MemberPanel>
</template>
