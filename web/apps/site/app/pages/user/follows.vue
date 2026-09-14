<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const kind = ref(2)
const { data, error, refresh } = await useAsyncData(
  () => `follows-${kind.value}`,
  () => api.post('/v1/mcenter/follows/list', { kind: kind.value, page: 1, page_size: 20 }),
)
watch(kind, () => refresh())
async function toggle(row: { target_uid?: number; uid?: number; target_kind?: number }) {
  await api.post('/v1/mcenter/follows', {
    target_kind: row.target_kind || kind.value,
    target_uid: row.target_uid || row.uid,
  })
  refresh()
}
function nameOf(row: { name?: string; com_name?: string; target_uid?: number }) {
  return row.name || row.com_name || String(row.target_uid || '')
}
useSeoMeta({ title: t('wap_01142') })
</script>

<template>
  <MemberPanel :title="$t('wap_01142')" :error="error" :empty="!error && !(data?.list || []).length">
    <div class="site-h5 m_tab">
      <div class="m_tabbox category">
        <ul>
          <li :class="{ m_tabactive: kind === 2 }" @click="kind = 2">{{ $t('common.company') }}</li>
          <li :class="{ m_tabactive: kind === 1 }" @click="kind = 1">{{ $t('ui.user_kind') }}</li>
        </ul>
      </div>
    </div>
    <div class="site-pc job_list_tit">
      <ul>
        <li :class="{ job_list_tit_cur: kind === 2 }" @click="kind = 2">
          <a href="javascript:;">{{ $t('common.company') }}</a>
        </li>
        <li :class="{ job_list_tit_cur: kind === 1 }" @click="kind = 1">
          <a href="javascript:;">{{ $t('ui.user_kind') }}</a>
        </li>
      </ul>
    </div>
    <div v-if="(data?.list || []).length" class="user_new_listtit site-pc">
      <div class="user_new_job">{{ $t('common.company') }}</div>
      <div class="user_new_cz">{{ $t('member_user_00048') }}</div>
    </div>
    <div v-for="row in data?.list || []" :key="row.target_uid || row.uid" class="jobnotice_list site-pc">
      <div class="user_new_job">
        <NuxtLink v-if="kind === 2" :to="`/companies/${row.target_uid || row.uid}`" class="user_new_jobname">{{ nameOf(row) }}</NuxtLink>
        <span v-else class="user_new_jobname">{{ nameOf(row) }}</span>
      </div>
      <div class="user_new_cz">
        <a href="javascript:;" class="user_new_yqh_sc" @click="toggle(row)">{{ $t('common.delete') }}</a>
      </div>
    </div>
    <div class="site-h5 m_cardbox">
      <div class="m_cardbgbox">
        <MemberPostedCard
          v-for="row in data?.list || []"
          :key="'h5-' + (row.target_uid || row.uid)"
          :title="nameOf(row)"
          :to="kind === 2 ? `/companies/${row.target_uid || row.uid}` : undefined"
        />
      </div>
    </div>
  </MemberPanel>
</template>
