<script setup lang="ts">
const uid = computed(() => String(useRoute().query.uid || ''))
const { t } = useI18n()
const api = useApi()
const { me } = useSiteChrome()
const { data: poster } = await useAsyncData(
  () => `invite-poster-${me.value?.uid || uid.value || 0}`,
  () =>
    api
      .post<{
        template?: { pic?: string; title?: string }
        qr_scene?: string
        fields?: Record<string, string>
      }>('/v1/wap/posters/invite-reg/me', {
        uid: Number(uid.value || me.value?.uid || 0) || undefined,
      })
      .catch(() => null),
)
useSeoMeta({ title: t('common.register') })
</script>

<template>
  <section class="site-inner">
    <h1>{{ $t('common.register') }}</h1>
    <p>
      <NuxtLink :to="{ path: '/register', query: uid ? { uid } : {} }">{{ $t('common.register') }}</NuxtLink>
    </p>
    <div v-if="poster?.template?.pic" class="stack">
      <img :src="poster.template.pic" alt="" style="max-width: 320px" />
      <p class="muted">{{ poster.template.title }} · {{ poster.qr_scene }}</p>
      <p v-for="(v, k) in poster.fields || {}" :key="k" class="muted">{{ k }}：{{ v }}</p>
    </div>
  </section>
</template>
