<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('my-reports', () =>
  api.post('/v1/mcenter/reports/list', { page: 1, page_size: 20 }),
)
const form = reactive({ target_kind: 1, target_id: 0, reason_code: 'spam', detail: '' })
const msg = ref('')
async function submit() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/reports', { ...form })
    msg.value = t('ui.submitted')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('ui.my_reports') })
</script>

<template>
  <section>
    <h1>{{ $t('ui.my_reports') }}</h1>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('wap_00376') : $t('ui.load_failed') }}</p>
    <form class="form" @submit.prevent="submit">
      <select v-model.number="form.target_kind">
        <option :value="1">{{ $t('common.job') }}</option>
        <option :value="2">{{ $t('common.company') }}</option>
        <option :value="3">{{ $t('common.resume') }}</option>
        <option :value="4">{{ $t('common.article') }}</option>
        <option :value="5">{{ $t('ui.user_kind') }}</option>
      </select>
      <input v-model.number="form.target_id" type="number" placeholder="target_id" />
      <input v-model="form.reason_code" placeholder="reason_code" />
      <textarea v-model="form.detail" rows="3" :placeholder="$t('ui.detail')" />
      <button type="submit">{{ $t('common.submit') }}</button>
    </form>
    <p v-if="msg">{{ msg }}</p>
    <p v-if="!(data?.list || []).length" class="muted">{{ $t('ui.no_reports') }}</p>
    <div class="stack">
      <article v-for="row in data?.list || []" :key="row.id" class="job-card">
        <h3>kind {{ row.target_kind }} #{{ row.target_id }}</h3>
        <p class="muted">status {{ row.status }}</p>
      </article>
    </div>
  </section>
</template>
