<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const onlyUnanswered = ref(false)
const { data, error, refresh } = await useAsyncData('com-job-msg', () =>
  api.post('/v1/mcenter/job-messages', {
    page: 1,
    page_size: 20,
    only_unanswered: onlyUnanswered.value,
  }),
)
watch(onlyUnanswered, () => {
  refresh()
})
const msg = ref('')
const replyDraft = ref<Record<number, string>>({})

async function reply(id: number) {
  msg.value = ''
  const text = String(replyDraft.value[id] || '').trim()
  if (!text) {
    msg.value = t('ui.failed')
    return
  }
  try {
    await api.post('/v1/mcenter/job-messages/reply', { id, reply: text })
    replyDraft.value[id] = ''
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}

async function hide(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/job-messages/hide', { id })
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}

useSeoMeta({ title: t('member_user_00115') })
</script>

<template>
  <section>
    <h1>{{ $t('member_user_00115') }}</h1>
    <p>
      <label>
        <input v-model="onlyUnanswered" type="checkbox" />
        {{ $t('member_user_00481') }}
      </label>
    </p>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}</p>
    <p v-else-if="!(data?.list || []).length" class="muted">{{ $t('ui.no_items') }}</p>
    <div v-else class="stack">
      <article v-for="row in data?.list || []" :key="row.id" class="job-card">
        <h3>
          <NuxtLink v-if="row.jobid" :to="`/jobs/${row.jobid}`">{{ row.job_name || $t('common.job') }}</NuxtLink>
          <span v-else>{{ row.job_name || $t('common_02082') }}</span>
        </h3>
        <p class="muted">{{ row.username }} · {{ row.datetime_n }}</p>
        <p>{{ row.content }}</p>
        <p v-if="row.reply" class="muted">{{ row.reply }}</p>
        <form v-else class="form" @submit.prevent="reply(row.id)">
          <textarea v-model="replyDraft[row.id]" rows="2" required />
          <button type="submit">{{ $t('common.submit') }}</button>
        </form>
        <button type="button" @click="hide(row.id)">{{ $t('common.delete') }}</button>
      </article>
    </div>
    <p v-if="msg">{{ msg }}</p>
  </section>
</template>
