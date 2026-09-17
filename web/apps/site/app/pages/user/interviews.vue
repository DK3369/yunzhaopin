<script setup lang="ts">
const api = useApi()
const { t } = useI18n()
const { page, pageSize, inferTotal, go } = useMemberListPage()
const { data, error, refresh } = await useAsyncData(
  () => `yqms-${page.value}`,
  () => api.post('/v1/mcenter/yqms/list', { page: page.value, page_size: pageSize }),
)
const openId = ref(0)
const rejectId = ref(0)
const reviewId = ref(0)
const remark = ref('')
const reviewComment = ref('')
const reviewScores = reactive({ desscore: 5, comscore: 5, hrscore: 5 })
const reviewView = ref<{ submitted?: boolean; comment?: string; dimensions?: Array<{ key: string; score: number }>; total?: number } | null>(null)
const msg = ref('')
async function accept(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/yqms/accept', { id })
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function reject(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/yqms/reject', { id, remark: remark.value })
    rejectId.value = 0
    remark.value = ''
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function shield(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/blacklist', { yqms_id: id })
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function remove(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/yqms/delete', { id })
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function openReview(id: number) {
  reviewId.value = id
  reviewComment.value = ''
  reviewScores.desscore = 5
  reviewScores.comscore = 5
  reviewScores.hrscore = 5
  reviewView.value = null
  try {
    const r = await api.post<{ submitted?: boolean; comment?: string; dimensions?: Array<{ key: string; score: number }>; total?: number }>(
      '/v1/mcenter/interviews/review',
      { yqms_id: id },
    )
    reviewView.value = r
    if (r?.submitted) {
      for (const d of r.dimensions || []) {
        if (d.key in reviewScores) (reviewScores as Record<string, number>)[d.key] = d.score
      }
      reviewComment.value = r.comment || ''
    }
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function submitReview() {
  msg.value = ''
  try {
    reviewView.value = await api.post('/v1/mcenter/interviews/review/submit', {
      yqms_id: reviewId.value,
      dimensions: [
        { key: 'desscore', score: reviewScores.desscore },
        { key: 'comscore', score: reviewScores.comscore },
        { key: 'hrscore', score: reviewScores.hrscore },
      ],
      comment: reviewComment.value,
    })
    msg.value = t('common.success')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
function browseLabel(state?: number) {
  if (state === 3) return t('wap_com_00190')
  if (state === 4) return t('wap_user_00257')
  if (state === 2) return t('wap_user_00258')
  return t('wap_user_00260')
}
const total = computed(() => inferTotal(data.value))
useSeoMeta({ title: t('wap_user_00216') })
</script>

<template>
  <MemberPanel
    :title="$t('wap_user_00216')"
    :sub="$t('member_user_00102')"
    :error="error"
    :empty="!error && !(data?.list || []).length"
    empty-to="/jobs"
    :empty-action="$t('wap_user_00254')"
  >
    <div v-if="(data?.list || []).length" class="user_new_listtit site-pc">
      <div class="user_new_job">{{ $t('common.job') }}</div>
      <div class="user_new_time">{{ $t('wap_00040') }}</div>
      <div class="user_new_zt">{{ $t('member_user_00104') }}</div>
      <div class="user_new_yqh">{{ $t('member_user_00107') }}</div>
      <div class="user_new_cz">{{ $t('member_user_00048') }}</div>
    </div>
    <div v-for="row in data?.list || []" :key="row.id" class="jobnotice_list site-pc">
      <div class="user_new_job">
        <NuxtLink v-if="row.job_id" :to="`/jobs/${row.job_id}`" class="user_new_jobname">{{ row.job_name || $t('common.job') }}</NuxtLink>
        <span v-else class="user_new_jobname">{{ row.job_name || row.title || row.id }}</span>
        <div v-if="row.salary" class="user_new_jobxz">{{ row.salary }}</div>
        <div class="user_new_comname">
          <NuxtLink v-if="row.fid" :to="`/companies/${row.fid}`">{{ row.fname }}</NuxtLink>
        </div>
      </div>
      <div class="user_new_time">
        <span class="msg_zt_s">{{ row.datetime_n }}</span>
      </div>
      <div class="user_new_zt">
        <template v-if="row.is_browse === 3">{{ $t('wap_com_00190') }}</template>
        <template v-else-if="row.is_browse === 4">
          <span class="msg_zt_s">{{ $t('wap_user_00257') }}</span>
        </template>
        <template v-else-if="row.is_browse !== 3 && row.is_browse !== 4">
          <a href="javascript:;" class="user_new_bth" @click="accept(row.id)">{{ $t('wap_user_00262') }}</a>
          <a href="javascript:;" class="user_new_btjh" @click="rejectId = row.id">{{ $t('wap_01053') }}</a>
        </template>
      </div>
      <div class="user_new_yqh">
        <a href="javascript:;" class="user_new_yqh_a" @click="openId = openId === row.id ? 0 : row.id">{{ $t('member_user_00107') }}</a>
      </div>
      <div class="user_new_cz">
        <a href="javascript:;" class="user_new_yqh_sc" @click="remove(row.id)">{{ $t('common.delete') }}</a>
        <a v-if="row.is_browse === 3" href="javascript:;" class="user_new_yqh_a" @click="openReview(row.id)">{{ $t('common_01866') }}</a>
      </div>
      <div v-if="openId === row.id" class="invitation_cont site-pc">
        <div class="invitation_user">{{ $t('wap_00529') }} <span class="invitation_user_name">{{ row.username || row.fname }}</span></div>
        <div class="invitation_cont">
          <i class="invitation_cont_job">{{ row.job_name }}</i>
        </div>
        <div class="invitation_cont_tip">{{ $t('member_user_00107') }}</div>
        <div class="invitation_cont_p"><span class="invitation_cont_pn">{{ $t('wap_user_00255') }}</span><em class="audition_list_e">{{ row.intertime }}</em></div>
        <div class="invitation_cont_p"><span class="invitation_cont_pn">{{ $t('wap_user_00243') }}</span><em class="audition_list_e">{{ row.address }}</em></div>
        <div v-if="row.content" class="invitation_cont_p nocontent"><span class="invitation_cont_pn">{{ $t('ui.detail') }}</span><em>{{ row.content }}</em></div>
        <div class="invitation_cont_p">
          <span class="invitation_cont_pn">{{ $t('common_02051') }}</span>
          <em class="audition_list_e">{{ row.linkman }}</em>
          TEL：<em class="invitation_cont_tel">{{ row.linktel }}</em>
        </div>
        <div class="invitation_cont_jy">
          <div class="invitation_cont_d">{{ row.fname }}</div>
          <div class="invitation_cont_d">{{ row.datetime_n }}</div>
        </div>
      </div>
      <form v-if="rejectId === row.id" class="invite_no" @submit.prevent="reject(row.id)">
        <textarea v-model="remark" class="invite_notextarea" :placeholder="$t('wap_01053')" />
        <div class="invite_nobth">
          <button type="submit" class="invite_nobth_bth">{{ $t('common.confirm') }}</button>
        </div>
      </form>
    </div>
    <div class="site-h5 m_cardbox">
      <div class="m_cardbgbox">
        <div v-for="row in data?.list || []" :key="'h5-' + row.id" class="Posted_body_card">
          <div class="Posted_card_top" @click="openId = openId === row.id ? 0 : row.id">
            <div class="Posted_card_name">
              <i v-if="row.is_browse === 1" class="m_ms_new">NEW</i>
              {{ row.fname || row.job_name }}
            </div>
            <div class="Posted_card_pay">{{ browseLabel(row.is_browse) }}</div>
          </div>
          <div class="interview_time_box">
            <div class="card_interview_text">{{ $t('wap_user_00255') }}</div>
            <div class="card_interview_time">{{ row.intertime }}</div>
          </div>
          <div class="interview_job_box">
            <NuxtLink v-if="row.job_id" :to="`/jobs/${row.job_id}`" class="interview_box_post">
              <div class="card_interview_text">{{ $t('wap_user_00256') }}</div>
              <div class="card_interview_time">{{ row.job_name }}</div>
            </NuxtLink>
            <div class="interview_box_icons">
              <div v-if="row.is_browse !== 3 && row.is_browse !== 4" class="interview_box_icon" @click="accept(row.id)">{{ $t('wap_user_00262') }}</div>
              <div v-if="row.is_browse !== 3 && row.is_browse !== 4" class="interview_box_icon" @click="rejectId = row.id">{{ $t('wap_01053') }}</div>
              <div class="interview_box_icon" @click="remove(row.id)">
                <img src="/legacy/h5/images/resume_del.png" alt="" width="100%" height="100%" />
              </div>
            </div>
          </div>
          <p v-if="openId === row.id" class="interview_job_box">
            <div class="card_interview_text">{{ row.address }} {{ row.linkman }} {{ row.linktel }}</div>
            <div v-if="row.content" class="card_interview_time">{{ row.content }}</div>
          </p>
          <form v-if="rejectId === row.id" class="invite_no" @submit.prevent="reject(row.id)" @click.stop>
            <div class="invite_no_textarea">
              <textarea v-model="remark" :placeholder="$t('wap_01053')" />
            </div>
            <div class="invite_no_bth">
              <button type="submit">{{ $t('common.confirm') }}</button>
            </div>
          </form>
          <p v-if="row.is_browse === 3">
            <a href="javascript:;" @click="openReview(row.id)">{{ $t('common_01866') }}</a>
          </p>
        </div>
      </div>
    </div>
    <MemberPager :page="page" :page-size="pageSize" :total="total" @update:page="go" />
    <form v-if="reviewId" class="invite_no" @submit.prevent="submitReview">
      <p>{{ $t('common_01866') }}</p>
      <p v-if="reviewView?.submitted">{{ reviewView.total }}</p>
      <p>
        {{ $t('company_00008') }}
        <select v-model.number="reviewScores.desscore">
          <option v-for="n in 5" :key="'d' + n" :value="n">{{ n }}</option>
        </select>
      </p>
      <p>
        {{ $t('wap_com_00096') }}
        <select v-model.number="reviewScores.comscore">
          <option v-for="n in 5" :key="'c' + n" :value="n">{{ n }}</option>
        </select>
      </p>
      <p>
        {{ $t('wap_01073') }}
        <select v-model.number="reviewScores.hrscore">
          <option v-for="n in 5" :key="'h' + n" :value="n">{{ n }}</option>
        </select>
      </p>
      <textarea v-model="reviewComment" class="invite_notextarea" :placeholder="$t('common_01866')" />
      <div class="invite_nobth">
        <button type="submit" class="invite_nobth_bth">{{ $t('common.submit') }}</button>
        <button type="button" class="invite_nobth_bth" @click="reviewId = 0">{{ $t('common.close') }}</button>
      </div>
    </form>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
