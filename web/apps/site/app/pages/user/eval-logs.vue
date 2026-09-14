<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { page, pageSize, inferTotal, go } = useMemberListPage()
const { data, error } = await useAsyncData(
  () => `eval-logs-mine-${page.value}`,
  () => api.post('/v1/mcenter/eval-logs', { page: page.value, page_size: pageSize }),
)
const total = computed(() => inferTotal(data.value))
useSeoMeta({ title: t('wap_00194') })
</script>

<template>
  <MemberPanel :title="$t('wap_00194')" :error="error && !isUnauthErr(error) ? error : undefined" :empty="!error && !(data?.list || []).length">
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('wap_00376') : $t('ui.load_failed') }}</p>
    <p v-else-if="!(data?.list || []).length" class="muted">{{ $t('ui.no_items') }}</p>
    <div v-else>
      <div v-for="row in data?.list || []" :key="row.id" class="jobnotice_list site-pc">
        <div class="user_new_job">
          <NuxtLink :to="`/user/eval-logs/${row.id}`" class="user_new_jobname">{{ row.paper_name || $t('wap_00194') }}</NuxtLink>
        </div>
        <div class="user_new_time">{{ row.score }} · {{ row.created_at_n }}</div>
      </div>
      <div class="site-h5 m_cardbox">
        <div class="m_cardbgbox">
          <MemberPostedCard
            v-for="row in data?.list || []"
            :key="'h5-' + row.id"
            :title="row.paper_name || $t('wap_00194')"
            :pay="String(row.score ?? '')"
            :time="row.created_at_n"
            :to="`/user/eval-logs/${row.id}`"
          />
        </div>
      </div>
      <MemberPager :page="page" :page-size="pageSize" :total="total" @update:page="go" />
    </div>
  </MemberPanel>
</template>
