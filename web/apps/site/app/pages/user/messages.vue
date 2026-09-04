<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('msgs', () =>
  api.post('/v1/mcenter/messages', { page: 1, page_size: 20 }),
)
async function read(id: number) {
  await api.post('/v1/mcenter/messages/read', { id })
  refresh()
}
useSeoMeta({ title: t('common.message') })
</script>

<template>
  <MemberPanel :title="$t('common.message')" :error="error" :empty="!error && !(data?.list || []).length">
    <article v-for="row in data?.list || []" :key="row.id" class="look_resume_list">
      <p>{{ row.title || row.content || row.id }}</p>
      <p class="muted">{{ row.datetime_n }}</p>
      <button type="button" @click="read(row.id)">{{ $t('common.confirm') }}</button>
    </article>
  </MemberPanel>
</template>
