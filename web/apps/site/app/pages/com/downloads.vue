<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { page, pageSize, inferTotal, go } = useMemberListPage()
const { data, error } = await useAsyncData(
  () => `downloads-${page.value}`,
  () => api.post('/v1/mcenter/resume-downloads/outbox', { page: page.value, page_size: pageSize }),
)
const rows = computed(() =>
  (data.value?.list || []).map((row: Record<string, unknown>) => ({
    key: Number(row.id || row.uid),
    name: String(row.name || row.display_name || row.uname || row.uid || ''),
    time: String(row.datetime_n || ''),
    to: `/resumes/${row.eid || row.uid}`,
    downloaded: true,
  })),
)
const total = computed(() => inferTotal(data.value))
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
    <template #pcTabs><MemberHrTabs /></template>
    <template #h5Tabs><MemberHrTabs /></template>
    <p class="site-pc">
      <button type="button" class="com_topbth" @click="exportCsv">{{ $t('common.submit') }} CSV</button>
    </p>
    <MemberHrResumeRows :rows="rows" />
    <MemberPager :page="page" :page-size="pageSize" :total="total" @update:page="go" />
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
