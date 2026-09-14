<script setup lang="ts">
import { isUnauthErr, mediaUrl } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { page, pageSize, inferTotal, go } = useMemberListPage()
const { data, error, refresh } = await useAsyncData(
  () => `com-look-resumes-${page.value}`,
  () => api.post('/v1/mcenter/look-resumes/mine', { page: page.value, page_size: pageSize }),
)
const rows = computed(() =>
  (data.value?.list || []).map((row: Record<string, unknown>) => ({
    key: Number(row.id),
    name: String(row.resume_name || row.uid || ''),
    time: String(row.datetime_n || ''),
    to: `/resumes/${row.resume_id || row.uid}`,
    photo: row.photo ? mediaUrl(String(row.photo)) : undefined,
    info: [row.sex_n, row.edu_n, row.exp_n].map((x) => String(x || '')).filter(Boolean),
  })),
)
const total = computed(() => inferTotal(data.value))
const msg = ref('')
async function remove(id: number) {
  if (!window.confirm(t('member_com_00083'))) return
  msg.value = ''
  try {
    await api.post('/v1/mcenter/look-resumes/delete', { id })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('member_com_00006') })
</script>

<template>
  <MemberPanel :title="$t('member_com_00006')" :error="error && !isUnauthErr(error) ? error : undefined" :empty="!error && !(data?.list || []).length">
    <template #pcTabs><MemberHrTabs /></template>
    <template #h5Tabs><MemberHrTabs /></template>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}</p>
    <MemberHrResumeRows v-else :rows="rows">
      <template #pc-acts="{ row }">
        <a href="javascript:;" class="com_bth" @click="remove(Number(row.key))">{{ $t('common.delete') }}</a>
      </template>
      <template #h5-acts="{ row }">
        <div class="hr_userlist_czicon" @click="remove(Number(row.key))">{{ $t('common.delete') }}</div>
      </template>
    </MemberHrResumeRows>
    <MemberPager :page="page" :page-size="pageSize" :total="total" @update:page="go" />
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>

<style scoped>
.row {
  display: flex;
  gap: 0.5rem;
  align-items: center;
  flex-wrap: wrap;
}
</style>
