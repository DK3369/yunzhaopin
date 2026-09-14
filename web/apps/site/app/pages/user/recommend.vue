<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error } = await useAsyncData('recommend-jobs', () =>
  api.post('/v1/mcenter/recommend/jobs', { limit: 20 }).catch(() => []),
)
const list = computed(() => (Array.isArray(data.value) ? data.value : data.value?.list || data.value || []))
useSeoMeta({ title: t('wap_user_00211') })
</script>

<template>
  <MemberPanel :title="$t('wap_user_00211')" :error="error && !isUnauthErr(error) ? error : undefined" :empty="!error && !list.length">
    <p v-if="error && isUnauthErr(error)" class="muted">{{ $t('wap_00376') }}</p>
    <div class="stack">
      <article v-for="row in list" :key="row.id" class="jobnotice_list">
        <h3>
          <NuxtLink :to="`/jobs/${row.id}`">{{ row.name }}</NuxtLink>
        </h3>
        <p class="muted">
          <NuxtLink v-if="row.uid" :to="`/companies/${row.uid}`">{{ row.com_name }}</NuxtLink>
          · {{ row.min_salary }} - {{ row.max_salary }}
          <span v-if="row.pre"> · {{ row.pre }}%</span>
        </p>
      </article>
    </div>
  </MemberPanel>
</template>
