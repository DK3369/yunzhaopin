<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('com-banners', () =>
  api.post<Array<Record<string, unknown>>>('/v1/mcenter/company-banners/list', {}),
)
const form = reactive({ pic: '', link: '', sort: 0 })
const msg = ref('')
async function add() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/company-banners', { ...form })
    form.pic = ''
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function remove(row: { id: number }) {
  await api.post('/v1/mcenter/company-banners/delete', { ids: [row.id] })
  await refresh()
}
useSeoMeta({ title: t('ui.com_banner') })
</script>

<template>
  <section>
    <h1>{{ $t('ui.com_banner') }}</h1>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('ui.please_login_com') : $t('ui.load_failed') }}</p>
    <form v-else class="form" @submit.prevent="add">
      <input v-model="form.pic" :placeholder="$t('ui.image')" />
      <input v-model="form.link" :placeholder="$t('ui.link')" />
      <button type="submit">{{ $t('ui.add') }}</button>
      <p v-if="msg">{{ msg }}</p>
    </form>
    <div class="stack">
      <article v-for="row in data || []" :key="row.id">
        <p>{{ row.pic }}</p>
        <button type="button" @click="remove(row)">{{ $t('common.delete') }}</button>
      </article>
    </div>
    <p><NuxtLink to="/com">{{ $t('ui.back_com') }}</NuxtLink></p>
  </section>
</template>
