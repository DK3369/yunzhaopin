<script setup lang="ts">
const props = defineProps<{
  seekerUid: number
  jobId?: number
}>()
const emit = defineEmits<{ done: []; cancel: [] }>()
const api = useApi()
const { t } = useI18n()
const invite = reactive({
  seeker_uid: props.seekerUid,
  job_id: Number(props.jobId || 0),
  content: '',
  address: '',
  intertime: '',
  linkman: '',
  linktel: '',
  save_yqmb: false,
})
watch(
  () => props.seekerUid,
  (n) => {
    invite.seeker_uid = n
  },
)
watch(
  () => props.jobId,
  (n) => {
    if (n) invite.job_id = Number(n)
  },
)
const { data: myJobs } = await useAsyncData('com-yqms-jobs', () =>
  api
    .post<{ list: Array<{ id: number; name: string }> }>('/v1/mcenter/jobs/list', {
      page: 1,
      page_size: 50,
      w: 1,
    })
    .catch(() => ({ list: [] as Array<{ id: number; name: string }> })),
)
watch(
  myJobs,
  (v) => {
    if (!invite.job_id) invite.job_id = Number(v?.list?.[0]?.id || 0)
  },
  { immediate: true },
)
const msg = ref('')
async function send(confirm = false) {
  msg.value = ''
  if (!invite.seeker_uid || !invite.job_id) {
    msg.value = t('common_01153')
    return
  }
  if (!invite.intertime.trim()) {
    msg.value = t('member_com_00681')
    return
  }
  try {
    const res = await api.post<{ status: number; jifen?: number; price?: number }>(
      '/v1/mcenter/company/yqms/create',
      { ...invite, confirm },
    )
    if (res.status === 2) {
      const text = res.jifen
        ? `${t('common_00697')}${res.jifen}${t('common_01935')}?`
        : res.price
          ? `${t('common_00696')}${res.price}${t('common_00757')}?`
          : t('common_00696')
      if (window.confirm(text)) await send(true)
      return
    }
    msg.value = t('wap_00291')
    emit('done')
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
</script>

<template>
  <form class="com_release_box site-pc" @submit.prevent="send()">
    <ul>
      <MemberReleaseRow :label="$t('wap_00190')">
        <select v-model.number="invite.job_id">
          <option v-for="j in myJobs?.list || []" :key="j.id" :value="j.id">{{ j.name }}</option>
        </select>
      </MemberReleaseRow>
      <MemberReleaseRow :label="$t('wap_00040')" required>
        <input v-model="invite.intertime" type="datetime-local" required />
      </MemberReleaseRow>
      <MemberReleaseRow :label="$t('wap_user_00243')" required>
        <input v-model="invite.address" required />
      </MemberReleaseRow>
      <MemberReleaseRow :label="$t('common_02051')">
        <input v-model="invite.linkman" />
      </MemberReleaseRow>
      <MemberReleaseRow :label="$t('common.phone')" required>
        <input v-model="invite.linktel" required />
      </MemberReleaseRow>
      <MemberReleaseRow :label="$t('wap_user_00102')" area>
        <textarea v-model="invite.content" rows="3" />
      </MemberReleaseRow>
      <MemberReleaseRow :label="$t('member_com_00512')">
        <input v-model="invite.save_yqmb" type="checkbox" />
      </MemberReleaseRow>
    </ul>
    <button type="submit" class="btn_01">{{ $t('common.submit') }}</button>
    <button type="button" class="btn_01" @click="emit('cancel')">{{ $t('common.cancel') }}</button>
    <p v-if="msg">{{ msg }}</p>
  </form>
  <div class="site-h5 issue_post_body">
    <form class="yun_createbox" @submit.prevent="send()">
      <MemberField wap :label="$t('wap_00190')">
        <select v-model.number="invite.job_id">
          <option v-for="j in myJobs?.list || []" :key="'h5j-' + j.id" :value="j.id">{{ j.name }}</option>
        </select>
      </MemberField>
      <MemberField wap :label="$t('wap_00040')">
        <input v-model="invite.intertime" type="datetime-local" required />
      </MemberField>
      <MemberField wap :label="$t('wap_user_00243')">
        <input v-model="invite.address" required />
      </MemberField>
      <MemberField wap :label="$t('common_02051')">
        <input v-model="invite.linkman" />
      </MemberField>
      <MemberField wap :label="$t('common.phone')">
        <input v-model="invite.linktel" required />
      </MemberField>
      <MemberField wap area :label="$t('wap_user_00102')">
        <textarea v-model="invite.content" rows="3" />
      </MemberField>
      <MemberField wap :label="$t('member_com_00512')">
        <input v-model="invite.save_yqmb" type="checkbox" />
      </MemberField>
      <button type="submit" class="issue_post_body_btn">{{ $t('common.submit') }}</button>
      <button type="button" class="issue_post_body_btn" @click="emit('cancel')">{{ $t('common.cancel') }}</button>
      <p v-if="msg">{{ msg }}</p>
    </form>
  </div>
</template>
