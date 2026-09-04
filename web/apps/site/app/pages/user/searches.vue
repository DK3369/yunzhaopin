<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('saved-searches', () =>
  api.post('/v1/mcenter/saved-searches/list', { page: 1, page_size: 20 }),
)
const form = reactive({ name: '', kind: 'job', params: '{"keyword":""}', notify: true })
const msg = ref('')
async function create() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/saved-searches', {
      name: form.name,
      kind: form.kind,
      params: JSON.parse(form.params || '{}'),
      notify: form.notify,
    })
    msg.value = t('ui.saved')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function remove(id: number) {
  await api.post('/v1/mcenter/saved-searches/delete', { id })
  refresh()
}
useSeoMeta({ title: t('ui.searcher') })
</script>

<template>
  <section>
    <h1>{{ $t('ui.searcher') }}</h1>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('wap_00376') : $t('ui.load_failed') }}</p>
    <form class="form" @submit.prevent="create">
      <input v-model="form.name" :placeholder="$t('ui.name')" />
      <input v-model="form.kind" placeholder="kind" />
      <textarea v-model="form.params" rows="3" placeholder='{"keyword":""}' />
      <label><input v-model="form.notify" type="checkbox" /> {{ $t('ui.notify') }}</label>
      <button type="submit">{{ $t('ui.add') }}</button>
    </form>
    <p v-if="msg">{{ msg }}</p>
    <p v-if="!(data?.list || []).length" class="muted">{{ $t('ui.no_searcher') }}</p>
    <div class="stack">
      <article v-for="row in data?.list || []" :key="row.id" class="job-card">
        <h3>{{ row.name }}</h3>
        <p class="muted">kind {{ row.kind }} · notify {{ row.notify }}</p>
        <button type="button" @click="remove(row.id)">{{ $t('common.delete') }}</button>
      </article>
    </div>
  </section>
</template>
