<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { page, pageSize, inferTotal, go } = useMemberListPage()
const { data, error, refresh } = await useAsyncData(
  () => `com-yqms-${page.value}`,
  () => api.post('/v1/mcenter/company/yqms/list', { page: page.value, page_size: pageSize }),
)
const { data: tpls, refresh: refreshTpls } = await useAsyncData('com-iv-tpls', () =>
  api.post('/v1/mcenter/interview-templates/list', {}).catch(() => []),
)
const form = reactive({
  id: 0,
  name: '',
  content: '',
  address: '',
  linkman: '',
  linktel: '',
  intertime: 0,
})
const invite = reactive({
  seeker_uid: 0,
  job_id: 0,
  content: '',
  address: '',
  intertime: '',
  linkman: '',
  linktel: '',
  ymid: 0,
  save_yqmb: false,
})
const msg = ref('')
function fill(row: {
  id: number
  name?: string
  content?: string
  address?: string
  linkman?: string
  linktel?: string
  intertime?: number | string
}) {
  form.id = row.id
  form.name = String(row.name || '')
  form.content = String(row.content || '')
  form.address = String(row.address || '')
  form.linkman = String(row.linkman || '')
  form.linktel = String(row.linktel || '')
  form.intertime = Number(row.intertime || 0)
  invite.content = form.content
  invite.address = form.address
  invite.linkman = form.linkman
  invite.linktel = form.linktel
  invite.ymid = row.id
}
async function saveTpl() {
  msg.value = ''
  try {
    if (form.id) {
      await api.post('/v1/mcenter/interview-templates/update', { ...form })
    } else {
      await api.post('/v1/mcenter/interview-templates', { ...form })
    }
    msg.value = t('common.success')
    form.id = 0
    await refreshTpls()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function removeTpl(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/interview-templates/update', { id, status: 2 })
    msg.value = t('common.success')
    await refreshTpls()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function cancel(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/company/yqms/cancel', { id })
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
function browseLabel(state?: number) {
  if (state === 3) return t('wap_com_00190')
  if (state === 4) return t('wap_user_00257')
  return t('wap_user_00260')
}
useSeoMeta({ title: t('wap_user_00216') })
const ivRows = computed(() =>
  (data.value?.list || []).map((row: Record<string, unknown>) => ({
    key: Number(row.id),
    name: String(row.uname || row.uid || ''),
    job: String(row.job_name || row.job_id || ''),
    time: String(row.datetime_n || ''),
    to: `/resumes/${row.uid}`,
    invited: true,
    info: [String(row.address || ''), String(row.intertime || '')].filter(Boolean),
  })),
)
const ivTotal = computed(() => inferTotal(data.value))
</script>

<template>
  <MemberPanel :title="$t('wap_user_00216')" :error="error" :empty="false">
    <MemberHrResumeRows show-job :rows="ivRows">
      <template #pc-acts="{ row }">
        <a href="javascript:;" class="List_dete cblue" @click="cancel(Number(row.key))">{{ $t('common.delete') }}</a>
      </template>
      <template #h5-acts="{ row }">
        <div class="hr_userlist_czicon" @click="cancel(Number(row.key))">{{ $t('common.delete') }}</div>
      </template>
    </MemberHrResumeRows>
    <MemberPager :page="page" :page-size="pageSize" :total="ivTotal" @update:page="go" />
    <MemberResumeH1 :title="$t('member_com_00512')" />
    <div v-for="row in (Array.isArray(tpls) ? tpls : tpls?.list || [])" :key="row.id" class="sysynews_list site-pc">
      <div class="sysynews_span sysynews_name">{{ row.name }}</div>
      <div class="sysynews_span sysynews_time">{{ row.address }} · {{ row.linkman }}</div>
      <div class="sysynews_span sysynews_cz">
        <a href="javascript:;" class="cblue" @click="fill(row)">{{ $t('common.edit') }}</a>
        <a href="javascript:;" class="List_dete cblue" @click="removeTpl(row.id)">{{ $t('common.delete') }}</a>
      </div>
    </div>
    <div class="site-h5 m_cardbox">
      <MemberSxNewsCard
        v-for="row in (Array.isArray(tpls) ? tpls : tpls?.list || [])"
        :key="'h5-tpl-' + row.id"
        :title="row.name"
        :time="`${row.address} · ${row.linkman}`"
      />
    </div>
    <form class="com_release_box" @submit.prevent="saveTpl">
      <ul>
        <MemberReleaseRow :label="$t('wap_com_00288')" required><input v-model="form.name" required class="com_release_textnew_text" /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_user_00102')" area required><textarea v-model="form.content" rows="3" required /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_00040')" required><input v-model="form.address" required class="com_release_textnew_text" /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('common_02051')" required><input v-model="form.linkman" required class="com_release_textnew_text" /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('common.phone')" required><input v-model="form.linktel" required class="com_release_textnew_text" /></MemberReleaseRow>
      </ul>
      <button type="submit" class="btn_01">{{ form.id ? $t('common.save') : $t('common.submit') }}</button>
    </form>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
