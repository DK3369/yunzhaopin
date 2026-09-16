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
const tplRows = computed(() => {
  const raw = tpls.value as unknown
  if (Array.isArray(raw)) return raw as Array<{ id: number; name?: string; address?: string; linkman?: string }>
  if (raw && typeof raw === 'object' && 'list' in raw) {
    return ((raw as { list?: Array<{ id: number; name?: string; address?: string; linkman?: string }> }).list || [])
  }
  return []
})
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
    <table v-if="tplRows.length" class="com_table mt20 site-pc">
      <tr>
        <th>{{ $t('wap_com_00413') }}</th>
        <th>{{ $t('wap_00040') }}</th>
        <th>{{ $t('member_user_00048') }}</th>
      </tr>
      <tr v-for="row in tplRows" :key="row.id">
        <td>{{ row.name }}</td>
        <td>{{ row.address }} · {{ row.linkman }}</td>
        <td>
          <a href="javascript:;" class="com_bth cblue" @click="fill(row)">{{ $t('common.edit') }}</a>
          <a href="javascript:;" class="com_bth cblue" @click="removeTpl(row.id)">{{ $t('common.delete') }}</a>
        </td>
      </tr>
    </table>
    <div class="site-h5">
      <div v-for="row in tplRows" :key="'h5-tpl-' + row.id" class="com_cardlist">
        <div class="com_cardlist_tit">{{ row.name }}</div>
        <div class="com_cardlist_p">
          <span class="com_cardlist_p_name">{{ $t('wap_00040') }}</span>
          {{ row.address }} · {{ row.linkman }}
        </div>
        <div class="com_card_cz">
          <a href="javascript:;" class="com_bth cblue" @click="fill(row)">{{ $t('common.edit') }}</a>
          <span class="com_card_delete" @click="removeTpl(row.id)" />
        </div>
      </div>
    </div>
    <form class="com_release_box site-pc" @submit.prevent="saveTpl">
      <ul>
        <MemberReleaseRow :label="$t('wap_com_00288')" required><input v-model="form.name" required class="com_release_textnew_text" /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_user_00102')" area required><textarea v-model="form.content" rows="3" required /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_00040')" required><input v-model="form.address" required class="com_release_textnew_text" /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('common_02051')" required><input v-model="form.linkman" required class="com_release_textnew_text" /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('common.phone')" required><input v-model="form.linktel" required class="com_release_textnew_text" /></MemberReleaseRow>
      </ul>
      <button type="submit" class="btn_01">{{ form.id ? $t('common.save') : $t('common.submit') }}</button>
    </form>
    <div class="site-h5 issue_post_body">
      <form class="yun_createbox" @submit.prevent="saveTpl">
        <MemberField wap :label="$t('wap_com_00288')"><input v-model="form.name" required /></MemberField>
        <MemberField wap area :label="$t('wap_user_00102')"><textarea v-model="form.content" rows="3" required /></MemberField>
        <MemberField wap :label="$t('wap_00040')"><input v-model="form.address" required /></MemberField>
        <MemberField wap :label="$t('common_02051')"><input v-model="form.linkman" required /></MemberField>
        <MemberField wap :label="$t('common.phone')"><input v-model="form.linktel" required /></MemberField>
        <button type="submit" class="issue_post_body_btn">{{ form.id ? $t('common.save') : $t('common.submit') }}</button>
      </form>
    </div>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
