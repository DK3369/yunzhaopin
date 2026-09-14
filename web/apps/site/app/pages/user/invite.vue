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
const rows = computed(() => refs.value?.list || [])
</script>

<template>
  <MemberPanel :title="$t('ui.invite_reg')" user-wrap="none" :empty="false">
    <div class="site-h5 yun_usermember_financebg">
      <div class="yun_usermember_integral">
        {{ $t('ui.invite_reg') }}
        <span class="yun_usermember_integral_n">{{ summary?.count ?? 0 }}</span>
      </div>
    </div>
    <div class="site-pc resume_Prompt_box">
      <div class="resume_Prompt">
        <i class="resume_Prompt_icon" />
        {{ $t('ui.invite_reg') }} {{ summary?.count ?? 0 }} · {{ summary?.total_points ?? 0 }}
      </div>
    </div>
    <form class="form verification_form" @submit.prevent="send">
      <MemberField :label="$t('member_user_00282')">
        <input v-model="form.email" type="email" required />
      </MemberField>
      <MemberField :label="$t('wap_user_00102')" area>
        <textarea v-model="form.content" rows="4" />
      </MemberField>
      <button type="submit" class="verification_form_btn site-h5">{{ $t('ui.send_invite_reg') }}</button>
      <input type="submit" class="uesr_submit site-pc" :value="$t('ui.send_invite_reg')" />
    </form>
    <div v-for="row in rows" :key="row.id" class="job_search_box site-pc">
      <div class="job_search_box_left">
        <div class="job_search_box_jobmane">{{ row.invitee_uid }}</div>
        <div class="job_search_box_tj">{{ row.points }} · {{ row.created_at_n }}</div>
      </div>
    </div>
    <div class="site-h5 m_cardbox">
      <div class="m_cardbgbox">
        <MemberPostedCard
          v-for="row in rows"
          :key="'h5-' + row.id"
          variant="issue"
          :title="String(row.invitee_uid)"
          :pay="String(row.points ?? '')"
          :time="row.created_at_n"
        />
      </div>
    </div>
    <MemberPager :page="page" :page-size="pageSize" :total="total" @update:page="go" />
    <p v-if="msg" class="muted">{{ msg }}</p>
  </MemberPanel>
</template>
