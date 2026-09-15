<script setup lang="ts">
import { isUnauthErr } from '~/utils/site'
import { ApiError } from '~/utils/envelope'

type PartRow = {
  id: number
  name?: string
  type?: number
  province_id?: number
  city_id?: number
  three_city_id?: number
  address?: string
  salary?: number
  salary_type?: number
  billing_cycle?: number
  linkman?: string
  linktel?: string
  state?: number
  status?: number
  content?: string
  number?: number
  sex?: number
}

const api = useApi()
const { t } = useI18n()
const { data, error, refresh } = await useAsyncData('com-parts', () =>
  api.post('/v1/mcenter/com-parts/list', { page: 1, page_size: 20 }),
)
const { data: applies, refresh: refreshApplies } = await useAsyncData('com-part-applies', () =>
  api.post('/v1/mcenter/com-part-applications', { page: 1, page_size: 20 }),
)
const { data: partCats } = await usePartCats()
const roots = computed(() =>
  (partCats.value || []).filter((c) => !c.parent_id).sort((a, b) => a.id - b.id),
)
function childrenOf(rootIdx: number) {
  const root = roots.value[rootIdx]
  if (!root) return []
  return (partCats.value || []).filter((c) => Number(c.parent_id) === root.id)
}
const typeItems = computed(() => childrenOf(0))
const salaryTypeItems = computed(() => childrenOf(1))
const cycleItems = computed(() => childrenOf(2))
const sexItems = computed(() => childrenOf(3))
const form = reactive({
  id: 0,
  name: '',
  type: 0,
  provinceid: 0,
  cityid: 0,
  three_cityid: 0,
  address: '',
  salary: 0,
  salary_type: 0,
  billing_cycle: 0,
  number: 1,
  sex: 0,
  linkman: '',
  linktel: '',
  content: '',
  x: '',
  y: '',
})
const msg = ref('')
const buyHint = ref('')
function fail(e: unknown) {
  if (e instanceof ApiError && e.key === 'part_refresh_quota') {
    buyHint.value = e.message
    return t('wap_com_00048')
  }
  return e instanceof Error ? e.message : t('ui.failed')
}
function fill(row: PartRow) {
  form.id = row.id
  form.name = String(row.name || '')
  form.type = Number(row.type || 0)
  form.provinceid = Number(row.province_id || 0)
  form.cityid = Number(row.city_id || 0)
  form.three_cityid = Number(row.three_city_id || 0)
  form.address = String(row.address || '')
  form.salary = Number(row.salary || 0)
  form.salary_type = Number(row.salary_type || 0)
  form.billing_cycle = Number(row.billing_cycle || 0)
  form.number = Number(row.number || 1)
  form.sex = Number(row.sex || 0)
  form.linkman = String(row.linkman || '')
  form.linktel = String(row.linktel || '')
  form.content = String(row.content || '')
}
async function save() {
  msg.value = ''
  buyHint.value = ''
  if (!form.name.trim() || form.salary <= 0) {
    msg.value = t('ui.failed')
    return
  }
  try {
    if (form.id) await api.post('/v1/mcenter/com-parts/update', { ...form })
    else await api.post('/v1/mcenter/com-parts/create', { ...form })
    msg.value = t('common.success')
    form.id = 0
    form.name = ''
    await refresh()
  } catch (e: unknown) {
    msg.value = fail(e)
  }
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
  msg.value = ''
  try {
    await api.post('/v1/mcenter/com-parts', { ids: [id] })
    msg.value = t('common.success')
    await refresh()
  } catch (e: unknown) {
    msg.value = fail(e)
  }
}
function partState(row: PartRow) {
  if (row.state === 0) return t('wap_user_00006')
  if (row.state === 3) return t('wap_user_00167')
  if (row.status === 1) return t('wap_com_00242')
  return t('wap_com_00243')
}
useSeoMeta({ title: t('member_com_00480') })
</script>

<template>
  <MemberPanel :title="$t('member_com_00480')" :error="error && !isUnauthErr(error) ? error : undefined">
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}</p>
    <form class="com_release_box" @submit.prevent="save">
      <ul>
        <MemberReleaseRow :label="$t('wap_com_00288')" required><input v-model="form.name" required class="com_release_textnew_text" /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_com_00311')">
          <select v-model.number="form.type">
            <option :value="0">{{ $t('wap_com_00311') }}</option>
            <option v-for="c in typeItems" :key="c.id" :value="c.id">{{ c.name }}</option>
          </select>
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_user_00243')">
          <LocationFields
            v-model:province-id="form.provinceid"
            v-model:city-id="form.cityid"
            v-model:district-id="form.three_cityid"
          />
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_00040')"><input v-model="form.address" class="com_release_textnew_text" /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('ui.headcount')"><input v-model.number="form.number" type="number" min="1" /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_com_00303')">
          <select v-model.number="form.sex">
            <option :value="0">{{ $t('common.not_limited') }}</option>
            <option v-for="c in sexItems" :key="c.id" :value="c.id">{{ c.name }}</option>
          </select>
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_00925')" required><input v-model.number="form.salary" type="number" min="1" required /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_00925')">
          <select v-model.number="form.salary_type">
            <option :value="0">{{ $t('common.all') }}</option>
            <option v-for="c in salaryTypeItems" :key="c.id" :value="c.id">{{ c.name }}</option>
          </select>
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_user_00220')">
          <select v-model.number="form.billing_cycle">
            <option :value="0">{{ $t('wap_user_00220') }}</option>
            <option v-for="c in cycleItems" :key="c.id" :value="c.id">{{ c.name }}</option>
          </select>
        </MemberReleaseRow>
        <MemberReleaseRow :label="$t('common_02051')"><input v-model="form.linkman" class="com_release_textnew_text" /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('common.phone')"><input v-model="form.linktel" class="com_release_textnew_text" /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('ui.job_desc')" area><textarea v-model="form.content" rows="4" /></MemberReleaseRow>
        <MemberReleaseRow :label="$t('wap_user_00243')">
          <MapPick v-model:x="form.x" v-model:y="form.y" />
        </MemberReleaseRow>
      </ul>
      <button type="submit" class="btn_01">{{ form.id ? $t('common.save') : $t('member_com_00480') }}</button>
    </form>
    <MemberResumeH1 :title="$t('ui.published')" />
    <table v-if="(data?.list || []).length" class="com_table site-pc">
      <tr>
        <th>{{ $t('wap_com_00288') }}</th>
        <th>{{ $t('member_user_00181') }}</th>
        <th>{{ $t('member_user_00048') }}</th>
      </tr>
      <tr v-for="row in data?.list || []" :key="row.id">
        <td>{{ row.name || row.id }}</td>
        <td>{{ partState(row) }}</td>
        <td>
          <a href="javascript:;" class="cblue" @click="fill(row)">{{ $t('common.edit') }}</a>
          <a href="javascript:;" class="cblue" @click="refreshPart(row.id)">{{ $t('wap_user_00199') }}</a>
          <a href="javascript:;" class="List_dete cblue" @click="removePart(row.id)">{{ $t('common.delete') }}</a>
        </td>
      </tr>
    </table>
    <div class="site-h5 more_position_body">
      <div v-for="row in data?.list || []" :key="'h5-' + row.id" class="position_body_card">
        <div class="position_body_card_top">
          <span class="body_card_top_name">{{ row.name || row.id }}</span>
        </div>
        <div class="position_body_card_bom">
          <span>{{ partState(row) }}</span>
        </div>
      </div>
    </div>
    <MemberResumeH1 :title="$t('ui.recv_applies')" />
    <MemberHrResumeRows
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
    </MemberHrResumeRows>
    <p v-if="buyHint" class="muted">
      {{ buyHint }}
      <NuxtLink to="/com/added">{{ $t('wap_com_00048') }}</NuxtLink>
      ·
      <NuxtLink to="/com/pay">{{ $t('common_01946') }}</NuxtLink>
    </p>
    <p v-if="msg">{{ msg }}</p>
  </MemberPanel>
</template>
