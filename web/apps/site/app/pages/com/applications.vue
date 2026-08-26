<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('com-apps', () =>
  api.post('/v1/mcenter/applications', { page: 1, page_size: 20 }),
)
const list = computed(() => data.value?.list || [])
const invite = reactive({
  apply_id: 0,
  inter_time: Math.floor(Date.now() / 1000) + 86400,
  address: '',
  linkman: '',
  linktel: '',
  remark: '',
})
const msg = ref('')
function pick(id: number) {
  invite.apply_id = id
}
async function sendInvite() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/company/interviews/create', { ...invite })
    msg.value = t('ui.send_invite')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('ui.received_resumes') })
</script>

<template>
  <section>
    <h1>{{ $t('ui.received_resumes') }}</h1>
    <p v-if="error" class="muted">{{ $t('ui.please_login_com') }}</p>
    <p v-else-if="!list.length" class="muted">{{ $t('ui.no_applies') }}</p>
    <div class="stack">
      <article v-for="row in list" :key="row.id" class="job-card">
        <h3>{{ $t('ui.apply_id') }} #{{ row.id }} · {{ $t('ui.seeker') }} {{ row.uid }}</h3>
        <p class="muted">{{ $t('common.job') }} {{ row.job_id }} · {{ row.datetime_n }}</p>
        <button type="button" @click="pick(row.id)">{{ $t('ui.invite_interview') }}</button>
      </article>
    </div>
    <h2>{{ $t('ui.send_invite') }}</h2>
    <form class="form" @submit.prevent="sendInvite">
      <input v-model.number="invite.apply_id" type="number" :placeholder="$t('ui.apply_id')" />
      <input v-model.number="invite.inter_time" type="number" :placeholder="$t('ui.interview_unix')" />
      <input v-model="invite.address" :placeholder="$t('ui.interview_place')" />
      <input v-model="invite.linkman" :placeholder="$t('ui.linkman')" />
      <input v-model="invite.linktel" :placeholder="$t('ui.linkphone')" />
      <input v-model="invite.remark" :placeholder="$t('ui.remark')" />
      <button type="submit">{{ $t('ui.send_invite') }}</button>
      <p v-if="msg">{{ msg }}</p>
    </form>
  </section>
</template>
