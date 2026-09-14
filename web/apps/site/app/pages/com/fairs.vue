<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error } = await useAsyncData('com-zph', () => api.post('/v1/mcenter/zph/my-reservation', {}))
useSeoMeta({ title: t('wap_00558') })
</script>

<template>
  <MemberPanel :title="$t('wap_00558')" :error="error" :empty="!error && !(Array.isArray(data?.list) ? data.list.length : false)">
    <div v-for="row in data?.list || []" :key="row.id" class="sysynews_list site-pc">
      <div class="sysynews_span sysynews_name">
        <NuxtLink v-if="row.zid" :to="`/fairs/${row.zid}?tab=reserve`">{{ row.title || row.name || row.zid }}</NuxtLink>
        <span v-else>{{ row.title || row.name || row.id }}</span>
      </div>
      <div class="sysynews_span sysynews_time">{{ row.start_at_n || row.datetime_n }}</div>
    </div>
    <div class="site-h5 m_cardbox">
      <MemberSxNewsCard
        v-for="row in data?.list || []"
        :key="'h5-' + row.id"
        :title="String(row.title || row.name || row.zid || row.id)"
        :time="row.start_at_n || row.datetime_n"
        :to="row.zid ? `/fairs/${row.zid}?tab=reserve` : undefined"
      />
    </div>
  </MemberPanel>
</template>
