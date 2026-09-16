<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { page, pageSize, inferTotal, go } = useMemberListPage()
const keywordInput = ref('')
const keyword = ref('')
const inviteUid = ref(0)
const { data, error } = await useAsyncData(
  () => `com-fans-${page.value}-${keyword.value}`,
  () =>
    api.post('/v1/mcenter/fans', {
      page: page.value,
      page_size: pageSize,
      keyword: keyword.value || undefined,
    }),
)
const raw = computed(() => (data.value?.list || []) as Record<string, unknown>[])
const rows = computed(() =>
  raw.value.map((row) => ({
    key: Number(row.uid),
    name: String(row.username || row.uid || ''),
    time: String(row.last_datetime_n || ''),
    to: `/resumes/${row.uid}`,
    info: Number(row.fav_count || 0) > 0 ? [String(row.fav_count)] : [],
  })),
)
const total = computed(() => inferTotal(data.value))
function search() {
  keyword.value = keywordInput.value.trim()
  go(1)
}
useSeoMeta({ title: t('wap_com_00407') })
</script>

<template>
  <MemberPanel :title="$t('wap_com_00407')" :error="error && !isUnauthErr(error) ? error : undefined" :empty="!error && !(data?.list || []).length">
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
    <MemberHrResumeRows v-if="!error" :rows="rows">
      <template #pc-acts="{ row }">
        <a href="javascript:;" class="cblue" @click="inviteUid = Number(row.key)">{{ $t('wap_com_00046') }}</a>
      </template>
      <template #h5-acts="{ row }">
        <div class="hr_userlist_czicon" @click="inviteUid = Number(row.key)">{{ $t('wap_com_00046') }}</div>
      </template>
    </MemberHrResumeRows>
    <MemberComYqmsForm v-if="inviteUid" :seeker-uid="inviteUid" @done="inviteUid = 0" @cancel="inviteUid = 0" />
    <MemberPager :page="page" :page-size="pageSize" :total="total" @update:page="go" />
  </MemberPanel>
</template>
