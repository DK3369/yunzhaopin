<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error } = await useAsyncData('com-zph', () => api.post('/v1/mcenter/zph/my-reservation', {}))
useSeoMeta({ title: t('wap_00558') })
</script>

<template>
  <MemberPanel :title="$t('wap_00558')" :error="error" :empty="!error && !(Array.isArray(data?.list) ? data.list.length : false)">
    <div v-for="row in data?.list || []" :key="row.id" class="issue_post_body_card">
      <div class="Posted_card_top">
        <NuxtLink v-if="row.zid" :to="`/fairs/${row.zid}?tab=reserve`" class="Posted_card_name">{{ row.title || row.name || row.zid }}</NuxtLink>
        <span v-else class="Posted_card_name">{{ row.title || row.name || row.id }}</span>
        <div class="Posted_card_pay">{{ row.start_at_n || row.datetime_n }}</div>
      </div>
    </div>
  </MemberPanel>
</template>
