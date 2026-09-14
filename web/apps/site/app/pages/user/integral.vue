<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

type Mission = {
  base_info?: boolean
  photo?: boolean
  signin?: boolean
  email_checked?: boolean
  phone_checked?: boolean
  identification?: boolean
  weixin_bind?: boolean
  question?: boolean
  answer?: boolean
  answerpl?: boolean
  resume?: boolean
}

const api = useApi()
const { t } = useI18n()
const { settings } = useSiteChrome()
const { page, pageSize, inferTotal } = useMemberListPage()
const { data: bal, error, refresh: refreshBal } = await useAsyncData('integral-bal', () =>
  api.post('/v1/mcenter/integral/balance', {}),
)
const { data: hist } = await useAsyncData(
  () => `integral-hist-${page.value}`,
  () => api.post('/v1/mcenter/integral/history', { page: page.value, page_size: pageSize }),
)
const { data: signSt, refresh: refreshSign } = await useAsyncData('sign-status', () =>
  api.post<{ signed_today?: boolean; signday?: number; signdays?: number }>('/v1/mcenter/sign/status', {}).catch(() => null),
)
const { data: mission, refresh: refreshMission } = await useAsyncData('user-integral-mission', () =>
  api.post<Mission>('/v1/mcenter/integral/mission', {}).catch(() => null),
)
const msg = ref('')
function pts(key: string) {
  const n = String(settings.value[key] || '').trim()
  return n ? `+${n}` : ''
}
const tasks = computed(() => [
  { done: mission.value?.signin || signSt.value?.signed_today, title: t('wap_user_00114'), reward: pts('integral_signin'), to: '', doneText: t('wap_01022'), go: t('wap_01023'), sign: true, pc: 'integral_list_n_c4', h5: 'yun_integral_icon_qd' },
  { done: false, title: t('wap_user_00108'), reward: pts('integral_invite_reg'), to: '/user/invite', doneText: '', go: t('wap_01024'), sign: false, pc: 'integral_list_n_c3', h5: 'yun_integral_icon_yq' },
  { done: mission.value?.base_info, title: t('wap_00990'), reward: pts('integral_userinfo'), to: '/user/resume', doneText: t('wap_user_00125'), go: t('wap_user_00117'), sign: false, pc: 'integral_list_n_c1', h5: 'yun_integral_icon_ws' },
  { done: mission.value?.photo, title: t('wap_user_00110'), reward: pts('integral_avatar'), to: '/user/resume', doneText: t('wap_user_00123'), go: t('wap_user_00110'), sign: false, pc: 'integral_list_n_c2', h5: 'yun_integral_icon_sctx' },
  { done: true, title: t('wap_01025'), reward: pts('integral_login'), to: '', doneText: t('wap_01026'), go: '', sign: false, pc: 'integral_list_n_c4', h5: 'yun_integral_icon_dl' },
  { done: mission.value?.email_checked, title: t('wap_01027'), reward: pts('integral_emailcert'), to: '/user/binding', doneText: t('wap_user_00246'), go: t('wap_01029'), sign: false, pc: 'integral_list_n_c5', h5: 'yun_integral_icon_rzyx' },
  { done: mission.value?.phone_checked, title: t('wap_01028'), reward: pts('integral_mobliecert'), to: '/user/binding', doneText: t('wap_user_00128'), go: t('wap_01029'), sign: false, pc: 'integral_list_n_c6', h5: 'yun_integral_icon_rzsj' },
  { done: mission.value?.resume, title: t('common.publish_resume'), reward: pts('integral_add_resume'), to: '/user/resume', doneText: t('wap_user_00124'), go: t('common.publish_resume'), sign: false, pc: 'integral_list_n_c1', h5: 'yun_integral_icon_fbjl' },
  { done: mission.value?.identification, title: t('wap_user_00106'), reward: pts('integral_identity'), to: '/user/ident', doneText: t('wap_user_00246'), go: t('wap_01030'), sign: false, pc: 'integral_list_n_c2', h5: 'yun_integral_icon_yz' },
  { done: mission.value?.weixin_bind, title: t('wap_user_00115'), reward: pts('integral_bind_wx'), to: '/user/binding', doneText: t('wap_user_00127'), go: t('wap_user_00119'), sign: false, pc: 'integral_list_n_c3', h5: 'yun_integral_icon_wx' },
  { done: mission.value?.question, title: t('wap_user_00112'), reward: pts('integral_question'), to: '/questions', doneText: t('wap_00992'), go: t('wap_user_00112'), sign: false, pc: 'integral_list_n_c5', h5: 'yun_integral_icon_fbwt' },
  { done: mission.value?.answer, title: t('wap_user_00113'), reward: pts('integral_answer'), to: '/questions', doneText: t('wap_00993'), go: t('wap_user_00113'), sign: false, pc: 'integral_list_n_c6', h5: 'yun_integral_icon_hdwt' },
  { done: mission.value?.answerpl, title: t('wap_00995'), reward: pts('integral_answerpl'), to: '/questions', doneText: t('wap_00996'), go: t('wap_00995'), sign: false, pc: 'integral_list_n_c4', h5: 'yun_integral_icon_plwt' },
])
async function sign() {
  msg.value = ''
  try {
    const r = await api.post<{ reward?: number }>('/v1/mcenter/sign', {})
    msg.value = `${t('common.success')} ${r?.reward ?? ''}`
    await refreshSign()
    await refreshBal()
    await refreshMission()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
const histTotal = computed(() => inferTotal(hist.value))
useSeoMeta({ title: t('wap_user_00008') })
</script>

<template>
  <MemberPanel :title="$t('wap_user_00008')" :error="error && !isUnauthErr(error) ? error : undefined">
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('wap_00376') : $t('ui.load_failed') }}</p>
    <template v-else>
      <div class="site-h5 yun_usermember_financebg">
        <div class="yun_usermember_integral">
          {{ $t('wap_01018') }}{{ $t('wap_user_00008') }}：
          <span class="yun_usermember_integral_n">{{ bal?.balance ?? 0 }}</span>
        </div>
      </div>
      <div class="site-h5 yun_usermember_integral_b">
        <div class="yun_usermember_integral_line" />
        <ul class="yun_usermember_integral_nav">
          <li>
            <NuxtLink to="/user/finance"><i class="yun_usermember_integral_nav_icon yun_usermember_integral_nav_iconmx" />{{ $t('wap_01020') }}</NuxtLink>
          </li>
          <li>
            <NuxtLink to="/user/pay"><i class="yun_usermember_integral_nav_icon yun_usermember_integral_nav_icongz" />{{ $t('common_01946') }}</NuxtLink>
          </li>
          <li>
            <NuxtLink to="/redeem"><i class="yun_usermember_integral_nav_icon yun_usermember_integral_nav_iconsc" />{{ $t('wap_00398') }}</NuxtLink>
          </li>
        </ul>
      </div>
      <div class="site-pc job_list_tit">
        <ul>
          <li class="job_list_tit_cur"><a href="javascript:;">{{ $t('wap_user_00008') }}</a></li>
          <li><NuxtLink to="/user/finance">{{ $t('member_user_00190') }}</NuxtLink></li>
        </ul>
      </div>
      <p class="site-pc muted">{{ $t('ui.balance') }} {{ bal?.balance ?? 0 }}
        <span v-if="signSt"> {{ signSt.signday ?? 0 }} / {{ signSt.signdays ?? 0 }}</span>
      </p>
      <div class="site-pc integral_list_box">
        <ul class="integral_list">
          <li v-for="(row, i) in tasks" :key="i">
            <div class="integral_list_n" :class="row.pc">{{ row.reward }}</div>
            <div class="integral_listname">{{ row.title }}</div>
            <div class="integral_list_p">{{ row.reward }}</div>
            <div class="integral_list_bth">
              <span v-if="row.done" class="integral_list_bth_s">{{ row.doneText }}</span>
              <a v-else-if="row.sign" href="javascript:;" class="integral_list_bth_a" @click="sign">{{ row.go }}</a>
              <NuxtLink v-else-if="row.to" :to="row.to" class="integral_list_bth_a">{{ row.go }}</NuxtLink>
            </div>
          </li>
        </ul>
      </div>
      <div class="site-h5 yun_usermember_integral_box">
        <div class="yun_usermember_integral_box_h1">{{ $t('wap_01021') }}</div>
        <ul class="yun_usermember_integral_list">
          <li v-for="(row, i) in tasks" :key="'h5-' + i">
            <i class="yun_integral_icon" :class="row.h5" />
            <div class="yun_integral_name">{{ row.title }}</div>
            <div class="yun_integral_n">{{ row.reward }}</div>
            <span v-if="row.done" class="yun_integral_bth">{{ row.doneText }}</span>
            <a v-else-if="row.sign" href="javascript:;" class="yun_integral_a" @click="sign">{{ row.go }}</a>
            <NuxtLink v-else-if="row.to" :to="row.to" class="yun_integral_a">{{ row.go }}</NuxtLink>
          </li>
        </ul>
      </div>
      <MemberResumeH1 :title="$t('ui.flow')" />
      <div v-for="(row, i) in hist?.list || []" :key="row.id || i" class="site-pc paylist_list">
        <span class="paylist_span paylist_dh">{{ row.item_id || row.id }}</span>
        <span class="paylist_span paylist_money">{{ row.cost ?? row.delta ?? '' }}</span>
        <span class="paylist_span paylist_time">{{ row.created_at || row.ctime }}</span>
      </div>
      <div class="site-h5 detail_body">
        <div v-if="(hist?.list || []).length" class="detail_body_card">
          <ul>
            <li v-for="(row, i) in hist?.list || []" :key="'h-' + (row.id || i)">
              <div class="detail_box">
                <div class="detail_box_title">{{ row.item_id || row.id }}</div>
                <div class="detail_box_time">{{ row.created_at || row.ctime }}</div>
              </div>
              <div class="detail_integral">{{ row.cost ?? row.delta ?? '' }}</div>
            </li>
          </ul>
        </div>
      </div>
      <MemberPager :page="page" :page-size="pageSize" :total="histTotal" @update:page="page = $event" />
      <p v-if="msg" class="muted">{{ msg }}</p>
    </template>
  </MemberPanel>
</template>
