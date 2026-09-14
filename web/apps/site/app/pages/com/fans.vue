<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error } = await useAsyncData('com-followers', () =>
  api.post('/v1/mcenter/followers', { page: 1, page_size: 20 }),
)
useSeoMeta({ title: t('wap_com_00407') })
</script>

<template>
  <MemberPanel :title="$t('wap_com_00407')" :error="error && !isUnauthErr(error) ? error : undefined" :empty="!error && !(data?.list || []).length">
    <template #pcTabs><MemberHrTabs /></template>
    <template #h5Tabs><MemberHrTabs /></template>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}</p>
    <p v-else-if="!(data?.list || []).length" class="muted">{{ $t('ui.no_items') }}</p>
    <div v-else class="stack">
      <article v-for="row in data?.list || []" :key="row.id || row.uid" class="jobnotice_list">
        <h3>
          <NuxtLink :to="`/resumes/${row.uid}`">{{ row.uname || row.username || row.uid }}</NuxtLink>
        </h3>
        <p class="muted">{{ row.datetime_n || row.time }}</p>
      </article>
    </div>
  </MemberPanel>
</template>
