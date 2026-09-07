<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

type Row = {
  id: number
  r_name?: string
  username?: string
  r_reason?: string
  result?: string | null
  inputtime_n?: string
}

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('com-crm-reports', () =>
  api.post('/v1/mcenter/crm-reports/list', { page: 1, page_size: 20 }),
)
const list = computed(() => (data.value?.list || []) as Row[])
const reason = ref('')
const msg = ref('')

async function submit() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/crm-reports', { reason: reason.value.trim() })
    reason.value = ''
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.load_failed')
  }
}

async function remove(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/crm-reports/delete', { ids: [id] })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.load_failed')
  }
}

useSeoMeta({ title: t('member_com_00148') })
</script>

<template>
  <MemberPanel :title="$t('member_com_00148')" :error="error && !isUnauthErr(error) ? error : undefined">
    <p v-if="error && isUnauthErr(error)" class="muted">{{ $t('common_01153') }}</p>
    <form class="stack" @submit.prevent="submit">
      <textarea v-model="reason" rows="3" :placeholder="$t('member_com_00331')" maxlength="200" />
      <button type="submit">{{ $t('common.submit') }}</button>
    </form>
    <p v-if="!list.length" class="muted">{{ $t('member_com_00656') }}</p>
    <article v-for="row in list" :key="row.id" class="job-card">
      <p>{{ $t('member_com_00333') }} {{ row.r_name }}</p>
      <p>{{ $t('member_com_00331') }} {{ row.r_reason }}</p>
      <p class="muted">{{ row.inputtime_n }} · {{ row.result || $t('admin_user_00371') }}</p>
      <button type="button" @click="remove(row.id)">{{ $t('common.delete') }}</button>
    </article>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
