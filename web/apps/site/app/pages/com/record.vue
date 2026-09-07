<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

type Row = {
  id: number
  eid?: number
  uid?: number
  user_name?: string
  job_name?: string
  ctime_n?: string
}

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('com-entrust-records', () =>
  api.post('/v1/mcenter/entrust-records/list', { page: 1, page_size: 20 }),
)
const list = computed(() => (data.value?.list || []) as Row[])
const msg = ref('')

async function remove(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/entrust-records/delete', { ids: [id] })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.load_failed')
  }
}

useSeoMeta({ title: t('member_com_00555') })
</script>

<template>
  <MemberPanel :title="$t('member_com_00555')" :error="error && !isUnauthErr(error) ? error : undefined" :empty="!error && !list.length">
    <p v-if="error && isUnauthErr(error)" class="muted">{{ $t('common_01153') }}</p>
    <article v-for="row in list" :key="row.id" class="look_resume_list">
      <h3>
        <NuxtLink :to="`/resumes/${row.eid || row.uid}`">{{ row.user_name || row.eid || row.uid }}</NuxtLink>
      </h3>
      <p class="muted">{{ row.job_name }} · {{ row.ctime_n }}</p>
      <p>
        <NuxtLink :to="`/resumes/${row.eid || row.uid}`">{{ $t('wap_user_00216') }}</NuxtLink>
        <button type="button" @click="remove(row.id)">{{ $t('common.delete') }}</button>
      </p>
    </article>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
