<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('com-apps', () =>
  api.post<{ list: Array<{ id: number; uid: number; job_id: number; datetime_n?: string; is_browse?: number; eid?: number }> }>(
    '/v1/mcenter/applications',
    { page: 1, page_size: 20 },
  ),
)
const list = computed(() => data.value?.list || [])
const invite = reactive({
  seeker_uid: 0,
  job_id: 0,
  content: '',
  address: '',
  intertime: '',
  linkman: '',
  linktel: '',
  save_yqmb: false,
})
const msg = ref('')
function pick(row: { uid: number; job_id: number }) {
  invite.seeker_uid = row.uid
  invite.job_id = row.job_id
}
async function setState(id: number, state: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/applications/state', { id, state })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('common_00888')
  }
}
function browseLabel(state?: number) {
  if (state === 1) return t('wap_00129')
  if (state === 0) return t('wap_com_00427')
  if (state === 3) return t('wap_com_00046')
  if (state === 4) return t('wap_user_00167')
  if (state === 7) return t('common.yes')
  return String(state ?? '')
}
async function sendInvite(confirm = false) {
  msg.value = ''
  if (!invite.seeker_uid || !invite.job_id) {
    msg.value = t('common_01153')
    return
  }
  if (!invite.intertime.trim()) {
    msg.value = t('member_com_00681')
    return
  }
  try {
    const res = await api.post<{ status: number; jifen?: number; price?: number; msg_key?: string }>(
      '/v1/mcenter/company/yqms/create',
      { ...invite, confirm },
    )
    if (res.status === 2) {
      const text = res.jifen
        ? `${t('common_00697')}${res.jifen}${t('common_01935')}?`
        : res.price
          ? `${t('common_00696')}${res.price}${t('common_00757')}?`
          : t('common_00696')
      if (window.confirm(text)) {
        await sendInvite(true)
      }
      return
    }
    msg.value = t('wap_00291')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('common_00888')
  }
}
useSeoMeta({ title: t('wap_com_00420') })
</script>

<template>
  <section>
    <h1>{{ $t('wap_com_00420') }}</h1>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}</p>
    <p v-else-if="!list.length" class="muted">{{ $t('wap_00129') }}</p>
    <div class="stack">
      <article v-for="row in list" :key="row.id" class="job-card">
        <h3>#{{ row.id }} · {{ $t('common.resume') }} {{ row.uid }}</h3>
        <p class="muted">{{ $t('common.job') }} {{ row.job_id }} · {{ row.datetime_n }} · {{ browseLabel(row.is_browse) }}</p>
        <button type="button" @click="pick(row)">{{ $t('wap_com_00046') }}</button>
        <button type="button" @click="setState(row.id, 0)">{{ $t('wap_com_00427') }}</button>
        <button type="button" @click="setState(row.id, 4)">{{ $t('wap_user_00167') }}</button>
        <button type="button" @click="setState(row.id, 3)">{{ $t('wap_com_00046') }}</button>
        <button type="button" @click="setState(row.id, 7)">{{ $t('common.yes') }}</button>
      </article>
    </div>
    <h2>{{ $t('wap_com_00046') }}</h2>
    <form class="form" @submit.prevent="sendInvite">
      <input v-model.number="invite.seeker_uid" type="number" :placeholder="$t('common.resume')" required />
      <input v-model.number="invite.job_id" type="number" :placeholder="$t('common.job')" required />
      <input v-model="invite.intertime" type="datetime-local" required />
      <input v-model="invite.address" :placeholder="$t('wap_00040')" required />
      <input v-model="invite.linkman" :placeholder="$t('common_02051')" />
      <input v-model="invite.linktel" :placeholder="$t('common.phone')" required />
      <textarea v-model="invite.content" rows="3" />
      <label><input v-model="invite.save_yqmb" type="checkbox" /> {{ $t('member_com_00512') }}</label>
      <button type="submit">{{ $t('common.submit') }}</button>
      <p v-if="msg">{{ msg }}</p>
    </form>
  </section>
</template>
