<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('finder-list', () =>
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
  await api.post('/v1/mcenter/finder/delete', { id })
  refresh()
}
useSeoMeta({ title: t('member_user_00108') })
</script>

<template>
  <MemberPanel :title="$t('member_user_00108')" :error="error && !isUnauthErr(error) ? error : undefined">
    <p v-if="error && isUnauthErr(error)" class="muted">{{ $t('wap_00376') }}</p>
    <form class="form verification_form" @submit.prevent="create">
      <MemberField :label="$t('wap_00529')">
        <input v-model="form.name" required />
      </MemberField>
      <MemberField :label="$t('common.job')">
        <input v-model="form.keyword" />
      </MemberField>
      <MemberField :label="$t('ui.min_salary')">
        <input v-model="form.minsalary" type="number" />
      </MemberField>
      <MemberField :label="$t('ui.max_salary')">
        <input v-model="form.maxsalary" type="number" />
      </MemberField>
      <button type="submit" class="verification_form_btn">{{ $t('common.submit') }}</button>
    </form>
    <p v-if="msg">{{ msg }}</p>
    <div v-for="row in data?.list || []" :key="row.id" class="jobnotice_list site-pc">
      <div class="user_new_job">
        <span class="user_new_jobname">{{ row.name }}</span>
        <div class="user_new_comname">{{ row.para_n || row.para }}</div>
      </div>
      <div class="user_new_cz">
        <NuxtLink v-if="row.search_to" :to="row.search_to" class="user_new_yqh_a">{{ $t('wap_com_00427') }}</NuxtLink>
        <a href="javascript:;" class="user_new_yqh_sc" @click="remove(row.id)">{{ $t('common.delete') }}</a>
      </div>
    </div>
    <div class="site-h5 m_cardbox">
      <div class="m_cardbgbox">
        <MemberPostedCard
          v-for="row in data?.list || []"
          :key="'h5-' + row.id"
          :title="row.name"
          :sub="row.para_n || row.para"
          :to="row.search_to || undefined"
        />
      </div>
    </div>
  </MemberPanel>
</template>
