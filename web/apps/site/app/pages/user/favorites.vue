<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const kind = ref(1)
const { data, error, refresh } = await useAsyncData(
  () => `fav-${kind.value}`,
  () => api.post('/v1/mcenter/favorites/list', { kind: kind.value, page: 1, page_size: 20 }),
)
watch(kind, () => refresh())
async function remove(targetId: number) {
  await api.post('/v1/mcenter/favorites/remove', { kind: kind.value, target_id: targetId })
  refresh()
}
function titleOf(row: { target_id: number; detail?: Record<string, string> }) {
  const d = row.detail || {}
  return d.name || d.job_name || d.com_name || d.display_name || String(row.target_id)
}
useSeoMeta({ title: t('member_user_00103') })
</script>

<template>
  <MemberPanel
    :title="$t('member_user_00103')"
    :sub="$t('member_user_00102')"
    :error="error && !isUnauthErr(error) ? error : undefined"
    :empty="!error && !(data?.list || []).length"
    :empty-text="$t('ui.no_fav')"
    empty-to="/jobs"
    :empty-action="$t('wap_user_00254')"
  >
    <p v-if="error && isUnauthErr(error)" class="muted">{{ $t('wap_00376') }}</p>
    <div class="site-h5 m_tab">
      <div class="m_tabbox category">
        <ul>
          <li :class="{ m_tabactive: kind === 1 }" @click="kind = 1">{{ $t('common.job') }}</li>
          <li :class="{ m_tabactive: kind === 2 }" @click="kind = 2">{{ $t('common.company') }}</li>
          <li :class="{ m_tabactive: kind === 3 }" @click="kind = 3">{{ $t('ui.user_kind') }}</li>
        </ul>
      </div>
    </div>
    <p class="site-pc">
      <button type="button" :class="{ on: kind === 1 }" @click="kind = 1">{{ $t('common.job') }}</button>
      <button type="button" :class="{ on: kind === 2 }" @click="kind = 2">{{ $t('common.company') }}</button>
      <button type="button" :class="{ on: kind === 3 }" @click="kind = 3">{{ $t('ui.user_kind') }}</button>
    </p>
    <div v-if="(data?.list || []).length" class="user_new_listtit site-pc">
      <div class="user_new_job">{{ $t('member_user_00105') }}</div>
      <div class="user_new_cz">{{ $t('member_user_00048') }}</div>
    </div>
    <div v-for="row in data?.list || []" :key="row.target_id" class="jobnotice_list">
      <div class="user_new_job">
        <NuxtLink v-if="kind === 1" :to="`/jobs/${row.target_id}`" class="user_new_jobname">{{ titleOf(row) }}</NuxtLink>
        <NuxtLink v-else-if="kind === 2" :to="`/companies/${row.target_id}`" class="user_new_jobname">{{ titleOf(row) }}</NuxtLink>
        <span v-else class="user_new_jobname">{{ titleOf(row) }}</span>
      </div>
      <div class="user_new_cz">
        <a href="javascript:;" class="user_new_yqh_sc" @click="remove(row.target_id)">{{ $t('ui.unfav') }}</a>
      </div>
    </div>
  </MemberPanel>
</template>
