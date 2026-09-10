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
const { data: bal, error, refresh: refreshBal } = await useAsyncData('integral-bal', () =>
  api.post('/v1/mcenter/integral/balance', {}),
)
const { data: hist } = await useAsyncData('integral-hist', () =>
  api.post('/v1/mcenter/integral/history', { page: 1, page_size: 20 }),
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
  { done: mission.value?.signin || signSt.value?.signed_today, title: t('wap_user_00114'), reward: pts('integral_signin'), to: '', doneText: t('wap_01022'), go: t('wap_01023'), sign: true },
  { done: false, title: t('wap_user_00108'), reward: pts('integral_invite_reg'), to: '/user/invite', doneText: '', go: t('wap_01024'), sign: false },
  { done: mission.value?.base_info, title: t('wap_00990'), reward: pts('integral_userinfo'), to: '/user/resume', doneText: t('wap_user_00125'), go: t('wap_user_00117'), sign: false },
  { done: mission.value?.photo, title: t('wap_user_00110'), reward: pts('integral_avatar'), to: '/user/resume', doneText: t('wap_user_00123'), go: t('wap_user_00110'), sign: false },
  { done: true, title: t('wap_01025'), reward: pts('integral_login'), to: '', doneText: t('wap_01026'), go: '', sign: false },
  { done: mission.value?.email_checked, title: t('wap_01027'), reward: pts('integral_emailcert'), to: '/user/binding', doneText: t('wap_user_00246'), go: t('wap_01029'), sign: false },
  { done: mission.value?.phone_checked, title: t('wap_01028'), reward: pts('integral_mobliecert'), to: '/user/binding', doneText: t('wap_user_00128'), go: t('wap_01029'), sign: false },
  { done: mission.value?.resume, title: t('common.publish_resume'), reward: pts('integral_add_resume'), to: '/user/resume', doneText: t('wap_user_00124'), go: t('common.publish_resume'), sign: false },
  { done: mission.value?.identification, title: t('wap_user_00106'), reward: pts('integral_identity'), to: '/user/ident', doneText: t('wap_user_00246'), go: t('wap_01030'), sign: false },
  { done: mission.value?.weixin_bind, title: t('wap_user_00115'), reward: pts('integral_bind_wx'), to: '/user/binding', doneText: t('wap_user_00127'), go: t('wap_user_00119'), sign: false },
  { done: mission.value?.question, title: t('wap_user_00112'), reward: pts('integral_question'), to: '/questions', doneText: t('wap_00992'), go: t('wap_user_00112'), sign: false },
  { done: mission.value?.answer, title: t('wap_user_00113'), reward: pts('integral_answer'), to: '/questions', doneText: t('wap_00993'), go: t('wap_user_00113'), sign: false },
  { done: mission.value?.answerpl, title: t('wap_00995'), reward: pts('integral_answerpl'), to: '/questions', doneText: t('wap_00996'), go: t('wap_00995'), sign: false },
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
useSeoMeta({ title: t('wap_user_00008') })
</script>

<template>
  <section>
    <h1>{{ $t('wap_user_00008') }}</h1>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('wap_00376') : $t('ui.load_failed') }}</p>
    <template v-else>
      <p>{{ $t('ui.balance') }} {{ bal?.balance ?? 0 }}</p>
      <p>
        <span v-if="signSt" class="muted"> {{ signSt.signday ?? 0 }} / {{ signSt.signdays ?? 0 }}</span>
      </p>
      <h2>{{ $t('wap_01021') }}</h2>
      <div class="stack">
        <article v-for="(row, i) in tasks" :key="i" class="job-card">
          <h3>{{ row.title }} <span v-if="row.reward" class="muted">{{ row.reward }}</span></h3>
          <p v-if="row.done" class="muted">{{ row.doneText }}</p>
          <p v-else-if="row.sign">
            <button type="button" :disabled="!!signSt?.signed_today" @click="sign">{{ row.go }}</button>
          </p>
          <p v-else-if="row.to">
            <NuxtLink :to="row.to">{{ row.go }}</NuxtLink>
          </p>
        </article>
      </div>
      <h2>{{ $t('ui.flow') }}</h2>
      <p v-if="!(hist?.list || []).length" class="muted">{{ $t('ui.no_items') }}</p>
      <div class="stack">
        <article v-for="(row, i) in hist?.list || []" :key="row.id || i" class="job-card">
          <h3>{{ row.item_id || row.id }}</h3>
          <p class="muted">{{ row.cost ?? row.delta ?? '' }} · {{ row.status ?? '' }} · {{ row.created_at || row.ctime }}</p>
        </article>
      </div>
      <p v-if="msg">{{ msg }}</p>
    </template>
  </section>
</template>
