<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('resume-outbox', () =>
  api.post('/v1/mcenter/resume-outbox/list', { page: 1, page_size: 20 }),
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
useSeoMeta({ title: t('member_user_00188') })
</script>

<template>
  <MemberPanel :title="$t('member_user_00188')" :error="error && !isUnauthErr(error) ? error : undefined">
    <p v-if="error && isUnauthErr(error)" class="muted">{{ $t('wap_00376') }}</p>
    <form class="form verification_form" @submit.prevent="send">
      <MemberField :label="$t('common.resume')">
        <select v-model.number="form.resume_id" required>
          <option :value="0">{{ $t('common.resume') }}</option>
          <option v-for="row in expectList" :key="row.id" :value="row.id">{{ row.name || row.id }}</option>
        </select>
      </MemberField>
      <MemberField :label="$t('member_user_00282')">
        <input v-model="form.email" :placeholder="$t('member_user_00282')" />
      </MemberField>
      <MemberField :label="$t('wap_com_00157')">
        <input v-model="form.com_name" />
      </MemberField>
      <MemberField :label="$t('wap_com_00288')">
        <input v-model="form.job_name" />
      </MemberField>
      <button type="submit" class="verification_form_btn">{{ $t('common.submit') }}</button>
    </form>
    <p v-if="msg">{{ msg }}</p>
    <div v-for="row in data?.list || []" :key="row.id" class="jobnotice_list site-pc">
      <div class="user_new_job">
        <span class="user_new_jobname">{{ row.com_name }} · {{ row.job_name }}</span>
        <div class="user_new_comname">{{ row.email }}</div>
      </div>
      <div class="user_new_time">{{ row.addtime_n }}</div>
      <div class="user_new_cz">
        <a href="javascript:;" class="user_new_yqh_sc" @click="remove(row.id)">{{ $t('common.delete') }}</a>
      </div>
    </div>
    <div class="site-h5 m_cardbox">
      <div class="m_cardbgbox">
        <MemberPostedCard
          v-for="row in data?.list || []"
          :key="'h5-' + row.id"
          :title="`${row.com_name} · ${row.job_name}`"
          :sub="row.email"
          :time="row.addtime_n"
        />
      </div>
    </div>
  </MemberPanel>
</template>
