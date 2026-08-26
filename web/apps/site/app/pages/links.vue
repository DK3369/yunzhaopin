<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { data } = await useAsyncData('links', () => api.get('/v1/wap/friend-links'))
const list = computed(() => (Array.isArray(data.value) ? data.value : []) as { id: number; name: string; url: string }[])
useSeoMeta({ title: t('ui.links') })
</script>

<template>
  <section>
    <h1>{{ $t('ui.links') }}</h1>
    <p v-if="!list.length" class="muted">{{ $t('ui.no_links') }}</p>
    <ul v-else class="stack">
      <li v-for="row in list" :key="row.id">
        <a :href="row.url" rel="nofollow noopener" target="_blank">{{ row.name }}</a>
      </li>
    </ul>
  </section>
</template>
