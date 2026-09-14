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
    <form class="form verification_form" @submit.prevent="create">
      <MemberField :label="$t('wap_00529')"><input v-model="form.name" required /></MemberField>
      <MemberField :label="$t('common.resume')"><input v-model="form.keyword" /></MemberField>
      <MemberField :label="$t('common_02110')"><input v-model.number="form.cityid" type="number" /></MemberField>
      <MemberField :label="$t('ui.min_salary')"><input v-model="form.minsalary" type="number" /></MemberField>
      <MemberField :label="$t('ui.max_salary')"><input v-model="form.maxsalary" type="number" /></MemberField>
      <button type="submit" class="verification_form_btn">{{ $t('member_com_00556') }}</button>
    </form>
    <p v-if="msg">{{ msg }}</p>
    <div v-for="row in data?.list || []" :key="row.id" class="job_search_box">
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
  </MemberPanel>
</template>
