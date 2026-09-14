<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { page, pageSize, inferTotal, go } = useMemberListPage()
const { data, error, refresh } = await useAsyncData(
  () => `resume-outbox-${page.value}`,
  () => api.post('/v1/mcenter/resume-outbox/list', { page: page.value, page_size: pageSize }),
)
const { data: expects } = await useAsyncData('outbox-expects', () =>
  api.post('/v1/mcenter/resume/expects/list', {}).catch(() => []),
)
const expectList = computed(() => {
  const raw = expects.value
  if (Array.isArray(raw)) return raw
  return raw?.list || []
})
const form = reactive({
  resume_id: 0,
  email: '',
  com_name: '',
  job_name: '',
  resume_name: '',
})
watch(
  expectList,
  (list) => {
    if (!form.resume_id && list[0]?.id) form.resume_id = Number(list[0].id)
  },
  { immediate: true },
)
const msg = ref('')
async function send() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/resume-outbox', { ...form })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function remove(id: number) {
  if (!window.confirm(t('member_com_00083'))) return
  msg.value = ''
  try {
    await api.post('/v1/mcenter/resume-outbox/delete', { ids: [id] })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
const total = computed(() => inferTotal(data.value))
useSeoMeta({ title: t('member_user_00188') })
</script>

<template>
  <MemberPanel :title="$t('member_user_00188')" :error="error && !isUnauthErr(error) ? error : undefined">
    <p v-if="error && isUnauthErr(error)" class="muted">{{ $t('wap_00376') }}</p>
    <form class="resume_fk_box" @submit.prevent="send">
      <div class="yun_send_resume_list">
        <div class="yun_send_resume_list_name"><span class="yun_send_resume_list_x">*</span>{{ $t('common.resume') }}</div>
        <div class="yun_send_resume_list_right">
          <select v-model.number="form.resume_id" required>
            <option :value="0">{{ $t('common.resume') }}</option>
            <option v-for="row in expectList" :key="row.id" :value="row.id">{{ row.name || row.id }}</option>
          </select>
        </div>
      </div>
      <div class="yun_send_resume_list">
        <div class="yun_send_resume_list_name"><span class="yun_send_resume_list_x">*</span>{{ $t('member_user_00282') }}</div>
        <div class="yun_send_resume_list_right">
          <input v-model="form.email" class="yun_send_resume_txt" :placeholder="$t('member_user_00282')" />
        </div>
      </div>
      <div class="yun_send_resume_list">
        <div class="yun_send_resume_list_name"><span class="yun_send_resume_list_x">*</span>{{ $t('wap_com_00157') }}</div>
        <div class="yun_send_resume_list_right">
          <input v-model="form.com_name" class="yun_send_resume_txt" />
        </div>
      </div>
      <div class="yun_send_resume_list">
        <div class="yun_send_resume_list_name"><span class="yun_send_resume_list_x">*</span>{{ $t('wap_com_00288') }}</div>
        <div class="yun_send_resume_list_right">
          <input v-model="form.job_name" class="yun_send_resume_txt" />
        </div>
      </div>
      <div class="yun_send_resume_list">
        <div class="yun_send_resume_list_name">&nbsp;</div>
        <div class="yun_send_resume_list_right">
          <button type="submit" class="verification_form_btn">{{ $t('common.submit') }}</button>
        </div>
      </div>
    </form>
    <p v-if="msg">{{ msg }}</p>
    <div v-if="(data?.list || []).length" class="resumeout_tit mt40 site-pc">
      <div class="resumeout_span resumeout_comname">{{ $t('wap_com_00157') }}</div>
      <div class="resumeout_span resumeout_jobname">{{ $t('wap_com_00288') }}</div>
      <div class="resumeout_span resumeout_emil">{{ $t('member_user_00282') }}</div>
      <div class="resumeout_span resumeout_send">{{ $t('member_user_00280') }}</div>
      <div class="resumeout_span List_Title_w80">{{ $t('member_user_00048') }}</div>
    </div>
    <div v-for="row in data?.list || []" :key="row.id" class="resumeout_listbox site-pc">
      <div class="resumeout_span resumeout_comname">{{ row.com_name }}</div>
      <div class="resumeout_span resumeout_jobname">{{ row.job_name }}</div>
      <div class="resumeout_span resumeout_emil">{{ row.email }}</div>
      <div class="resumeout_span resumeout_send">{{ row.addtime_n }}</div>
      <div class="resumeout_span List_Title_w80">
        <a href="javascript:;" class="List_dete cblue" @click="remove(row.id)">{{ $t('common.delete') }}</a>
      </div>
    </div>
    <div class="site-h5 m_cardbox">
      <div class="m_cardbgbox">
        <MemberPostedCard
          v-for="row in data?.list || []"
          :key="'h5-' + row.id"
          variant="issue"
          :title="`${row.com_name} · ${row.job_name}`"
          :sub="row.email"
          :time="row.addtime_n"
        />
      </div>
    </div>
    <MemberPager :page="page" :page-size="pageSize" :total="total" @update:page="go" />
  </MemberPanel>
</template>
