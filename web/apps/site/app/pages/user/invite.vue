<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const form = reactive({ email: '', content: t('ui.invite_body') })
const msg = ref('')
const { data: summary } = await useAsyncData('referrals-summary', () =>
  api.post<{ count?: number; total_points?: number }>('/v1/mcenter/referrals/summary', {}).catch(() => null),
)
const { page, pageSize, inferTotal, go } = useMemberListPage()
const { data: refs } = await useAsyncData(
  () => `referrals-${page.value}`,
  () => api.post('/v1/mcenter/referrals', { page: page.value, page_size: pageSize }).catch(() => null),
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
const total = computed(() => inferTotal(refs.value))
</script>

<template>
  <MemberPanel :title="$t('ui.invite_reg')">
    <div class="yun_usermember_integral_box site-h5">
      <div class="yun_usermember_integral_box_h1">{{ $t('ui.invite_reg') }}</div>
    </div>
    <p v-if="summary" class="muted">
      {{ summary.count ?? 0 }} · {{ summary.total_points ?? 0 }}
    </p>
    <form class="form verification_form" @submit.prevent="send">
      <MemberField :label="$t('member_user_00282')">
        <input v-model="form.email" type="email" required />
      </MemberField>
      <MemberField :label="$t('wap_user_00102')" area>
        <textarea v-model="form.content" rows="4" />
      </MemberField>
      <button type="submit" class="verification_form_btn">{{ $t('ui.send_invite_reg') }}</button>
    </form>
    <div v-for="row in refs?.list || []" :key="row.id" class="attention_enterprises_list site-pc">
      <div class="attention_enterprises_span attention_enterprises_name">{{ row.invitee_uid }}</div>
      <div class="attention_enterprises_span attention_enterprises_time">{{ row.points }} · {{ row.created_at_n }}</div>
    </div>
    <MemberPager :page="page" :page-size="pageSize" :total="total" @update:page="go" />
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
