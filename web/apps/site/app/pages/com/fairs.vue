<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { data, error } = await useAsyncData('com-zph', () => api.post('/v1/mcenter/zph/my-reservation', {}))
const form = reactive({ id: 0, name: '', moblie: '', job_ids: '' })
const msg = ref('')
async function reserve() {
  try {
    await api.post('/v1/mcenter/zph/reserve', { ...form })
    msg.value = t('common.confirm')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('common.no')
  }
}
useSeoMeta({ title: t('wap_00223') })
</script>

<template>
  <section>
    <h1>{{ $t('wap_00223') }}</h1>
    <p v-if="error" class="muted">{{ $t('common.login') }}</p>
    <p v-if="data">{{ JSON.stringify(data) }}</p>
    <form class="form" @submit.prevent="reserve">
      <input v-model.number="form.id" placeholder="id" />
      <input v-model="form.name" :placeholder="$t('common.confirm')" />
      <input v-model="form.moblie" :placeholder="$t('common.phone')" />
      <button type="submit">{{ $t('common.submit') }}</button>
    </form>
    <p v-if="msg">{{ msg }}</p>
  </section>
</template>
