<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { page, pageSize, inferTotal, go } = useMemberListPage()
const { data, error } = await useAsyncData(
  () => `com-followers-${page.value}`,
  () => api.post('/v1/mcenter/followers', { page: page.value, page_size: pageSize }),
)
const rows = computed(() =>
  (data.value?.list || []).map((row: Record<string, unknown>) => ({
    key: Number(row.id || row.uid),
    name: String(row.uname || row.username || row.uid || ''),
    time: String(row.datetime_n || row.time || ''),
    to: `/resumes/${row.uid}`,
  })),
)
const total = computed(() => inferTotal(data.value))
useSeoMeta({ title: t('wap_com_00407') })
</script>

<template>
  <MemberPanel :title="$t('wap_com_00407')" :error="error && !isUnauthErr(error) ? error : undefined" :empty="!error && !(data?.list || []).length">
    <template #pcTabs><MemberHrTabs /></template>
    <template #h5Tabs><MemberHrTabs /></template>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}</p>
    <MemberHrResumeRows v-else :rows="rows" />
    <MemberPager :page="page" :page-size="pageSize" :total="total" @update:page="go" />
  </MemberPanel>
</template>
