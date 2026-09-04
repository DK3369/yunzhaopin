<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('com-news', () =>
  api.post('/v1/mcenter/company/news/list', { page: 1, page_size: 20 }),
)
const form = reactive({ title: '', body: '', sort: 0 })
const msg = ref('')
const list = computed(() => data.value?.list || [])
async function add() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/company/news', { ...form })
    form.title = ''
    form.body = ''
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('ui.com_news') })
</script>

<template>
  <section>
    <h1>{{ $t('ui.com_news') }}</h1>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}</p>
    <form v-else class="form" @submit.prevent="add">
      <input v-model="form.title" :placeholder="$t('wap_user_00103')" />
      <textarea v-model="form.body" :placeholder="$t('ui.body')" rows="4" />
      <button type="submit">{{ $t('common.publish') }}</button>
      <p v-if="msg">{{ msg }}</p>
    </form>
    <div class="stack">
      <article v-for="row in list" :key="row.id">
        <h3>{{ row.title }}</h3>
        <p class="muted">status {{ row.status }}</p>
      </article>
    </div>
    <p><NuxtLink to="/com">{{ $t('ui.back_com') }}</NuxtLink></p>
  </section>
</template>
