<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { page, pageSize, inferTotal, go } = useMemberListPage()
const { data, error } = await useAsyncData(
  () => `resume-downloads-inbox-${page.value}`,
  () => api.post('/v1/mcenter/resume-downloads/inbox', { page: page.value, page_size: pageSize }),
)
const total = computed(() => inferTotal(data.value))
useSeoMeta({ title: t('admin_user_00263') })
</script>

<template>
  <MemberPanel :title="$t('admin_user_00263')" :error="error && !isUnauthErr(error) ? error : undefined" :empty="!error && !(data?.list || []).length">
    <p v-if="error && isUnauthErr(error)" class="muted">{{ $t('wap_00376') }}</p>
    <div class="job_list_tit">
      <ul>
        <li class="job_list_tit_cur"><a href="javascript:;">{{ $t('admin_user_00263') }}</a></li>
      </ul>
    </div>
    <div v-if="(data?.list || []).length" class="sysynews_tit site-pc">
      <div class="sysynews_span sysynews_name">{{ $t('common.company') }}</div>
      <div class="sysynews_span sysynews_time">{{ $t('member_user_00104') }}</div>
    </div>
    <div v-for="row in data?.list || []" :key="row.id" class="sysynews_list site-pc">
      <div class="sysynews_span sysynews_name">
        <NuxtLink v-if="row.com_id" :to="`/companies/${row.com_id}`">{{ row.uname || row.com_id }}</NuxtLink>
        <span v-else>{{ row.uname || row.com_id || row.id }}</span>
      </div>
      <div class="sysynews_span sysynews_time">{{ row.datetime_n }}</div>
    </div>
    <div class="site-h5 m_cardbox">
      <div class="m_cardbgbox">
        <MemberPostedCard
          v-for="row in data?.list || []"
          :key="'h5-' + row.id"
          :title="String(row.uname || row.com_id || row.id)"
          :time="row.datetime_n"
          :to="row.com_id ? `/companies/${row.com_id}` : undefined"
        />
      </div>
    </div>
    <MemberPager :page="page" :page-size="pageSize" :total="total" @update:page="go" />
  </MemberPanel>
</template>
