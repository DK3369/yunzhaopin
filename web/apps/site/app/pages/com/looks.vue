<script setup lang="ts">
import { isUnauthErr, mediaUrl } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { page, pageSize, inferTotal, go } = useMemberListPage()
const keywordInput = ref('')
const keyword = ref('')
const inviteUid = ref(0)
const picked = ref<number[]>([])
const { data, error, refresh } = await useAsyncData(
  () => `com-look-jobs-${page.value}-${keyword.value}`,
  () =>
    api.post('/v1/mcenter/look-jobs/list', {
      page: page.value,
      page_size: pageSize,
      keyword: keyword.value || undefined,
    }),
)
const raw = computed(() => (data.value?.list || []) as Record<string, unknown>[])
const rows = computed(() =>
  raw.value.map((row) => ({
    key: Number(row.id),
    name: String(row.uname || row.uid || ''),
    job: String(row.job_name || row.job_id || ''),
    time: String(row.datetime_n || ''),
    to: `/resumes/${row.uid}?eid=${row.eid || ''}`,
    photo: row.photo ? mediaUrl(String(row.photo)) : undefined,
    info: [row.sex_n, row.edu_n, row.exp_n].map((x) => String(x || '')).filter(Boolean),
  })),
)
const total = computed(() => inferTotal(data.value))
const msg = ref('')
function search() {
  keyword.value = keywordInput.value.trim()
  go(1)
}
async function remove(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/look-jobs/delete', { id })
    picked.value = picked.value.filter((x) => x !== id)
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function removePicked() {
  if (!picked.value.length) {
    msg.value = t('common_01164')
    return
  }
  msg.value = ''
  try {
    await api.post('/v1/mcenter/look-jobs/delete', { ids: picked.value })
    picked.value = []
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
function pickInvite(id: number) {
  const row = raw.value.find((r) => Number(r.id) === id)
  if (row) inviteUid.value = Number(row.uid || 0)
}
watch(page, () => {
  picked.value = []
})
useSeoMeta({ title: t('member_com_00007') })
</script>

<template>
  <MemberPanel :title="$t('member_com_00007')" :error="error && !isUnauthErr(error) ? error : undefined" :empty="!error && !(data?.list || []).length">
    <template #pcTabs><MemberHrTabs /></template>
    <template #h5Tabs><MemberHrTabs /></template>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}</p>
    <p class="site-pc">
      <input v-model="keywordInput" type="search" :placeholder="$t('admin_00149')" @keydown.enter.prevent="search" />
      <button type="button" class="com_topbth" @click="search">{{ $t('common.search') }}</button>
    </p>
    <div class="site-h5 com-h5-filters">
      <input v-model="keywordInput" type="search" class="com-h5-filters__kw" :placeholder="$t('admin_00149')" @keydown.enter.prevent="search" />
      <button type="button" class="issue_post_body_btn" @click="search">{{ $t('common.search') }}</button>
    </div>
    <MemberHrResumeRows v-if="!error" v-model:picked="picked" selectable show-job :rows="rows">
      <template #pc-acts="{ row }">
        <a href="javascript:;" class="com_bth" @click="pickInvite(Number(row.key))">{{ $t('wap_com_00046') }}</a>
        <a href="javascript:;" class="com_bth" @click="remove(Number(row.key))">{{ $t('common.delete') }}</a>
      </template>
      <template #h5-acts="{ row }">
        <div class="hr_userlist_czicon" @click="pickInvite(Number(row.key))">{{ $t('wap_com_00046') }}</div>
        <div class="hr_userlist_czicon" @click="remove(Number(row.key))">{{ $t('common.delete') }}</div>
      </template>
    </MemberHrResumeRows>
    <div v-if="rows.length" class="com_Release_job_bot">
      <a href="javascript:;" class="c_btn_02" @click="removePicked">{{ $t('common.delete') }}</a>
    </div>
    <MemberComYqmsForm v-if="inviteUid" :seeker-uid="inviteUid" @done="inviteUid = 0" @cancel="inviteUid = 0" />
    <MemberPager :page="page" :page-size="pageSize" :total="total" @update:page="go" />
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
