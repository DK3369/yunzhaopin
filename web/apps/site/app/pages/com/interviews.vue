<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('com-yqms', () =>
  api.post('/v1/mcenter/company/yqms/list', { page: 1, page_size: 20 }),
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
</script>

<template>
  <MemberPanel :title="$t('wap_user_00216')" :error="error" :empty="false">
    <article v-for="row in data?.list || []" :key="row.id" class="look_resume_list">
      <p>
        <NuxtLink :to="`/resumes/${row.uid}`">{{ row.uname || row.uid }}</NuxtLink>
        · {{ row.job_name || row.job_id }} · {{ browseLabel(row.is_browse) }}
      </p>
      <p class="muted">{{ row.datetime_n }} · {{ row.address }} · {{ row.intertime }}</p>
      <button type="button" @click="cancel(row.id)">{{ $t('common.delete') }}</button>
    </article>
    <p v-if="!error && !(data?.list || []).length" class="muted">{{ $t('ui.no_items') }}</p>
    <h2>{{ $t('member_com_00512') }}</h2>
    <article v-for="row in (Array.isArray(tpls) ? tpls : tpls?.list || [])" :key="row.id" class="look_resume_list">
      <h3>{{ row.name }}</h3>
      <p class="muted">{{ row.address }} · {{ row.linkman }} {{ row.linktel }}</p>
      <button type="button" @click="fill(row)">{{ $t('common.edit') }}</button>
      <button type="button" @click="removeTpl(row.id)">{{ $t('common.delete') }}</button>
    </article>
    <form class="form" @submit.prevent="saveTpl">
      <input v-model="form.name" required :placeholder="$t('wap_com_00288')" />
      <textarea v-model="form.content" rows="3" required />
      <input v-model="form.address" required :placeholder="$t('wap_00040')" />
      <input v-model="form.linkman" required :placeholder="$t('common_02051')" />
      <input v-model="form.linktel" required :placeholder="$t('common.phone')" />
      <button type="submit">{{ form.id ? $t('common.save') : $t('common.submit') }}</button>
    </form>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
