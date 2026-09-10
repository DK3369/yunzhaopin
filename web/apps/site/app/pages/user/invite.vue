<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const form = reactive({ email: '', content: t('ui.invite_body') })
const msg = ref('')
const { data: summary } = await useAsyncData('referrals-summary', () =>
  api.post<{ count?: number; total_points?: number }>('/v1/mcenter/referrals/summary', {}).catch(() => null),
)
const { data: refs } = await useAsyncData('referrals', () =>
  api.post('/v1/mcenter/referrals', { page: 1, page_size: 20 }).catch(() => null),
)
async function send() {
  msg.value = ''
  try {
    const r = await api.post<{ invite_id: number }>('/v1/mcenter/invite-reg', { ...form })
    msg.value = `${t('ui.send')} invite_id ${r.invite_id}`
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('ui.invite_reg') })
</script>

<template>
  <section>
    <h1>{{ $t('ui.invite_reg') }}</h1>
    <p v-if="summary" class="muted">
      {{ summary.count ?? 0 }} · {{ summary.total_points ?? 0 }}
    </p>
    <form class="form" @submit.prevent="send">
      <input v-model="form.email" type="email" :placeholder="$t('member_user_00282')" required />
      <textarea v-model="form.content" rows="4" :placeholder="$t('wap_user_00102')" />
      <button type="submit">{{ $t('ui.send_invite_reg') }}</button>
    </form>
    <h2>{{ $t('ui.flow') }}</h2>
    <p v-if="!(refs?.list || []).length" class="muted">{{ $t('ui.no_items') }}</p>
    <div class="stack">
      <article v-for="row in refs?.list || []" :key="row.id" class="job-card">
        <h3>{{ row.invitee_uid }}</h3>
        <p class="muted">{{ row.points }} · {{ row.created_at_n }}</p>
      </article>
    </div>
    <p v-if="msg">{{ msg }}</p>
  </section>
</template>
