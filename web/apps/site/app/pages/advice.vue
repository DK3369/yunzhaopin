<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const form = reactive({ infotype: 'advice', content: '', moblie: '' })
const msg = ref('')
async function submit() {
  msg.value = ''
  try {
    await api.post('/v1/wap/advice', { ...form })
    msg.value = t('common.confirm')
    form.content = ''
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('common.no')
  }
}
useSeoMeta({ title: t('common.site_notice') })
</script>

<template>
  <section>
    <h1>{{ $t('common.site_notice') }}</h1>
    <form class="form" @submit.prevent="submit">
      <input v-model="form.infotype" />
      <textarea v-model="form.content" rows="5" required />
      <input v-model="form.moblie" :placeholder="$t('wap_01619')" />
      <button type="submit">{{ $t('common.submit') }}</button>
    </form>
    <p v-if="msg">{{ msg }}</p>
  </section>
</template>
