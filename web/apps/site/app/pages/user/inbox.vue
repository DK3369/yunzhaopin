<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error } = await useAsyncData('resume-downloads-inbox', () =>
  api.post('/v1/mcenter/resume-downloads/inbox', { page: 1, page_size: 20 }),
)
useSeoMeta({ title: t('admin_user_00263') })
</script>

<template>
  <MemberPanel :title="$t('admin_user_00263')" :error="error && !isUnauthErr(error) ? error : undefined" :empty="!error && !(data?.list || []).length">
    <p v-if="error && isUnauthErr(error)" class="muted">{{ $t('wap_00376') }}</p>
    <div class="stack">
      <article v-for="row in data?.list || []" :key="row.id" class="jobnotice_list">
        <h3>
          <NuxtLink v-if="row.com_id" :to="`/companies/${row.com_id}`">{{ row.uname || row.com_id }}</NuxtLink>
          <span v-else>{{ row.uname || row.com_id || row.id }}</span>
        </h3>
        <p class="muted">{{ row.datetime_n }}</p>
      </article>
    </div>
  </MemberPanel>
</template>
