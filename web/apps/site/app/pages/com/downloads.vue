<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { data, error } = await useAsyncData('downloads', () =>
  api.post('/v1/mcenter/resume-downloads/outbox', { page: 1, page_size: 20 }),
)
const msg = ref('')

async function exportCsv() {
  msg.value = ''
  try {
    const r = await api.post<{ csv: string; filename?: string }>('/v1/mcenter/resume-downloads/export', {})
    const blob = new Blob([r.csv || ''], { type: 'text/csv;charset=utf-8' })
    const a = document.createElement('a')
    a.href = URL.createObjectURL(blob)
    a.download = r.filename || 'resume-downloads.csv'
    a.click()
    URL.revokeObjectURL(a.href)
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}

useSeoMeta({ title: t('wap_com_00235') })
</script>

<template>
  <MemberPanel :title="$t('wap_com_00235')" :error="error" :empty="!error && !(data?.list || []).length">
    <p>
      <button type="button" @click="exportCsv">{{ $t('common.submit') }} CSV</button>
    </p>
    <article v-for="row in data?.list || []" :key="row.id || row.uid" class="look_resume_list">
      <NuxtLink :to="`/resumes/${row.eid || row.uid}`">{{ row.name || row.display_name || row.uname || row.uid }}</NuxtLink>
      <p class="muted">{{ row.datetime_n }}</p>
    </article>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
