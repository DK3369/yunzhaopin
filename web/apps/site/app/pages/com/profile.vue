<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('com-profile', () =>
  api.post('/v1/mcenter/company/list', {}),
)
const form = reactive({
  name: '',
  shortname: '',
  content: '',
  linkman: '',
  linkphone: '',
  logo: '',
})
watch(
  data,
  (row) => {
    if (!row) return
    form.name = String(row.name || '')
    form.shortname = String(row.shortname || '')
    form.content = String(row.content || '')
    form.linkman = String(row.linkman || '')
    form.linkphone = String(row.linkphone || '')
    form.logo = String(row.logo || '')
  },
  { immediate: true },
)
const msg = ref('')
async function onLogo(ev: Event) {
  const file = (ev.target as HTMLInputElement).files?.[0]
  if (!file) return
  try {
    const r = await $fetch<{ key: string; url: string }>('/api/upload/company-logo', {
      method: 'POST',
      body: file,
      headers: { 'content-type': file.type || 'image/jpeg' },
    })
    form.logo = r.key || r.url
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function save() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/company', { ...form })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('member_com_00378') })
</script>

<template>
  <section>
    <h1>{{ $t('member_com_00378') }}</h1>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('ui.please_login_com') : $t('ui.load_failed') }}</p>
    <form v-else class="form" @submit.prevent="save">
      <input v-model="form.name" :placeholder="$t('ui.company_name')" />
      <input v-model="form.shortname" :placeholder="$t('ui.shortname')" />
      <input type="file" accept="image/jpeg,image/png,image/webp" @change="onLogo" />
      <textarea v-model="form.content" :placeholder="$t('ui.desc')" rows="6" />
      <input v-model="form.linkman" :placeholder="$t('ui.linkman')" />
      <input v-model="form.linkphone" :placeholder="$t('ui.linkphone')" />
      <button type="submit">{{ $t('common.save') }}</button>
      <p v-if="msg">{{ msg }}</p>
    </form>
  </section>
</template>
