<script setup lang="ts">
const uid = computed(() => String(useRoute().query.uid || ''))
const { t } = useI18n()
useSeoMeta({ title: t('common.register') })
</script>

<template>
  <section class="site-inner">
    <h1>{{ $t('common.register') }}</h1>
    <p>
      <NuxtLink :to="{ path: '/register', query: uid ? { uid } : {} }">{{ $t('common.register') }}</NuxtLink>
    </p>
  </section>
</template>
