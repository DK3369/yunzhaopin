<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('blacklist', () =>
  api.post('/v1/mcenter/blacklist/list', { page: 1, page_size: 20 }),
)
const uid = ref(0)
const msg = ref('')
async function add() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/blacklist', { blocked_uid: uid.value })
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
    <form class="form" @submit.prevent="add">
      <input v-model.number="uid" type="number" min="1" required />
      <button type="submit">{{ $t('common.submit') }}</button>
    </form>
    <p v-if="!(data?.list || []).length" class="muted">{{ $t('ui.no_items') }}</p>
    <article v-for="row in data?.list || []" :key="row.id" class="job-card">
      <h3>uid {{ row.blocked_uid }}</h3>
      <p class="muted">{{ row.reason }} {{ row.created_at_n }}</p>
      <button type="button" @click="remove(row.blocked_uid)">{{ $t('common.delete') }}</button>
    </article>
    <p v-if="msg">{{ msg }}</p>
  </section>
</template>
