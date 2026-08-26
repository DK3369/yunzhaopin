<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { data, error } = await useAsyncData('com-cert', () => api.post('/v1/mcenter/company/cert/list', {}))
const form = reactive({ license_photo: '', id_photo: '', note: '' })
const msg = ref('')
async function submit() {
  try {
    await api.post('/v1/mcenter/company/cert', { ...form })
    msg.value = t('common.confirm')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('common.no')
  }
}
useSeoMeta({ title: t('activate_00005') })
</script>

<template>
  <MemberPanel :title="$t('activate_00005')" :error="error">
    <p v-if="data" class="muted">{{ data.status_n || data.status }} {{ data.note }}</p>
    <form class="form" @submit.prevent="submit">
      <input v-model="form.license_photo" :placeholder="$t('common.save')" />
      <input v-model="form.id_photo" />
      <textarea v-model="form.note" />
      <button type="submit">{{ $t('common.submit') }}</button>
    </form>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
