<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const kind = ref(2)
const { page, pageSize, inferTotal, go } = useMemberListPage()
watch(kind, () => go(1))
const { data, error, refresh } = await useAsyncData(
  () => `follows-${kind.value}-${page.value}`,
  () => api.post('/v1/mcenter/follows/list', { kind: kind.value, page: page.value, page_size: pageSize }),
)
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
const total = computed(() => inferTotal(data.value))
useSeoMeta({ title: t('wap_01142') })
</script>

<template>
  <MemberPanel
    :title="$t('wap_01142')"
    :error="error"
    :empty="!error && !(data?.list || []).length"
    empty-to="/companies"
    :empty-action="$t('common.search')"
  >
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
    <div v-if="(data?.list || []).length" class="attention_enterprises_tit site-pc">
      <div class="attention_enterprises_span attention_enterprises_name">{{ $t('wap_com_00157') }}</div>
      <div class="attention_enterprises_span attention_enterprises_job">{{ $t('member_user_00047') }}</div>
      <div class="attention_enterprises_span attention_enterprises_time">{{ $t('member_user_00046') }}</div>
      <div class="attention_enterprises_span attention_enterprises_cz">{{ $t('member_user_00048') }}</div>
    </div>
    <div v-for="row in data?.list || []" :key="row.target_uid || row.uid" class="attention_enterprises_list site-pc">
      <div class="attention_enterprises_span attention_enterprises_name">
        <NuxtLink v-if="kind === 2" :to="`/companies/${row.target_uid || row.uid}`" class="attention_enterprises_name_a">{{ nameOf(row) }}</NuxtLink>
        <span v-else class="attention_enterprises_name_a">{{ nameOf(row) }}</span>
        <div class="mt10">
          {{ row.com_pr }}
          <span v-if="row.com_pr && row.com_mun" class="look_myresume_comline">|</span>
          {{ row.com_mun }}
        </div>
      </div>
      <div class="attention_enterprises_span attention_enterprises_job mt15">
        <template v-if="row.jobnum || row.job_name">
          {{ row.jobname || row.job_name }}
          <a v-if="kind === 2" :href="`/companies/${row.target_uid || row.uid}`" class="attention_enterprises_job_n">{{ $t('common_02057') }}{{ row.jobnum || '' }}{{ $t('wap_user_00151') }}</a>
        </template>
        <template v-else>{{ $t('default_00033') }}</template>
      </div>
      <div class="attention_enterprises_span attention_enterprises_time mt15">{{ row.ctime_n || row.time_n }}</div>
      <div class="attention_enterprises_span attention_enterprises_cz">
        <a href="javascript:;" class="cblue" @click="toggle(row)">{{ $t('wap_js_00140') }}</a>
      </div>
    </div>
    <div class="site-h5 m_cardbox">
      <div class="m_cardbgbox">
        <MemberPostedCard
          v-for="row in data?.list || []"
          :key="'h5-' + (row.target_uid || row.uid)"
          variant="issue"
          :title="nameOf(row)"
          :sub="kind === 2 ? `${row.com_pr || ''} ${row.com_mun || ''}`.trim() : ''"
          :time="row.ctime_n || row.time_n"
          :to="kind === 2 ? `/companies/${row.target_uid || row.uid}` : undefined"
        >
          <div class="Posted_state_hrtip">
            <a href="javascript:;" @click.prevent="toggle(row)">{{ $t('wap_js_00140') }}</a>
          </div>
        </MemberPostedCard>
      </div>
    </div>
    <MemberPager :page="page" :page-size="pageSize" :total="total" @update:page="go" />
  </MemberPanel>
</template>
