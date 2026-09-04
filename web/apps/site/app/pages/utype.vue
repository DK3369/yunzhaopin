<script setup lang="ts">
const { t } = useI18n()
const api = useApi()
const { me } = useSiteChrome()
const err = ref('')
async function pick(usertype: 1 | 2) {
  err.value = ''
  try {
    await api.post('/v1/wap/usertype/select', { usertype })
    const next = String(useRoute().query.next || '')
    if (next.startsWith('/') && !next.startsWith('//')) {
      await navigateTo(next)
      return
    }
    await navigateTo(usertype === 2 ? '/com' : '/user')
  } catch (e: unknown) {
    err.value = e instanceof Error ? e.message : t('common_00888')
  }
}
onMounted(async () => {
  if (!me.value) {
    await navigateTo('/login')
    return
  }
  if (me.value.usertype !== 0) {
    await navigateTo(me.value.usertype === 2 ? '/com' : '/user')
  }
})
useSeoMeta({ title: t('common.register') })
</script>

<template>
  <section class="site-inner">
    <h1>{{ $t('common.register') }}</h1>
    <p>
      <button type="button" @click="pick(1)">{{ $t('wap_00687') }}</button>
      <button type="button" @click="pick(2)">{{ $t('wap_00688') }}</button>
    </p>
    <p v-if="err">{{ err }}</p>
  </section>
</template>
