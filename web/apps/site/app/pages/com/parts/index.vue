<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'
import { ApiError } from '~/utils/envelope'

type PartRow = {
  id: number
  name?: string
  part_type_n?: string
  number?: number
  salary?: number
  salary_type_n?: string
  deadline_n?: string
  state?: number
  status?: number
}

const api = useApi()
const { t } = useI18n()
const route = useRoute()
const router = useRouter()
const { page, pageSize, inferTotal, go } = useMemberListPage()
const wRaw = Number(route.query.w)
const w = ref(Number.isFinite(wRaw) ? wRaw : 1)
const tab = ref(route.query.tab === 'apply' ? 'apply' : 'list')
const { data, error, refresh } = await useAsyncData(
  () => `com-parts-${tab.value}-${page.value}-${w.value}`,
  () =>
    tab.value === 'apply'
      ? Promise.resolve(null)
      : api.post<{
          list?: PartRow[]
          total?: number
          counts?: { w0?: number; w1?: number; w2?: number; w3?: number; w4?: number }
        }>('/v1/mcenter/com-parts/list', { page: page.value, page_size: pageSize, w: w.value }),
)
const { data: applies, refresh: refreshApplies } = await useAsyncData('com-part-applies', () =>
  api.post('/v1/mcenter/com-part-applications', { page: 1, page_size: 20 }),
)
const list = computed(() => (data.value?.list || []) as PartRow[])
const counts = computed(() => data.value?.counts || null)
const total = computed(() => inferTotal(data.value, list.value))
const msg = ref('')
const buyHint = ref('')
const picked = ref<number[]>([])
const allPicked = computed({
  get: () => list.value.length > 0 && picked.value.length === list.value.length,
  set: (v: boolean) => {
    picked.value = v ? list.value.map((j) => j.id) : []
  },
})
function togglePick(id: number) {
  if (picked.value.includes(id)) picked.value = picked.value.filter((x) => x !== id)
  else picked.value = [...picked.value, id]
}
function fail(e: unknown) {
  if (e instanceof ApiError && e.key === 'part_refresh_quota') {
    buyHint.value = e.message
    return t('wap_com_00048')
  }
  return e instanceof Error ? e.message : t('ui.failed')
}
function partState(row: PartRow) {
  if (row.state === 0) return t('wap_user_00006')
  if (row.state === 3) return t('wap_user_00167')
  if (row.status === 1) return t('wap_com_00242')
  if (row.state === 2) return t('wap_com_00245')
  return t('wap_com_00243')
}
function setW(n: number) {
  tab.value = 'list'
  w.value = n
  go(1)
  router.replace({ query: { ...route.query, w: String(n), tab: undefined } })
}
function setApplyTab() {
  tab.value = 'apply'
  router.replace({ query: { ...route.query, tab: 'apply' } })
}
async function refreshPart(id: number) {
  msg.value = ''
  buyHint.value = ''
  try {
    await api.post('/v1/mcenter/com-parts/refresh', { id })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}
async function setStatus(id: number, status: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/com-parts/status', { id, status })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}
async function batchStatus(status: number) {
  if (!picked.value.length) return
  msg.value = ''
  try {
    await api.post('/v1/mcenter/com-parts/batch/status', { ids: picked.value, status })
    picked.value = []
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}
async function setApply(id: number, status: number) {
  msg.value = ''
  try {
    await api.post('/v1/mcenter/com-part-applications/status', { id, status })
    msg.value = t('common.success')
    await refreshApplies()
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}
async function removePart(id: number) {
  if (!window.confirm(t('member_com_00083'))) return
  msg.value = ''
  try {
    await api.post('/v1/mcenter/com-parts', { ids: [id] })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}
async function batchDelete() {
  if (!picked.value.length) return
  if (!window.confirm(t('member_com_00083'))) return
  msg.value = ''
  try {
    await api.post('/v1/mcenter/com-parts', { ids: picked.value })
    picked.value = []
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}

const tabs = computed(() => [
  { value: 1, label: t('wap_com_00243'), on: tab.value === 'list' && w.value === 1, count: counts.value?.w1, select: () => setW(1) },
  { value: 0, label: t('wap_user_00006'), on: tab.value === 'list' && w.value === 0, count: counts.value?.w0, select: () => setW(0) },
  { value: 3, label: t('wap_user_00167'), on: tab.value === 'list' && w.value === 3, count: counts.value?.w3, select: () => setW(3) },
  { value: 4, label: t('wap_com_00242'), on: tab.value === 'list' && w.value === 4, count: counts.value?.w4, select: () => setW(4) },
  { value: 2, label: t('wap_com_00245'), on: tab.value === 'list' && w.value === 2, count: counts.value?.w2, select: () => setW(2) },
  { value: 'apply', label: t('ui.recv_applies'), on: tab.value === 'apply', select: () => setApplyTab() },
])

watch([page, w, tab], () => {
  picked.value = []
})

useSeoMeta({ title: t('wap_user_00271') })
</script>

<template>
  <MemberPanel :title="$t('wap_user_00271')" :error="error && !isUnauthErr(error) ? error : undefined">
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}</p>
    <MemberComScreen :tabs="tabs" add-to="/com/parts/new" :add-label="$t('member_com_00480')" />
    <template v-if="tab !== 'apply'">
      <p class="site-pc">
        <label><input v-model="allPicked" type="checkbox" /> {{ $t('wap_js_00074') }}</label>
        <a href="javascript:;" class="cblue" @click="batchStatus(0)">{{ $t('wap_com_00243') }}</a>
        <a href="javascript:;" class="cblue" @click="batchStatus(1)">{{ $t('wap_com_00242') }}</a>
        <a href="javascript:;" class="List_dete cblue" @click="batchDelete">{{ $t('common.delete') }}</a>
      </p>
      <table v-if="list.length" class="com_table site-pc">
        <tr>
          <th />
          <th>{{ $t('wap_com_00288') }}</th>
          <th>{{ $t('wap_com_00311') }}</th>
          <th>{{ $t('ui.headcount') }}</th>
          <th>{{ $t('member_user_00106') }}</th>
          <th>{{ $t('wap_com_00234') }}</th>
          <th>{{ $t('member_user_00181') }}</th>
          <th>{{ $t('member_user_00048') }}</th>
        </tr>
        <tr v-for="row in list" :key="row.id">
          <td><input type="checkbox" :checked="picked.includes(row.id)" @change="togglePick(row.id)" /></td>
          <td>{{ row.name || row.id }}</td>
          <td>{{ row.part_type_n }}</td>
          <td>{{ row.number }}</td>
          <td>{{ row.salary }} {{ row.salary_type_n }}</td>
          <td>{{ row.deadline_n }}</td>
          <td>{{ partState(row) }}</td>
          <td>
            <NuxtLink :to="`/com/parts/new?id=${row.id}`" class="cblue">{{ $t('common.edit') }}</NuxtLink>
            <a href="javascript:;" class="cblue" @click="refreshPart(row.id)">{{ $t('wap_user_00199') }}</a>
            <a v-if="row.status === 1" href="javascript:;" class="cblue" @click="setStatus(row.id, 0)">{{ $t('wap_com_00243') }}</a>
            <a v-else href="javascript:;" class="cblue" @click="setStatus(row.id, 1)">{{ $t('wap_com_00242') }}</a>
            <a href="javascript:;" class="List_dete cblue" @click="removePart(row.id)">{{ $t('common.delete') }}</a>
          </td>
        </tr>
      </table>
      <div class="site-h5 more_position_body">
        <div v-for="row in list" :key="'h5-' + row.id" class="position_body_card">
          <div class="position_body_card_top">
            <span class="body_card_top_name">{{ row.name || row.id }}</span>
          </div>
          <div class="position_body_card_center">
            <span>{{ partState(row) }} · {{ row.salary }} {{ row.salary_type_n }}</span>
          </div>
          <div class="position_body_card_bom">
            <ul>
              <li @click="refreshPart(row.id)">
                <div class="body_card_bom_name">{{ $t('wap_user_00199') }}</div>
              </li>
              <li>
                <NuxtLink :to="`/com/parts/new?id=${row.id}`">
                  <div class="body_card_bom_name">{{ $t('common.edit') }}</div>
                </NuxtLink>
              </li>
              <li @click="setStatus(row.id, row.status === 1 ? 0 : 1)">
                <div class="body_card_bom_name">{{ row.status === 1 ? $t('wap_com_00243') : $t('wap_com_00242') }}</div>
              </li>
              <li @click="removePart(row.id)">
                <div class="body_card_bom_name">{{ $t('common.delete') }}</div>
              </li>
            </ul>
          </div>
        </div>
      </div>
      <MemberPager :page="page" :page-size="pageSize" :total="total" @update:page="go" />
    </template>
    <MemberHrResumeRows
      v-else
      show-job
      :rows="(applies?.list || []).map((row: Record<string, unknown>) => ({
        key: Number(row.id),
        name: String(row.uname || row.uid || ''),
        job: String(row.job_name || row.job_id || ''),
        time: String(row.status_n || row.status || ''),
        to: `/resumes/${row.uid}`,
      }))"
    >
      <template #pc-acts="{ row }">
        <a href="javascript:;" class="cblue" @click="setApply(Number(row.key), 2)">{{ $t('wap_user_00258') }}</a>
        <a href="javascript:;" class="cblue" @click="setApply(Number(row.key), 3)">{{ $t('wap_com_00046') }}</a>
      </template>
      <template #h5-acts="{ row }">
        <div class="hr_userlist_czicon" @click="setApply(Number(row.key), 2)">{{ $t('wap_user_00258') }}</div>
        <div class="hr_userlist_czicon" @click="setApply(Number(row.key), 3)">{{ $t('wap_com_00046') }}</div>
      </template>
    </MemberHrResumeRows>
    <p v-if="buyHint" class="muted">
      {{ buyHint }}
      <NuxtLink to="/com/member-right">{{ $t('wap_com_00097') }}</NuxtLink>
    </p>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
