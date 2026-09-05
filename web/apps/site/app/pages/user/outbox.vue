<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('resume-outbox', () =>
  api.post('/v1/mcenter/resume-outbox/list', { page: 1, page_size: 20 }),
)
const { data: expects } = await useAsyncData('outbox-expects', () =>
  api.post('/v1/mcenter/resume/expects/list', {}).catch(() => []),
)
const expectList = computed(() => {
  const raw = expects.value
  if (Array.isArray(raw)) return raw
  return raw?.list || []
})
const form = reactive({
  resume_id: 0,
  email: '',
  com_name: '',
  job_name: '',
  resume_name: '',
})
watch(
  expectList,
  (list) => {
    if (!form.resume_id && list[0]?.id) form.resume_id = Number(list[0].id)
  },
  { immediate: true },
)
const msg = ref('')
async function send() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/resume-outbox', { ...form })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function remove(id: number) {
  if (!window.confirm(t('member_com_00083'))) return
  msg.value = ''
  try {
    await api.post('/v1/mcenter/resume-outbox/delete', { ids: [id] })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('member_user_00188') })
</script>

<template>
  <section>
    <h1>{{ $t('member_user_00188') }}</h1>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('wap_00376') : $t('ui.load_failed') }}</p>
    <form class="form" @submit.prevent="send">
      <select v-model.number="form.resume_id" required>
        <option :value="0">{{ $t('common.resume') }}</option>
        <option v-for="row in expectList" :key="row.id" :value="row.id">{{ row.name || row.id }}</option>
      </select>
      <input v-model="form.email" :placeholder="$t('member_user_00282')" />
      <input v-model="form.com_name" :placeholder="$t('wap_com_00157')" />
      <input v-model="form.job_name" :placeholder="$t('wap_com_00288')" />
      <input v-model="form.resume_name" :placeholder="$t('common.resume')" />
      <button type="submit">{{ $t('common.submit') }}</button>
    </form>
    <p v-if="msg">{{ msg }}</p>
    <p v-if="!(data?.list || []).length" class="muted">{{ $t('ui.no_outbox') }}</p>
    <div class="stack">
      <article v-for="row in data?.list || []" :key="row.id" class="job-card">
        <h3>{{ row.com_name }} · {{ row.job_name }}</h3>
        <p class="muted">{{ row.email }} · {{ row.addtime_n }}</p>
        <button type="button" @click="remove(row.id)">{{ $t('common.delete') }}</button>
      </article>
    </div>
  </section>
</template>
