<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { page, pageSize, inferTotal, go } = useMemberListPage()
const onlyUnanswered = ref(false)
const { data, error, refresh } = await useAsyncData(
  () => `com-job-msg-${page.value}-${onlyUnanswered.value}`,
  () =>
    api.post('/v1/mcenter/job-messages', {
      page: page.value,
      page_size: pageSize,
      only_unanswered: onlyUnanswered.value,
    }),
)
watch(onlyUnanswered, () => go(1))
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
const total = computed(() => inferTotal(data.value))
</script>

<template>
  <MemberPanel :title="$t('member_user_00115')" :error="error && !isUnauthErr(error) ? error : undefined">
    <MemberComScreen
      :tabs="[
        { value: false, label: $t('common.all'), on: !onlyUnanswered, select: () => (onlyUnanswered = false) },
        { value: true, label: $t('member_user_00481'), on: onlyUnanswered, select: () => (onlyUnanswered = true) },
      ]"
    />
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}</p>
    <div v-if="(data?.list || []).length" class="job_Consulting_tit site-pc">
      <span class="job_Consulting_span job_Consulting_jobname">{{ $t('wap_com_00288') }}</span>
      <span class="job_Consulting_span job_Consulting_comname">{{ $t('wap_00456') }}</span>
      <span class="job_Consulting_span job_Consulting_jobtime">{{ $t('member_user_00061') }}</span>
      <span class="job_Consulting_span job_Consulting_jobcz">{{ $t('member_user_00048') }}</span>
    </div>
    <div v-for="row in data?.list || []" :key="row.id" class="job_Consulting_list site-pc">
      <div class="job_Consulting_span job_Consulting_jobname">
        <NuxtLink v-if="row.jobid" :to="`/jobs/${row.jobid}`" class="job_Consulting_jobname_a">{{ row.job_name || $t('common.job') }}</NuxtLink>
        <span v-else>{{ row.job_name || $t('common_02082') }}</span>
      </div>
      <div class="job_Consulting_span job_Consulting_comname">{{ row.username }}</div>
      <div class="job_Consulting_span job_Consulting_jobtime">{{ row.datetime_n }}</div>
      <div class="job_Consulting_span job_Consulting_jobcz">
        <a href="javascript:;" class="List_dete cblue" @click="hide(row.id)">{{ $t('common.delete') }}</a>
      </div>
      <div class="job_Consulting_box">
        <div class="job_Consulting_my">
          <div class="job_Consulting_my_box">
            <span class="job_Consulting_my_ask">{{ $t('member_user_00480') }}</span>{{ row.content }}
          </div>
        </div>
        <div v-if="row.reply" class="job_Consulting_my">
          <div class="job_Consulting_com">
            <i class="job_Consulting_icon" />
            <div>{{ $t('wap_user_00155') }}：{{ row.reply }}</div>
          </div>
        </div>
        <form v-else class="job_Consulting_com" @submit.prevent="reply(row.id)">
          <textarea v-model="replyDraft[row.id]" rows="2" required />
          <button type="submit" class="verification_form_btn">{{ $t('common.submit') }}</button>
        </form>
      </div>
    </div>
    <div class="site-h5 m_cardbox">
      <div class="m_cardbgbox">
        <div v-for="row in data?.list || []" :key="'h5-' + row.id" class="issue_post_body_card">
          <div class="Posted_card_top">
            <div class="Posted_card_name">{{ row.job_name || $t('common.job') }}</div>
            <div class="Posted_card_pay">{{ row.username }}</div>
          </div>
          <div v-if="row.reply" class="job_Consulting_com">{{ $t('wap_user_00155') }}：{{ row.reply }}</div>
          <form v-else class="job_Consulting_com" @submit.prevent="reply(row.id)">
            <textarea v-model="replyDraft[row.id]" rows="2" required />
            <button type="submit" class="verification_form_btn">{{ $t('common.submit') }}</button>
          </form>
          <div class="Posted_card_time">{{ row.content }}</div>
        </div>
      </div>
    </div>
    <MemberPager :page="page" :page-size="pageSize" :total="total" @update:page="go" />
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
