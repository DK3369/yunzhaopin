<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

type Row = {
  id: number
  eid?: number
  uid?: number
  user_name?: string
  job_name?: string
  ctime_n?: string
}

const api = useApi()
const { t } = useI18n()
const { page, pageSize, inferTotal, go } = useMemberListPage()
const { data, error, refresh } = await useAsyncData(
  () => `com-entrust-records-${page.value}`,
  () => api.post('/v1/mcenter/entrust-records/list', { page: page.value, page_size: pageSize }),
)
const list = computed(() => (data.value?.list || []) as Row[])
const rows = computed(() =>
  list.value.map((row) => ({
    key: row.id,
    name: String(row.user_name || row.eid || row.uid || ''),
    job: row.job_name,
    time: row.ctime_n,
    to: `/resumes/${row.eid || row.uid}`,
  })),
)
const total = computed(() => inferTotal(data.value))
const msg = ref('')

async function remove(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/entrust-records/delete', { ids: [id] })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.load_failed')
  }
}

useSeoMeta({ title: t('member_com_00555') })
</script>

<template>
  <MemberPanel :title="$t('member_com_00555')" :error="error && !isUnauthErr(error) ? error : undefined" :empty="!error && !list.length">
    <p v-if="error && isUnauthErr(error)" class="muted">{{ $t('common_01153') }}</p>
    <MemberHrResumeRows show-job :rows="rows">
      <template #pc-acts="{ row }">
        <a href="javascript:;" class="List_dete cblue" @click="remove(Number(row.key))">{{ $t('common.delete') }}</a>
      </template>
      <template #h5-acts="{ row }">
        <div class="hr_userlist_czicon" @click="remove(Number(row.key))">{{ $t('common.delete') }}</div>
      </template>
    </MemberHrResumeRows>
    <MemberPager :page="page" :page-size="pageSize" :total="total" @update:page="go" />
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
