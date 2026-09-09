<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error } = await useAsyncData('com-zph', () => api.post('/v1/mcenter/zph/my-reservation', {}))
useSeoMeta({ title: t('wap_00558') })
</script>

<template>
  <MemberPanel :title="$t('wap_00558')" :error="error" :empty="!error && !(Array.isArray(data?.list) ? data.list.length : false)">
    <article v-for="row in data?.list || []" :key="row.id" class="look_resume_list">
      <h3>
        <NuxtLink v-if="row.zid" :to="`/fairs/${row.zid}?tab=reserve`">{{ row.title || row.name || row.zid }}</NuxtLink>
        <span v-else>{{ row.title || row.name || row.id }}</span>
      </h3>
      <p class="muted">{{ row.start_at_n || row.datetime_n }}</p>
      <p v-if="row.zid">
        <NuxtLink :to="`/fairs/${row.zid}?tab=reserve`">{{ $t('wap_01344') }}</NuxtLink>
      </p>
    </article>
  </MemberPanel>
</template>
