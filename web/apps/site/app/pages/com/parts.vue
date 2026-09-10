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
type CatNode = { id: number; name: string; parent_id?: number }

const api = useApi()
const { t, locale } = useI18n()
const { data, error, refresh } = await useAsyncData('com-parts', () =>
  api.post('/v1/mcenter/com-parts/list', { page: 1, page_size: 20 }),
)
const { data: applies, refresh: refreshApplies } = await useAsyncData('com-part-applies', () =>
  api.post('/v1/mcenter/com-part-applications', { page: 1, page_size: 20 }),
)
const { data: partCats } = await useAsyncData(
  () => `com-part-cats-${locale.value}`,
  () => api.get<CatNode[]>('/v1/wap/categories', { kind: 'part' }).catch(() => [] as CatNode[]),
)
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
  <section>
    <h1>{{ $t('member_com_00480') }}</h1>
    <p v-if="error" class="muted">{{ isUnauthErr(error) ? $t('common_01153') : $t('ui.load_failed') }}</p>
    <form class="form" @submit.prevent="save">
      <input v-model="form.name" :placeholder="$t('wap_com_00288')" required />
      <select v-model.number="form.type">
        <option :value="0">{{ $t('wap_com_00311') }}</option>
        <option v-for="c in typeItems" :key="c.id" :value="c.id">{{ c.name }}</option>
      </select>
      <LocationFields
        v-model:province-id="form.provinceid"
        v-model:city-id="form.cityid"
        v-model:district-id="form.three_cityid"
      />
      <input v-model="form.address" :placeholder="$t('wap_00040')" />
      <input v-model.number="form.number" type="number" min="1" :placeholder="$t('ui.headcount')" />
      <select v-model.number="form.sex">
        <option :value="0">{{ $t('common.not_limited') }}</option>
        <option v-for="c in sexItems" :key="c.id" :value="c.id">{{ c.name }}</option>
      </select>
      <input v-model.number="form.salary" type="number" min="1" required />
      <select v-model.number="form.salary_type">
        <option :value="0">{{ $t('common.all') }}</option>
        <option v-for="c in salaryTypeItems" :key="c.id" :value="c.id">{{ c.name }}</option>
      </select>
      <select v-model.number="form.billing_cycle">
        <option :value="0">{{ $t('wap_user_00220') }}</option>
        <option v-for="c in cycleItems" :key="c.id" :value="c.id">{{ c.name }}</option>
      </select>
      <input v-model="form.linkman" :placeholder="$t('common_02051')" />
      <input v-model="form.linktel" :placeholder="$t('common.phone')" />
      <input v-model="form.x" placeholder="x" />
      <input v-model="form.y" placeholder="y" />
      <textarea v-model="form.content" rows="4" />
      <button type="submit">{{ form.id ? $t('common.save') : $t('member_com_00480') }}</button>
    </form>
    <h2>{{ $t('ui.published') }}</h2>
    <p v-if="!(data?.list || []).length" class="muted">{{ $t('ui.no_parts') }}</p>
    <div class="stack">
      <article v-for="row in data?.list || []" :key="row.id" class="job-card">
        <h3>{{ row.name || row.id }}</h3>
        <p class="muted">{{ partState(row) }}</p>
        <button type="button" @click="fill(row)">{{ $t('common.edit') }}</button>
        <button type="button" @click="refreshPart(row.id)">{{ $t('wap_user_00199') }}</button>
        <button v-if="row.status === 1" type="button" @click="setStatus(row.id, 0)">{{ $t('wap_com_00244') }}</button>
        <button v-else type="button" @click="setStatus(row.id, 1)">{{ $t('wap_com_00245') }}</button>
        <button type="button" @click="removePart(row.id)">{{ $t('common.delete') }}</button>
      </article>
    </div>
    <h2>{{ $t('ui.recv_applies') }}</h2>
    <p v-if="!(applies?.list || []).length" class="muted">{{ $t('ui.no_apply') }}</p>
    <div class="stack">
      <article v-for="row in applies?.list || []" :key="row.id" class="job-card">
        <h3>
          <NuxtLink :to="`/resumes/${row.uid}`">{{ row.uname || row.uid }}</NuxtLink>
          · {{ row.job_name || row.job_id }}
        </h3>
        <p class="muted">status {{ row.status }} {{ row.status_n }}</p>
        <button type="button" @click="setApply(row.id, 2)">{{ $t('wap_user_00258') }}</button>
        <button type="button" @click="setApply(row.id, 3)">{{ $t('wap_com_00046') }}</button>
      </article>
    </div>
    <p v-if="buyHint" class="muted">
      {{ buyHint }}
      <NuxtLink to="/com/added">{{ $t('wap_com_00048') }}</NuxtLink>
      ·
      <NuxtLink to="/com/pay">{{ $t('common_01946') }}</NuxtLink>
    </p>
    <p v-if="msg">{{ msg }}</p>
  </section>
</template>
