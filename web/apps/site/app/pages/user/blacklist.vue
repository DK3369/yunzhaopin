<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'

const api = useApi()
const { t } = useI18n()
const { page, pageSize, inferTotal, go } = useMemberListPage()
const { data, error, refresh } = await useAsyncData(
  () => `blacklist-${page.value}`,
  () => api.post('/v1/mcenter/blacklist/list', { page: page.value, page_size: pageSize }),
)
const keyword = ref('')
const hits = ref<Array<{ uid: number; name?: string }>>([])
const msg = ref('')
async function search() {
  msg.value = ''
  try {
    const r = await api.get<{ list: Array<{ uid: number; name?: string }> }>('/v1/wap/companies', {
      keyword: keyword.value,
      page: 1,
      page_size: 10,
    })
    hits.value = r.list || []
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function add(uid: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/blacklist', { blocked_uid: uid })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
async function remove(blockedUid: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/blacklist/remove', { uid: blockedUid })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = e instanceof Error ? e.message : t('ui.failed')
  }
}
const total = computed(() => inferTotal(data.value))
useSeoMeta({ title: t('member_user_00044') })
</script>

<template>
  <MemberPanel :title="$t('member_user_00044')" :error="error && !isUnauthErr(error) ? error : undefined">
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('wap_00376') : $t('ui.load_failed') }}</p>
    <div class="black_sumit_box">
      <input v-model="keyword" class="black_text" :placeholder="$t('member_user_00563')" />
      <input type="button" class="black_sumit" :value="$t('common.search')" @click="search" />
    </div>
    <div class="blacklist">
      <ul class="clearfix" id="company_blench">
        <li v-for="row in hits" :key="'h-' + row.uid">
          <a href="javascript:;" @click="add(row.uid)">{{ row.name || row.uid }}</a>
        </li>
        <li v-for="row in data?.list || []" :key="row.id">
          <a href="javascript:;" class="close" @click="remove(row.blocked_uid)">x</a>
          <NuxtLink :to="`/companies/${row.blocked_uid}`">{{ row.com_name || row.reason || row.blocked_uid }}</NuxtLink>
        </li>
      </ul>
    </div>
    <MemberPager :page="page" :page-size="pageSize" :total="total" @update:page="go" />
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
