<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const kind = ref(1)
const { data, error, refresh } = await useAsyncData(
  () => `fav-${kind.value}`,
  () => api.post('/v1/mcenter/favorites/list', { kind: kind.value, page: 1, page_size: 20 }),
)
watch(kind, () => refresh())
async function remove(targetId: number) {
  await api.post('/v1/mcenter/favorites/remove', { kind: kind.value, target_id: targetId })
  refresh()
}
useSeoMeta({ title: t('ui.my_fav') })
</script>

<template>
  <section>
    <h1>{{ $t('ui.my_fav') }}</h1>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('wap_00376') : $t('ui.load_failed') }}</p>
    <p>
      <button type="button" @click="kind = 1">{{ $t('common.job') }}</button>
      <button type="button" @click="kind = 2">{{ $t('common.company') }}</button>
      <button type="button" @click="kind = 3">{{ $t('ui.user_kind') }}</button>
    </p>
    <p v-if="!(data?.list || []).length" class="muted">{{ $t('ui.no_fav') }}</p>
    <div class="stack">
      <article v-for="row in data?.list || []" :key="row.target_id" class="job-card">
        <h3>#{{ row.target_id }}</h3>
        <button type="button" @click="remove(row.target_id)">{{ $t('ui.unfav') }}</button>
      </article>
    </div>
  </section>
</template>
