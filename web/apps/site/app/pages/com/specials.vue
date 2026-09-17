<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

type Row = { id: number; sid: number; title?: string; status: number; datetime_n?: string }

const api = useApi()
const { t } = useI18n()
const { page, pageSize, inferTotal, go } = useMemberListPage()
const { data, error, refresh } = await useAsyncData(
  () => `com-specials-mine-${page.value}`,
  () => api.post<{ list: Row[]; total: number }>('/v1/mcenter/specials/mine', { page: page.value, page_size: pageSize }),
)
const msg = ref('')
const list = computed(() => data.value?.list || [])

function statusText(status: number) {
  if (status === 1) return t('wap_user_00165')
  if (status === 2) return t('wap_user_00167')
  return t('wap_user_00166')
}

async function remove(id: number) {
  if (!window.confirm(t('member_com_00083'))) return
  msg.value = ''
  try {
    await api.post('/v1/mcenter/specials/delete', { id })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}

useSeoMeta({ title: t('wap_com_00310') })
</script>

<template>
  <MemberPanel
    :title="$t('wap_com_00310')"
    :error="error && !isUnauthErr(error) ? error : undefined"
    :empty="!error && !list.length"
    empty-to="/specials"
    :empty-action="$t('member_com_00670')"
  >
    <p v-if="error && isUnauthErr(error)" class="muted">{{ $t('common_01153') }}</p>
    <table v-if="list.length" class="com_table site-pc">
      <tr>
        <th>{{ $t('member_com_00343') }}</th>
        <th>{{ $t('wap_js_00088') }}</th>
        <th>{{ $t('member_user_00181') }}</th>
        <th>{{ $t('member_user_00048') }}</th>
      </tr>
      <tr v-for="row in list" :key="row.id">
        <td>
          <NuxtLink :to="`/specials/${row.sid}`" class="cblue">{{ row.title || row.sid }}</NuxtLink>
        </td>
        <td>{{ row.datetime_n }}</td>
        <td>{{ statusText(row.status) }}</td>
        <td>
          <a href="javascript:;" class="com_bth cblue" @click="remove(row.id)">{{ $t('common.delete') }}</a>
        </td>
      </tr>
    </table>
    <div class="site-h5">
      <div v-for="row in list" :key="'h5-' + row.id" class="com_cardlist" @click="navigateTo(`/specials/${row.sid}`)">
        <div class="com_cardlist_tit">
          <span class="job_list_jobname com_member_hr_cblue">{{ row.title || row.sid }}</span>
        </div>
        <div class="com_cardlist_p">
          <span class="com_cardlist_p_name">{{ $t('wap_com_00406') }}</span>
          <span :class="{ wap_member_wtg: row.status === 2 }">{{ statusText(row.status) }}</span>
        </div>
        <div class="com_cardlist_p">
          <span class="com_cardlist_p_name">{{ $t('wap_user_00304') }}</span>
          {{ row.datetime_n }}
        </div>
        <div class="com_card_cz">
          <span class="com_card_delete" @click.stop="remove(row.id)" />
        </div>
      </div>
    </div>
    <p v-if="msg">{{ msg }}</p>
    <MemberPager :page="page" :page-size="pageSize" :total="inferTotal(data, list)" @update:page="go" />
  </MemberPanel>
</template>
