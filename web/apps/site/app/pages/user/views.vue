<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error } = await useAsyncData('profile-views', () =>
  api.post('/v1/mcenter/profile-views', { kind: 3, page: 1, page_size: 20 }),
)
useSeoMeta({ title: t('wap_user_00276') })
</script>

<template>
  <section>
    <h1>{{ $t('wap_user_00276') }}</h1>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('wap_00376') : $t('ui.load_failed') }}</p>
    <p v-else-if="!(data?.list || []).length" class="muted">{{ $t('ui.no_who_viewed') }}</p>
    <div v-else class="stack">
      <article v-for="row in data?.list || []" :key="row.id" class="job-card">
        <h3>viewer_uid {{ row.viewer_uid }}</h3>
        <p class="muted">kind {{ row.kind }} · {{ row.datetime_n }}</p>
      </article>
    </div>
  </section>
</template>
