<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { page, pageSize, inferTotal, go } = useMemberListPage()
const { data, error, refresh } = await useAsyncData(
  () => `finder-list-${page.value}`,
  () => api.post('/v1/mcenter/finder/list', { page: page.value, page_size: pageSize }),
)
const form = reactive({
  name: '',
  keyword: '',
  cityid: 0,
  minsalary: '',
  maxsalary: '',
})
const msg = ref('')
const showForm = ref(false)
async function create() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/finder', { ...form })
    msg.value = t('common.success')
    form.name = ''
    form.keyword = ''
    showForm.value = false
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function remove(id: number) {
  await api.post('/v1/mcenter/finder/delete', { id })
  refresh()
}
const total = computed(() => inferTotal(data.value))
useSeoMeta({ title: t('member_user_00108') })
</script>

<template>
  <MemberPanel
    :title="$t('member_user_00108')"
    :error="error && !isUnauthErr(error) ? error : undefined"
    :empty="!error && !(data?.list || []).length && !showForm"
    :empty-text="$t('member_user_00492')"
  >
    <p v-if="error && isUnauthErr(error)" class="muted">{{ $t('wap_00376') }}</p>
    <div class="resume_Prompt_box">
      <div class="resume_Prompt">
        <i class="resume_Prompt_icon" />{{ $t('wap_user_00205') }}
        <a href="javascript:;" class="finder_btn" @click="showForm = true">{{ $t('member_user_00491') }}</a>
      </div>
    </div>
    <form v-if="showForm || !(data?.list || []).length" class="form verification_form" @submit.prevent="create">
      <MemberField :label="$t('wap_00529')"><input v-model="form.name" required /></MemberField>
      <MemberField :label="$t('common.job')"><input v-model="form.keyword" /></MemberField>
      <MemberField :label="$t('ui.min_salary')"><input v-model="form.minsalary" type="number" /></MemberField>
      <MemberField :label="$t('ui.max_salary')"><input v-model="form.maxsalary" type="number" /></MemberField>
      <button type="submit" class="verification_form_btn">{{ $t('common.submit') }}</button>
    </form>
    <p v-if="msg">{{ msg }}</p>
    <div v-for="row in data?.list || []" :key="row.id" class="job_search_box">
      <div class="job_search_box_left">
        <div class="job_search_box_jobmane">
          <NuxtLink v-if="row.search_to" :to="row.search_to" class="index_Job_Finder_cont_name_a">{{ row.name }}</NuxtLink>
          <span v-else>{{ row.name }}</span>
        </div>
        <div class="job_search_box_tj">{{ $t('member_com_00557') }}：{{ row.para_n || row.para }}</div>
      </div>
      <div class="job_search_box_right">
        <div class="job_search_box_bth">
          <NuxtLink v-if="row.search_to" :to="row.search_to" class="job_search_box_bth_a">
            <i class="job_search_box_bth_a_icon" />{{ $t('common.search') }}
          </NuxtLink>
        </div>
        <a href="javascript:;" class="cblue" @click="remove(row.id)">{{ $t('common.delete') }}</a>
      </div>
    </div>
    <MemberPager :page="page" :page-size="pageSize" :total="total" @update:page="go" />
  </MemberPanel>
</template>
