<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('com-addresses', () =>
  api.post('/v1/mcenter/company-addresses', { page: 1, page_size: 20 }),
)
const form = reactive({
  link_man: '',
  link_moblie: '',
  link_phone: '',
  email: '',
  link_address: '',
  province_id: 0,
  city_id: 0,
  three_city_id: 0,
  x: '',
  y: '',
})
const msg = ref('')
async function create() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/company-addresses/create', { ...form })
    msg.value = t('ui.added')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('ui.map_addr') })
</script>

<template>
  <section>
    <h1>{{ $t('ui.map_addr') }}</h1>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}</p>
    <form class="form" @submit.prevent="create">
      <input v-model="form.link_man" :placeholder="$t('wap_01431')" />
      <input v-model="form.link_moblie" :placeholder="$t('common.phone')" />
      <input v-model="form.link_address" :placeholder="$t('ui.map_addr')" />
      <input v-model="form.x" :placeholder="$t('ui.lng')" />
      <input v-model="form.y" :placeholder="$t('ui.lat')" />
      <button type="submit">{{ $t('ui.add') }}</button>
    </form>
    <p v-if="msg">{{ msg }}</p>
    <p v-if="!(data?.list || []).length" class="muted">{{ $t('ui.no_addr') }}</p>
    <div class="stack">
      <article v-for="row in data?.list || []" :key="row.id" class="job-card">
        <h3>{{ row.link_man }} · {{ row.link_address }}</h3>
        <p class="muted">x {{ row.x }} y {{ row.y }}</p>
      </article>
    </div>
  </section>
</template>
