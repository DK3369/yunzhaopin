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
    <template v-else>
      <div class="job_list_tit">
        <ul>
          <li class="job_list_tit_cur"><a href="javascript:;">{{ $t('wap_00194') }}</a></li>
        </ul>
      </div>
      <div v-for="row in data?.list || []" :key="row.id" class="job_search_box site-pc">
        <div class="job_search_box_left">
          <div class="job_search_box_jobmane">
            <NuxtLink :to="`/user/eval-logs/${row.id}`" class="index_Job_Finder_cont_name_a">{{ row.paper_name || $t('wap_00194') }}</NuxtLink>
          </div>
          <div class="job_search_box_tj">{{ row.score }} · {{ row.created_at_n }}</div>
        </div>
      </div>
      <div class="site-h5 m_cardbox">
        <div class="m_cardbgbox">
          <MemberPostedCard
            v-for="row in data?.list || []"
            :key="'h5-' + row.id"
            variant="issue"
            :title="row.paper_name || $t('wap_00194')"
            :pay="String(row.score ?? '')"
            :time="row.created_at_n"
            :to="`/user/eval-logs/${row.id}`"
          />
        </div>
      </div>
      <MemberPager :page="page" :page-size="pageSize" :total="total" @update:page="go" />
    </template>
  </MemberPanel>
</template>
