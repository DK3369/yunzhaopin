<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const state = ref<number | null>(null)
const { data, error, refresh } = await useAsyncData(
  () => `my-apps-${state.value ?? 'all'}`,
  () =>
    api.post('/v1/mcenter/my-applications', {
      page: 1,
      page_size: 20,
      ...(state.value === null ? {} : { state: state.value }),
    }),
)
watch(state, () => refresh())
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
function browseLabel(row: { is_browse?: number; invited?: boolean }) {
  if (row.invited) return t('wap_user_00216')
  if (row.is_browse === 1) return t('wap_00129')
  if (row.is_browse === 0) return t('wap_com_00427')
  if (row.is_browse === 3) return t('wap_com_00046')
  if (row.is_browse === 4) return t('wap_user_00167')
  if (row.is_browse === 7) return t('common.yes')
  return String(row.is_browse ?? '')
}
useSeoMeta({ title: t('wap_user_00270') })
</script>

<template>
  <section>
    <h1>{{ $t('wap_user_00270') }}</h1>
    <p>
      <button type="button" @click="state = null">{{ $t('common.all') }}</button>
      <button type="button" @click="state = 1">{{ $t('wap_00129') }}</button>
      <button type="button" @click="state = 0">{{ $t('wap_com_00427') }}</button>
      <button type="button" @click="state = 3">{{ $t('wap_com_00046') }}</button>
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
      </article>
    </div>
    <p v-if="msg">{{ msg }}</p>
  </section>
</template>
