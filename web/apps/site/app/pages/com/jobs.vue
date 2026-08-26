<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { data, error } = await useAsyncData('com-jobs', () =>
  api.post('/v1/mcenter/jobs/list', { page: 1, page_size: 20 }),
)
const list = computed(() => data.value?.list || [])
useSeoMeta({ title: t('ui.job_mgmt') })
</script>

<template>
  <MemberPanel :title="$t('ui.job_mgmt')" :error="error" :empty="!error && !list.length">
    <p><NuxtLink to="/com/jobs/new">{{ $t('ui.publish_job') }}</NuxtLink></p>
    <article v-for="job in list" :key="job.id" class="look_resume_list">
      <h3>{{ job.name }}</h3>
      <p class="muted">{{ $t('ui.status') }} {{ job.state }}</p>
    </article>
  </MemberPanel>
</template>
