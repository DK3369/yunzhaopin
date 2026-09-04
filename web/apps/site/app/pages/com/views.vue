<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error } = await useAsyncData('com-profile-views', () =>
  api.post('/v1/mcenter/profile-views', { kind: 2, page: 1, page_size: 20 }),
)
useSeoMeta({ title: t('ui.who_viewed_com') })
</script>

<template>
  <section>
    <h1>{{ $t('ui.who_viewed_com') }}</h1>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('ui.please_login_com') : $t('ui.load_failed') }}</p>
    <p v-else-if="!(data?.list || []).length" class="muted">{{ $t('ui.no_views') }}</p>
    <div v-else class="stack">
      <article v-for="row in data?.list || []" :key="row.id" class="job-card">
        <h3>viewer_uid {{ row.viewer_uid }}</h3>
        <p class="muted">{{ row.datetime_n }}</p>
      </article>
    </div>
  </section>
</template>
