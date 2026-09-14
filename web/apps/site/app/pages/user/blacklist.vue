<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('blacklist', () =>
  api.post('/v1/mcenter/blacklist/list', { page: 1, page_size: 20 }),
)
const keyword = ref('')
const hits = ref<Array<{ uid: number; name?: string }>>([])
const msg = ref('')
async function search() {
  msg.value = ''
  try {
    const r = await api.get<{ list: Array<{ uid: number; name?: string }> }>('/v1/wap/companies', {
      keyword: keyword.value,
      page: 1,
      page_size: 10,
    })
    hits.value = r.list || []
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function add(uid: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/blacklist', { blocked_uid: uid })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function remove(blockedUid: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/blacklist/remove', { uid: blockedUid })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('member_user_00044') })
</script>

<template>
  <MemberPanel :title="$t('member_user_00044')" :error="error && !isUnauthErr(error) ? error : undefined">
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('wap_00376') : $t('ui.load_failed') }}</p>
    <form class="form verification_form" @submit.prevent="search">
      <MemberField :label="$t('wap_com_00157')">
        <input v-model="keyword" required />
      </MemberField>
      <button type="submit" class="verification_form_btn">{{ $t('wap_js_00086') }}</button>
    </form>
    <div v-for="row in hits" :key="row.uid" class="jobnotice_list">
      <div class="user_new_job">
        <span class="user_new_jobname">{{ row.name || row.uid }}</span>
      </div>
      <div class="user_new_cz">
        <a href="javascript:;" class="user_new_yqh_a" @click="add(row.uid)">{{ $t('wap_01060') }}</a>
      </div>
    </div>
    <div v-for="row in data?.list || []" :key="row.id" class="jobnotice_list site-pc">
      <div class="user_new_job">
        <NuxtLink :to="`/companies/${row.blocked_uid}`" class="user_new_jobname">{{ row.com_name || row.reason || row.blocked_uid }}</NuxtLink>
      </div>
      <div class="user_new_time">{{ row.created_at_n }}</div>
      <div class="user_new_cz">
        <a href="javascript:;" class="user_new_yqh_sc" @click="remove(row.blocked_uid)">{{ $t('common.delete') }}</a>
      </div>
    </div>
    <div class="site-h5 m_cardbox">
      <div class="m_cardbgbox">
        <MemberPostedCard
          v-for="row in data?.list || []"
          :key="'h5-' + row.id"
          :title="String(row.com_name || row.reason || row.blocked_uid)"
          :time="row.created_at_n"
          :to="`/companies/${row.blocked_uid}`"
        />
      </div>
    </div>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
