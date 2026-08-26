<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { data, error } = await useAsyncData('profile-views', () =>
  api.post('/v1/mcenter/profile-views', { kind: 3, page: 1, page_size: 20 }),
)
useSeoMeta({ title: t('ui.who_viewed_me') })
</script>

<template>
  <section>
    <h1>{{ $t('ui.who_viewed_me') }}</h1>
    <p v-if="error" class="muted">{{ $t('wap_00376') }}</p>
    <p v-else-if="!(data?.list || []).length" class="muted">{{ $t('ui.no_who_viewed') }}</p>
    <div v-else class="stack">
      <article v-for="row in data?.list || []" :key="row.id" class="job-card">
        <h3>viewer_uid {{ row.viewer_uid }}</h3>
        <p class="muted">kind {{ row.kind }} · {{ row.datetime_n }}</p>
      </article>
    </div>
  </section>
</template>
