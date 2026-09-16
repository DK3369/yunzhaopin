<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('com-finder-list', () =>
  api.post('/v1/mcenter/finder/list', { page: 1, page_size: 20 }),
)
const form = reactive({
  name: '',
  keyword: '',
  cityid: 0,
  minsalary: '',
  maxsalary: '',
})
const msg = ref('')
async function create() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/finder', { ...form })
    msg.value = t('common.success')
    form.name = ''
    form.keyword = ''
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function remove(id: number) {
  if (!window.confirm(t('member_com_00083'))) return
  msg.value = ''
  try {
    await api.post('/v1/mcenter/finder/delete', { id })
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('member_com_00086') })
</script>

<template>
  <MemberPanel :title="$t('member_com_00086')" :error="error && !isUnauthErr(error) ? error : undefined">
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}</p>
    <form class="com_release_box site-pc" @submit.prevent="create">
      <ul>
        <MemberReleaseRow :label="$t('wap_00529')" required><input v-model="form.name" required class="com_release_textnew_text" /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('common.resume')"><input v-model="form.keyword" class="com_release_textnew_text" /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('common_02110')"><input v-model.number="form.cityid" type="number" /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('ui.min_salary')"><input v-model="form.minsalary" type="number" /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('ui.max_salary')"><input v-model="form.maxsalary" type="number" /></MemberReleaseRow>
      </ul>
      <button type="submit" class="btn_01">{{ $t('member_com_00556') }}</button>
    </form>
    <div class="site-h5 issue_post_body">
      <form class="yun_createbox" @submit.prevent="create">
        <MemberField wap :label="$t('wap_00529')"><input v-model="form.name" required /></MemberField>
        <MemberField wap :label="$t('common.resume')"><input v-model="form.keyword" /></MemberField>
        <MemberField wap :label="$t('common_02110')"><input v-model.number="form.cityid" type="number" /></MemberField>
        <MemberField wap :label="$t('ui.min_salary')"><input v-model="form.minsalary" type="number" /></MemberField>
        <MemberField wap :label="$t('ui.max_salary')"><input v-model="form.maxsalary" type="number" /></MemberField>
        <button type="submit" class="issue_post_body_btn">{{ $t('member_com_00556') }}</button>
      </form>
    </div>
    <p v-if="msg">{{ msg }}</p>
    <div v-for="row in data?.list || []" :key="row.id" class="job_search_box site-pc">
      <div class="job_search_box_left">
        <div class="job_search_box_name">{{ row.name }}</div>
        <div class="job_search_box_p">{{ row.para_n || row.para }}</div>
      </div>
      <div class="job_search_box_right">
        <div class="job_search_box_bth">
          <NuxtLink v-if="row.search_to" :to="row.search_to" class="job_search_box_bth_a">{{ $t('common.search') }}</NuxtLink>
        </div>
        <a href="javascript:;" class="cblue" @click="remove(row.id)">{{ $t('common.delete') }}</a>
      </div>
    </div>
    <div class="site-h5">
      <div v-for="row in data?.list || []" :key="'h5-' + row.id" class="com_cardlist">
        <div class="com_cardlist_tit">{{ row.name }}</div>
        <div class="com_cardlist_p">{{ row.para_n || row.para }}</div>
        <div class="interview_card_bom">
          <div class="card_bom_icon">
            <NuxtLink v-if="row.search_to" :to="row.search_to">{{ $t('common.search') }}</NuxtLink>
          </div>
          <div class="card_bom_icon">
            <a href="javascript:;" @click="remove(row.id)">{{ $t('common.delete') }}</a>
          </div>
        </div>
      </div>
    </div>
  </MemberPanel>
</template>
