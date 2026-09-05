<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('blacklist', () =>
  api.post('/v1/mcenter/blacklist/list', { page: 1, page_size: 20 }),
)
const keyword = ref('')
const hits = ref<Array<{ uid: number; name?: string }>>([])
const msg = ref('')
async function search() {
  msg.value = ''
  try {
    const r = await api.get<{ list: Array<{ uid: number; name?: string }> }>('/v1/wap/companies', {
      keyword: keyword.value,
      page: 1,
      page_size: 10,
    })
    hits.value = r.list || []
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function add(uid: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/blacklist', { blocked_uid: uid })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function remove(blockedUid: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/blacklist/remove', { uid: blockedUid })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('member_user_00044') })
</script>

<template>
  <section>
    <h1>{{ $t('member_user_00044') }}</h1>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('wap_00376') : $t('ui.load_failed') }}</p>
    <form class="form" @submit.prevent="search">
      <input v-model="keyword" required :placeholder="$t('wap_com_00157')" />
      <button type="submit">{{ $t('wap_js_00086') }}</button>
    </form>
    <article v-for="row in hits" :key="row.uid" class="job-card">
      <h3>{{ row.name || row.uid }}</h3>
      <button type="button" @click="add(row.uid)">{{ $t('wap_01060') }}</button>
    </article>
    <p v-if="!(data?.list || []).length" class="muted">{{ $t('ui.no_items') }}</p>
    <article v-for="row in data?.list || []" :key="row.id" class="job-card">
      <h3>
        <NuxtLink :to="`/companies/${row.blocked_uid}`">{{ row.com_name || row.reason || row.blocked_uid }}</NuxtLink>
      </h3>
      <p class="muted">{{ row.created_at_n }}</p>
      <button type="button" @click="remove(row.blocked_uid)">{{ $t('common.delete') }}</button>
    </article>
    <p v-if="msg">{{ msg }}</p>
  </section>
</template>
