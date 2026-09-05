<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const state = ref<number | null>(null)
const days = ref<number | null>(null)
const { data, error, refresh } = await useAsyncData(
  () => `my-apps-${state.value ?? 'all'}-${days.value ?? 'all'}`,
  () =>
    api.post('/v1/mcenter/my-applications', {
      page: 1,
      page_size: 20,
      ...(state.value === null ? {} : { state: state.value }),
      ...(days.value === null ? {} : { days: days.value }),
    }),
)
watch([state, days], () => refresh())
const list = computed(() => data.value?.list || [])
const msg = ref('')
async function withdraw(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/my-applications/withdraw', { id })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function remove(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/my-applications/delete', { id })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
function browseLabel(row: { is_browse?: number; invited?: boolean }) {
  if (row.invited) return t('wap_user_00216')
  if (row.is_browse === 1) return t('wap_user_00260')
  if (row.is_browse === 2) return t('wap_user_00258')
  if (row.is_browse === 3) return t('wap_user_00266')
  if (row.is_browse === 4) return t('wap_user_00354')
  if (row.is_browse === 5) return t('member_com_00108')
  if (row.is_browse === 7) return t('wap_user_00356')
  return String(row.is_browse ?? '')
}
useSeoMeta({ title: t('wap_user_00270') })
</script>

<template>
  <section>
    <h1>{{ $t('wap_user_00270') }}</h1>
    <p>
      <button type="button" @click="state = null">{{ $t('common.all') }}</button>
      <button type="button" @click="state = 1">{{ $t('wap_user_00260') }}</button>
      <button type="button" @click="state = 3">{{ $t('wap_user_00266') }}</button>
      <button type="button" @click="state = 4">{{ $t('wap_user_00354') }}</button>
      <button type="button" @click="state = 7">{{ $t('wap_user_00356') }}</button>
    </p>
    <p>
      <button type="button" @click="days = null">{{ $t('common.all') }}</button>
      <button type="button" @click="days = 1">1</button>
      <button type="button" @click="days = 3">3</button>
      <button type="button" @click="days = 7">7</button>
      <button type="button" @click="days = 15">15</button>
      <button type="button" @click="days = 30">30</button>
    </p>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('wap_00376') : $t('ui.load_failed') }}</p>
    <p v-else-if="!list.length" class="muted">{{ $t('ui.no_applies') }}</p>
    <div v-else class="stack">
      <article v-for="row in list" :key="row.id" class="job-card">
        <h3>
          <NuxtLink :to="`/jobs/${row.job_id}`">{{ row.job_name || $t('common.job') }}</NuxtLink>
        </h3>
        <p>
          <NuxtLink v-if="row.com_id" :to="`/companies/${row.com_id}`">{{ row.com_name }}</NuxtLink>
        </p>
        <p class="muted">{{ browseLabel(row) }} · {{ row.datetime_n }}</p>
        <button v-if="!row.quxiao" type="button" @click="withdraw(row.id)">{{ $t('common.cancel') }}</button>
        <button type="button" @click="remove(row.id)">{{ $t('common.delete') }}</button>
      </article>
    </div>
    <p v-if="msg">{{ msg }}</p>
  </section>
</template>
