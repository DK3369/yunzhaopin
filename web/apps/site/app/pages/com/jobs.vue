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
  <section>
    <h1>{{ $t('ui.job_mgmt') }}</h1>
    <p v-if="error" class="muted">{{ $t('ui.please_login_com') }}</p>
    <p><NuxtLink to="/com/jobs/new">{{ $t('ui.publish_job') }}</NuxtLink></p>
    <p v-if="!error && !list.length" class="muted">{{ $t('ui.no_jobs') }}</p>
    <div class="stack">
      <article v-for="job in list" :key="job.id" class="job-card">
        <h3>{{ job.name }}</h3>
        <p class="muted">{{ $t('ui.status') }} {{ job.state }}</p>
      </article>
    </div>
  </section>
</template>
