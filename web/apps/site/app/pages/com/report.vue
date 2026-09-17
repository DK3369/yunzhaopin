<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

type Row = {
  id: number
  r_name?: string
  username?: string
  r_reason?: string
  result?: string | null
  inputtime_n?: string
}

const api = useApi()
const { t } = useI18n()
const { page, pageSize, inferTotal, go } = useMemberListPage()
const { data, error, refresh } = await useAsyncData(
  () => `com-crm-reports-${page.value}`,
  () => api.post('/v1/mcenter/crm-reports/list', { page: page.value, page_size: pageSize }),
)
const list = computed(() => (data.value?.list || []) as Row[])
const reason = ref('')
const msg = ref('')

async function submit() {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/crm-reports', { reason: reason.value.trim() })
    reason.value = ''
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.load_failed')
  }
}

async function remove(id: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/crm-reports/delete', { ids: [id] })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.load_failed')
  }
}

useSeoMeta({ title: t('member_com_00148') })
</script>

<template>
  <MemberPanel :title="$t('member_com_00148')" :error="error && !isUnauthErr(error) ? error : undefined">
    <p v-if="error && isUnauthErr(error)" class="muted">{{ $t('common_01153') }}</p>
    <form class="com_release_box site-pc" @submit.prevent="submit">
      <ul>
        <MemberReleaseRow :label="$t('member_com_00331')" area><textarea v-model="reason" rows="3" maxlength="200" /></MemberReleaseRow>
      </ul>
      <button type="submit" class="btn_01">{{ $t('common.submit') }}</button>
    </form>
    <div class="site-h5 issue_post_body">
      <form class="yun_createbox" @submit.prevent="submit">
        <MemberField wap area :label="$t('member_com_00331')"><textarea v-model="reason" rows="3" maxlength="200" /></MemberField>
        <button type="submit" class="issue_post_body_btn">{{ $t('common.submit') }}</button>
      </form>
    </div>
    <table v-if="list.length" class="com_table site-pc">
      <tr>
        <th>{{ $t('member_com_00148') }}</th>
        <th>{{ $t('member_user_00106') }}</th>
        <th>{{ $t('member_user_00048') }}</th>
      </tr>
      <tr v-for="row in list" :key="row.id">
        <td>{{ row.r_name }} · {{ row.r_reason }}</td>
        <td>{{ row.inputtime_n }} · {{ row.result || $t('admin_user_00371') }}</td>
        <td>
          <a href="javascript:;" class="com_bth cblue" @click="remove(row.id)">{{ $t('common.delete') }}</a>
        </td>
      </tr>
    </table>
    <div class="site-h5">
      <div v-for="row in list" :key="'h5-' + row.id" class="com_cardlist">
        <div class="com_cardlist_tit">{{ row.r_name }} · {{ row.r_reason }}</div>
        <div class="com_cardlist_p">
          <span class="com_cardlist_p_name">{{ $t('member_user_00106') }}</span>
          {{ row.inputtime_n }} · {{ row.result || $t('admin_user_00371') }}
        </div>
        <div class="com_card_cz">
          <span class="com_card_delete" @click="remove(row.id)" />
        </div>
      </div>
    </div>
    <p v-if="msg">{{ msg }}</p>
    <MemberPager :page="page" :page-size="pageSize" :total="inferTotal(data, list)" @update:page="go" />
  </MemberPanel>
</template>
