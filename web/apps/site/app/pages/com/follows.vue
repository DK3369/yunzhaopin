<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('com-follows', () =>
  api.post('/v1/mcenter/follows/list', { page: 1, page_size: 20 }),
)
async function toggle(row: { target_uid?: number; uid?: number }) {
  await api.post('/v1/mcenter/follows', { target_kind: 1, target_uid: row.target_uid || row.uid })
  refresh()
}
useSeoMeta({ title: t('wap_00385') })
</script>

<template>
  <section>
    <h1>{{ $t('wap_00385') }}</h1>
    <p v-if="error" class="muted">{{ $t('common.login') }}</p>
    <article v-for="row in data?.list || []" :key="row.target_uid || row.uid" class="job-card">
      <p>{{ row.name || row.target_uid }}</p>
      <button type="button" @click="toggle(row)">{{ $t('common.delete') }}</button>
    </article>
  </section>
</template>
