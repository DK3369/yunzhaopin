<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { data, error } = await useAsyncData('com-cert', () => api.post('/v1/mcenter/company/cert/list', {}))
const form = reactive({ license_photo: '', id_photo: '', note: '' })
const msg = ref('')
async function upload(kind: 'license' | 'id', ev: Event) {
  const input = ev.target as HTMLInputElement
  const file = input.files?.[0]
  if (!file) return
  msg.value = ''
  try {
    const r = await $fetch<{ key: string; url: string }>(`/api/upload/cert`, {
      method: 'POST',
      body: file,
      headers: { 'content-type': file.type || 'image/jpeg' },
    })
    if (kind === 'license') form.license_photo = r.key || r.url
    else form.id_photo = r.key || r.url
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function submit() {
  try {
    await api.post('/v1/mcenter/company/cert', { ...form })
    msg.value = t('common.success')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('activate_00005') })
</script>

<template>
  <MemberPanel :title="$t('activate_00005')" :error="error">
    <p v-if="data" class="muted">{{ data.status_n || data.status }} {{ data.note }}</p>
    <form class="form" @submit.prevent="submit">
      <label>{{ $t('activate_00005') }}
        <input type="file" accept="image/jpeg,image/png,image/webp" @change="upload('license', $event)" />
      </label>
      <label>{{ $t('wap_00274') }}
        <input type="file" accept="image/jpeg,image/png,image/webp" @change="upload('id', $event)" />
      </label>
      <textarea v-model="form.note" />
      <button type="submit" :disabled="!form.license_photo || !form.id_photo">{{ $t('common.submit') }}</button>
    </form>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
