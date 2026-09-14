<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { page, pageSize, inferTotal, go } = useMemberListPage()
const { data, error, refresh } = await useAsyncData(
  () => `my-reports-${page.value}`,
  () => api.post('/v1/mcenter/reports/list', { page: page.value, page_size: pageSize }),
)
const form = reactive({ target_kind: 1, target_id: 0, reason_code: 'spam', detail: '' })
const msg = ref('')
async function submit() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/reports', { ...form })
    msg.value = t('ui.submitted')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
useSeoMeta({ title: t('ui.my_reports') })
const total = computed(() => inferTotal(data.value))
</script>

<template>
  <MemberPanel :title="$t('ui.my_reports')" :error="error && !isUnauthErr(error) ? error : undefined">
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('wap_00376') : $t('ui.load_failed') }}</p>
    <form class="form verification_form" @submit.prevent="submit">
      <MemberField :label="$t('common.job')">
        <select v-model.number="form.target_kind">
          <option :value="1">{{ $t('common.job') }}</option>
          <option :value="2">{{ $t('common.company') }}</option>
          <option :value="3">{{ $t('common.resume') }}</option>
          <option :value="4">{{ $t('common.article') }}</option>
          <option :value="5">{{ $t('ui.user_kind') }}</option>
          <option :value="6">{{ $t('wap_00160') }}</option>
        </select>
      </MemberField>
      <MemberField :label="$t('ui.detail')" area>
        <textarea v-model="form.detail" rows="3" />
      </MemberField>
      <button type="submit" class="verification_form_btn">{{ $t('common.submit') }}</button>
    </form>
    <div class="job_list_tit">
      <ul>
        <li class="job_list_tit_cur"><a href="javascript:;">{{ $t('ui.my_reports') }}</a></li>
      </ul>
    </div>
    <p v-if="msg">{{ msg }}</p>
    <div v-for="row in data?.list || []" :key="row.id" class="sysynews_list site-pc">
      <div class="sysynews_span sysynews_name">#{{ row.target_id }}</div>
      <div class="sysynews_span sysynews_time">{{ row.status }}</div>
    </div>
    <MemberPager :page="page" :page-size="pageSize" :total="total" @update:page="go" />
  </MemberPanel>
</template>
