<script setup lang="ts">
const id = Number(useRoute().params.id)
const { t } = useI18n()
const api = useApi()
const { data } = await useAsyncData(`hr-${id}`, () => api.get('/v1/wap/hr-docs/detail', { id }))
const msg = ref('')
async function download() {
  msg.value = ''
  try {
    const r = await api.post<{ url?: string; download_url?: string }>('/v1/wap/hr-docs/download', { id })
    const url = String(r.url || r.download_url || data.value?.url || '')
    if (url) {
      await navigateTo(url, { external: true })
      return
    }
    msg.value = t('common.confirm')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('common_00888')
  }
}
useSeoMeta({ title: () => String(data.value?.name || t('ui.hr')) })
useHead({ link: [{ rel: 'canonical', href: `/hr/${id}` }] })
</script>

<template>
  <article>
    <h1>{{ data?.name || $t('common_02409') }}</h1>
    <div v-if="data?.body" v-html="data.body" />
    <p v-else-if="!data?.name" class="muted">{{ $t('common_02409') }}</p>
    <p v-if="data?.name">
      <button type="button" @click="download">{{ $t('common.more') }}</button>
    </p>
    <p v-if="msg">{{ msg }}</p>
  </article>
</template>
