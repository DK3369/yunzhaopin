<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('yqms', () =>
  api.post('/v1/mcenter/yqms/list', { page: 1, page_size: 20 }),
)
const openId = ref(0)
const rejectId = ref(0)
const remark = ref('')
const msg = ref('')
async function accept(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/yqms/accept', { id })
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function reject(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/yqms/reject', { id, remark: remark.value })
    rejectId.value = 0
    remark.value = ''
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function shield(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/blacklist', { yqms_id: id })
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function remove(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/yqms/delete', { id })
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
  <MemberPanel :title="$t('wap_user_00216')" :error="error" :empty="!error && !(data?.list || []).length">
    <div v-if="(data?.list || []).length" class="user_new_listtit site-pc">
      <div class="user_new_job">{{ $t('common.job') }}</div>
      <div class="user_new_time">{{ $t('wap_00040') }}</div>
      <div class="user_new_zt">{{ $t('member_user_00104') }}</div>
      <div class="user_new_cz">{{ $t('member_user_00048') }}</div>
    </div>
    <div v-for="row in data?.list || []" :key="row.id" class="jobnotice_list">
      <div class="user_new_job">
        <NuxtLink v-if="row.job_id" :to="`/jobs/${row.job_id}`" class="user_new_jobname">{{ row.job_name || $t('common.job') }}</NuxtLink>
        <span v-else class="user_new_jobname">{{ row.job_name || row.title || row.id }}</span>
        <div class="user_new_comname">
          <NuxtLink v-if="row.fid" :to="`/companies/${row.fid}`">{{ row.fname }}</NuxtLink>
        </div>
      </div>
      <div class="user_new_time">{{ row.intertime }} · {{ row.address }}</div>
      <div class="user_new_zt">{{ browseLabel(row.is_browse) }} · {{ row.datetime_n }}</div>
      <div class="user_new_cz">
        <template v-if="row.is_browse !== 3 && row.is_browse !== 4">
          <a href="javascript:;" class="user_new_yqh_sc" @click="accept(row.id)">{{ $t('wap_user_00262') }}</a>
          <a href="javascript:;" class="user_new_yqh_sc" @click="rejectId = row.id">{{ $t('wap_01053') }}</a>
        </template>
        <a href="javascript:;" class="user_new_yqh_sc" @click="openId = openId === row.id ? 0 : row.id">{{ $t('common.more') }}</a>
        <a href="javascript:;" class="user_new_yqh_sc" @click="shield(row.id)">{{ $t('wap_01060') }}</a>
        <a href="javascript:;" class="user_new_yqh_sc" @click="remove(row.id)">{{ $t('common.delete') }}</a>
      </div>
      <p v-if="openId === row.id" class="muted">{{ row.content }} · {{ row.linkman }} {{ row.linktel }}</p>
      <form v-if="rejectId === row.id" class="form" @submit.prevent="reject(row.id)">
        <input v-model="remark" :placeholder="$t('wap_01053')" />
        <button type="submit">{{ $t('common.submit') }}</button>
      </form>
    </div>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
