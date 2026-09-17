<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { page, pageSize, inferTotal, go } = useMemberListPage()
const keyword = ref('')
const preview = ref<Record<string, unknown> | null>(null)
const { data, error, refresh } = await useAsyncData(
  () => `com-yqms-${page.value}`,
  () => api.post('/v1/mcenter/company/yqms/list', { page: page.value, page_size: pageSize }),
)
const { data: received } = await useAsyncData('com-iv-received', () =>
  api
    .post<{ list?: Array<{ yqms_id: number; rater_uid: number; total?: number; comment?: string; created_at_n?: string }> }>(
      '/v1/mcenter/interviews/review/received',
      { page: 1, page_size: 20 },
    )
    .catch(() => ({ list: [] })),
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
function openPreview(id: number) {
  const row = (data.value?.list || []).find((r: Record<string, unknown>) => Number(r.id) === id) as
    | Record<string, unknown>
    | undefined
  preview.value = row || null
}
const ivRows = computed(() => {
  const k = keyword.value.trim().toLowerCase()
  return (data.value?.list || [])
    .filter((row: Record<string, unknown>) => {
      if (!k) return true
      const blob = [row.uname, row.uid, row.job_name, row.job_id, row.content, row.address]
        .map((x) => String(x || '').toLowerCase())
        .join(' ')
      return blob.includes(k)
    })
    .map((row: Record<string, unknown>) => ({
      key: Number(row.id),
      name: String(row.uname || row.uid || ''),
      job: String(row.job_name || row.job_id || ''),
      time: String(row.datetime_n || ''),
      to: `/resumes/${row.uid}`,
      invited: true,
      stateText: browseLabel(Number(row.is_browse)),
      info: [String(row.address || ''), String(row.intertime || '')].filter(Boolean),
    }))
})
const ivTotal = computed(() => inferTotal(data.value))
const tplRows = computed(() => {
  const raw = tpls.value as unknown
  if (Array.isArray(raw)) return raw as Array<{ id: number; name?: string; address?: string; linkman?: string }>
  if (raw && typeof raw === 'object' && 'list' in raw) {
    return ((raw as { list?: Array<{ id: number; name?: string; address?: string; linkman?: string }> }).list || [])
  }
  return []
})
useSeoMeta({ title: t('wap_user_00216') })
</script>

<template>
  <MemberPanel :title="$t('wap_user_00216')" :error="error" :empty="false">
    <p class="site-pc">
      <input v-model="keyword" type="search" :placeholder="$t('admin_00149')" />
    </p>
    <div class="site-h5 com-h5-filters">
      <input v-model="keyword" type="search" class="com-h5-filters__kw" :placeholder="$t('admin_00149')" />
    </div>
    <MemberHrResumeRows show-job :rows="ivRows">
      <template #pc-acts="{ row }">
        <a href="javascript:;" class="com_bth cblue" @click="openPreview(Number(row.key))">{{ $t('member_user_00410') }}</a>
        <a href="javascript:;" class="List_dete cblue" @click="cancel(Number(row.key))">{{ $t('common.delete') }}</a>
      </template>
      <template #h5-acts="{ row }">
        <div class="hr_userlist_czicon" @click="openPreview(Number(row.key))">{{ $t('member_user_00410') }}</div>
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
    <div v-if="preview" class="com_release_box site-pc">
      <h3>{{ $t('member_user_00427') }}</h3>
      <p>{{ preview.uname }} · {{ preview.job_name }}</p>
      <p>{{ $t('member_user_00421') }} {{ preview.intertime }}</p>
      <p>{{ $t('member_user_00422') }} {{ preview.address }}</p>
      <p>{{ $t('member_user_00423') }} {{ preview.content }}</p>
      <p>{{ $t('common_02051') }} {{ preview.linkman }} {{ preview.linktel }}</p>
      <button type="button" class="btn_01" @click="preview = null">{{ $t('common.close') }}</button>
    </div>
    <div v-if="preview" class="site-h5 issue_post_body">
      <div class="issue_post_body_card">
        <h3>{{ $t('member_user_00427') }}</h3>
        <p>{{ preview.uname }} · {{ preview.job_name }}</p>
        <p>{{ $t('member_user_00421') }} {{ preview.intertime }}</p>
        <p>{{ $t('member_user_00422') }} {{ preview.address }}</p>
        <p>{{ $t('member_user_00423') }} {{ preview.content }}</p>
        <p>{{ $t('common_02051') }} {{ preview.linkman }} {{ preview.linktel }}</p>
        <button type="button" class="issue_post_body_btn" @click="preview = null">{{ $t('common.close') }}</button>
      </div>
    </div>
    <MemberResumeH1 :title="$t('common.like')" />
    <div v-for="row in received?.list || []" :key="row.yqms_id" class="site-pc paylist_list">
      <span class="paylist_span paylist_dh">{{ row.rater_uid }}</span>
      <span class="paylist_span paylist_money">{{ row.total }}</span>
      <span class="paylist_span paylist_time">{{ row.comment }} · {{ row.created_at_n }}</span>
    </div>
    <div class="site-h5">
      <div v-for="row in received?.list || []" :key="'h5r-' + row.yqms_id" class="com_cardlist">
        <div class="com_cardlist_tit">{{ row.rater_uid }} · {{ row.total }}</div>
        <div class="com_cardlist_p">{{ row.comment }}</div>
      </div>
    </div>
  </MemberPanel>
</template>
