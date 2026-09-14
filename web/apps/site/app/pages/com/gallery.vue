<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('com-gallery', () =>
  api.post('/v1/mcenter/galleries/list', { kind: 'company', page: 1, page_size: 20 }),
)
const title = ref('')
const msg = ref('')
async function onFile(ev: Event) {
  const file = (ev.target as HTMLInputElement).files?.[0]
  if (!file) return
  msg.value = ''
  try {
    const r = await $fetch<{ key: string; url: string }>('/api/upload/attachment', {
      method: 'POST',
      body: file,
      headers: { 'content-type': file.type || 'image/jpeg' },
    })
    await api.post('/v1/mcenter/galleries/create', {
      kind: 'company',
      title: title.value,
      picurl: r.key || r.url,
    })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function remove(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/galleries/delete', { kind: 'company', ids: [id] })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('wap_user_00157') })
</script>

<template>
  <MemberPanel :title="$t('wap_user_00157')" :error="error && !isUnauthErr(error) ? error : undefined">
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}</p>
    <form class="form verification_form" @submit.prevent>
      <MemberField :label="$t('wap_user_00103')"><input v-model="title" /></MemberField>
      <input type="file" accept="image/jpeg,image/png,image/webp" @change="onFile" />
    </form>
    <div v-for="row in data?.list || []" :key="row.id" class="user_resume_box">
      <h3>{{ row.title || row.id }}</h3>
      <img v-if="row.picurl" :src="row.picurl" alt="" width="120" />
      <a href="javascript:;" class="List_dete cblue" @click="remove(row.id)">{{ $t('common.delete') }}</a>
    </div>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
