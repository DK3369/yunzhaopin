<script setup lang="ts">
const { t } = useI18n()
const api = useApi()
const token = computed(() => String(useRoute().query.token || ''))
const msg = ref('')
onMounted(async () => {
  if (!token.value) {
    msg.value = t('common_00888')
    return
  }
  try {
    await api.post('/v1/wap/cert/email/verify', { token: token.value })
    msg.value = t('common.success')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('common_00888')
  }
})
useSeoMeta({ title: t('common.success') })
</script>

<template>
  <section class="site-inner">
    <h1>{{ $t('common.success') }}</h1>
    <p>{{ msg }}</p>
    <p><NuxtLink to="/login">{{ $t('common.login') }}</NuxtLink></p>
  </section>
</template>
